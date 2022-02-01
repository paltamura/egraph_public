use crate::functor::Functor;
use crate::types::{Domain, Point};
use crate::types_helper::*;
use std::fmt::Debug;

pub trait ILayer: Debug + Sync + Send {
    fn get_name(&self) -> &String;
    fn get_instructions_domain(&self) -> &Domain;
    fn get_states_domain(&self) -> &Domain;
    fn forward(&mut self, instructions: Point);
    fn back(&mut self) -> Point;
    fn get_functors_mut_ref(&mut self) -> Vec<&mut Functor>;
    fn to_file(&mut self, file_name: String);
    fn from_file(&mut self, file_name: String);
}

pub fn get_states_doms(layers: &Vec<Box<dyn ILayer>>) -> Vec<Domain> {
    let mut vec: Vec<Domain> = vec![];
    for layer in layers {
        vec.push(layer.get_states_domain().clone());
    }
    vec
}

pub fn get_instructions_doms(layers: &Vec<Box<dyn ILayer>>) -> Vec<Domain> {
    let mut vec: Vec<Domain> = vec![];
    for layer in layers {
        vec.push(layer.get_instructions_domain().clone());
    }
    vec
}

#[derive(Debug)]
pub struct DimensionalReductionLayer {
    name: String,
    instructions_domain: Domain,
    states_domain: Domain,
    subordinates: Vec<Box<dyn ILayer>>,
    instructions_functor: Functor,
    states_functor: Functor,
    last_known_instructions: Point,
    last_known_states: Point,
    last_known_sub_states: Vec<Point>,
}

impl ILayer for DimensionalReductionLayer {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_functors_mut_ref(&mut self) -> Vec<&mut Functor> {
        let mut ret = vec![];
        ret.push(&mut self.instructions_functor);
        ret.push(&mut self.states_functor);
        for sub in &mut self.subordinates {
            ret.extend(sub.get_functors_mut_ref());
        }
        ret
    }

    fn get_instructions_domain(&self) -> &Domain {
        &self.instructions_domain
    }

    fn get_states_domain(&self) -> &Domain {
        &self.states_domain
    }

    fn forward(&mut self, instructions: Point) {
        self.last_known_instructions = instructions.clone();
        let mut points = self.last_known_sub_states.clone();
        points.push(self.last_known_instructions.clone());
        let joined_points = join_points(points);
        let out_point = self.instructions_functor.evaluate(joined_points);
        let mut sub_instructions_size = vec![];
        for sub in self.subordinates.iter_mut() {
            sub_instructions_size.push(sub.get_instructions_domain().1.len());
        }
        let points = split_points(out_point, sub_instructions_size);
        let mut i: usize = 0;
        for sub in self.subordinates.iter_mut() {
            sub.forward(points.get(i).unwrap().clone());
            i = i + 1;
        }
    }

    fn back(&mut self) -> Point {
        let mut sub_state_points = vec![];
        for sub in self.subordinates.iter_mut() {
            sub_state_points.push(sub.back().clone());
        }
        self.last_known_sub_states = sub_state_points.clone();
        let joined_points = join_points(sub_state_points);
        let out_point = self.states_functor.evaluate(joined_points);
        self.last_known_states = out_point.clone();
        out_point
    }

    fn to_file(&mut self, file_name: String) {
        let base = format!("{}_{}", file_name, self.name);
        self.instructions_functor
            .to_file(format!("{}_{}", &base, "_instructions_functor"));
        self.states_functor
            .to_file(format!("{}_{}", &base, "_states_functor"));
        for sub in &mut self.subordinates {
            sub.to_file(base.clone());
        }
    }

    fn from_file(&mut self, file_name: String) {
        let base = format!("{}_{}", file_name, self.name);
        self.instructions_functor
            .from_file(format!("{}_{}", &base, "_instructions_functor"));
        self.states_functor
            .from_file(format!("{}_{}", &base, "_states_functor"));
        for sub in &mut self.subordinates {
            sub.from_file(base.clone());
        }
    }
}

#[derive(Debug)]
pub struct BrokerLayerMock {
    name: String,
    instructions_domain: Domain,
    states_domain: Domain,
}

impl ILayer for BrokerLayerMock {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_functors_mut_ref(&mut self) -> Vec<&mut Functor> {
        vec![]
    }
    fn get_instructions_domain(&self) -> &Domain {
        &self.instructions_domain
    }
    fn get_states_domain(&self) -> &Domain {
        &self.states_domain
    }
    fn forward(&mut self, instructions: Point) {
        println!("Broker mock forward() {:?}", instructions);
    }
    fn back(&mut self) -> Point {
        println!("Broker mock back()");
        self.states_domain.get_empty_point()
    }

    fn to_file(&mut self, _file_name: String) {}

    fn from_file(&mut self, _file_name: String) {}
}

pub fn dr_layer(
    name: String,
    instructions_dom: Domain,
    states_dom: Domain,
    subordinates: Vec<Box<dyn ILayer>>,
) -> Box<dyn ILayer> {
    let mut subs_states_doms = get_states_doms(&subordinates);
    subs_states_doms.push(instructions_dom.clone());
    let iddr_in_dom: Domain = join_doms(subs_states_doms);
    let subs_instructions_doms = get_instructions_doms(&subordinates);
    let iddr_out_dom: Domain = join_doms(subs_instructions_doms);
    let sddr_in_dom: Domain = join_doms(get_states_doms(&subordinates));
    let sddr_out_dom: Domain = states_dom.clone();
    let instructions_functor: Functor = Functor::new(
        format!("{}_{}_{}", name, "inst", "functor"),
        iddr_in_dom,
        iddr_out_dom,
    );
    let states_functor: Functor = Functor::new(
        format!("{}_{}_{}", name, "stat", "functor"),
        sddr_in_dom,
        sddr_out_dom,
    );
    let layer: DimensionalReductionLayer = DimensionalReductionLayer {
        name,
        instructions_domain: instructions_dom.clone(),
        states_domain: states_dom.clone(),
        subordinates: subordinates,
        instructions_functor,
        states_functor,
        last_known_instructions: instructions_dom.get_empty_point(),
        last_known_states: states_dom.get_empty_point(),
        last_known_sub_states: vec![],
    };
    Box::new(layer)
}

pub fn specific_broker_layer(
    name: String,
    instructions_dom: Domain,
    states_dom: Domain,
) -> Box<dyn ILayer> {
    let layer: BrokerLayerMock = BrokerLayerMock {
        name,
        instructions_domain: instructions_dom,
        states_domain: states_dom,
    };
    Box::new(layer)
}
