use crate::types::Domain;
use crate::types::Point;
use crate::types_helper::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Functor {
    pub name: String,
    pub in_dom: Domain,
    pub out_dom: Domain,
    pub map: HashMap<String, Morphism>,
}

impl Functor {
    pub fn new(name:String, in_dom: Domain, out_dom: Domain) -> Self {
        let cases = get_cases(xploit(in_dom.clone()), out_dom.clone());
        Self {
            name,
            in_dom,
            out_dom,
            map: cases,
        }
    }

    pub fn evaluate(&self, point: Point) -> Point {
        let sig = point.get_point_signature();
        let morphism = self.map.get(&sig);
        let res = morphism.unwrap();
        res.out_point.clone()
    }

    pub fn from_file(&mut self, file: String)
    {
        let data_readed = fs::read_to_string(&file).expect(&format!("Unable to read file {:?}", &file));
        let deserialized = serde_json::from_str::<Functor>(&data_readed).expect("Error durante deserialización");
        self.clone_from(&deserialized);
    }
    pub fn to_file(&self, file: String)
    {
        let json = serde_json::to_string(&self).expect("Error serializando functor");
        fs::write(&file, &json).expect("Unable to write file");
    }

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Morphism {
    pub case_id: String,
    pub in_point: Point,
    pub out_point: Point,
    pub comment: String,
    pub pass_mark: bool,
}

pub fn get_cases(input_points: Vec<Point>, out_dom: Domain) -> HashMap<String, Morphism> {
    let mut map: HashMap<String, Morphism> = HashMap::new();
    let mut i: u32 = 0;
    for input_point in input_points {
        map.insert(
            input_point.get_point_signature().to_string(),
            Morphism {
                case_id: format!("{}", i),
                in_point: input_point,
                out_point: out_dom.get_empty_point(),
                comment: "".to_string(),
                pass_mark: false,
            },
        );
        i = i + 1;
    }
    map
}
