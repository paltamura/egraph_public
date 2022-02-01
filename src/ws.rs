use crate::{execution_service::*, functor::Morphism, model_service};
use serde_json;
use std::sync::{Arc, Mutex};
static mut V: Vec<Service> = vec![];

/////////////////////////////////////////////////
//Web server
/////////////////////////////////////////////////
pub fn run() {
    unsafe {
        V.push(Service {
            next_id: Arc::new(Mutex::new(0)),
            orders: Arc::new(Mutex::new(vec![])),
        });
    }
    let r = rocket::ignite()
        //Model service
        .mount("/buildandsave", routes![build_and_save])
        .mount("/getfunctors", routes![get_functors])
        .mount("/getallmorphisms", routes![get_all_morphisms])
        .mount("/getmorphism", routes![get_morphism_by_id])
        .mount("/setmorphism", routes![set_morphism_by_id])
        //Execution service
        .mount("/createorder", routes![create_order])
        .mount("/executeorder", routes![execute_order])
        .mount("/getorderlog", routes![get_order_log]);
    r.launch();
}

pub(crate) fn stop() {
    todo!()
}

/////////////////////////////////////////////////
//Model service
/////////////////////////////////////////////////

#[get("/<templatename>/<modelname>")]
fn build_and_save(templatename: String, modelname: String) -> String {
    model_service::build_and_save(templatename, modelname);
    "ok".to_string()
}

#[get("/<modelname>")]
fn get_functors(modelname: String) -> String {
    let res = model_service::get_functors(modelname);
    serde_json::to_string(&res).unwrap()
}

#[get("/<modelname>/<functorname>")]
fn get_all_morphisms(modelname: String, functorname: String) -> String {
    let res = model_service::get_all_morphisms(modelname, functorname);
    serde_json::to_string(&res).unwrap()
}

#[get("/<modelname>/<functorname>/<morphismid>")]
fn get_morphism_by_id(modelname: String, functorname: String, morphismid: String) -> String {
    let res = model_service::get_morphism_by_id(modelname, functorname, morphismid);
    serde_json::to_string(&res).unwrap()
}

#[post(
    "/<modelname>/<functorname>/<morphismid>",
    format = "application/json",
    data = "<json>"
)]
fn set_morphism_by_id(
    modelname: String,
    functorname: String,
    morphismid: String,
    json: String,
) -> String {
    let inst: Morphism = serde_json::from_str(&json).unwrap();
    model_service::set_morphism_by_id(modelname, functorname, morphismid, inst);
    "ok".to_string()
}

/////////////////////////////////////////////////
//Execution service
/////////////////////////////////////////////////
#[get("/<modelname>")]
fn create_order(modelname: String) -> String {
    unsafe { V[0].create_order(modelname) }
}

#[get("/<orderid>")]
fn execute_order(orderid: String) -> String {
    unsafe { V[0].run_order(orderid) }
}

#[get("/<orderid>")]
fn get_order_log(orderid: String) -> String {
    unsafe {
        let ret = V[0].get_order_log(orderid);
        serde_json::to_string(&ret).unwrap()
    }
}

////////////////////////////////////////////////////////////////////////
// Model service
// build_and_save(template_name: String, model_name: String) <<<<<<<< Done Done Done 
// get_functors(model_name: String) -> Vec<Functor>
// get_functors_names(model_name: String) -> Vec<String> <<<<<<<< Done Done Done
// get_all_morphisms(model_name: String, functor_name: String) -> Vec<Morphism> <<<<<<<< Done Done Done
// get_pending_morphisms(model_name: String, functor_name: String) -> Vec<Morphism>
// >>>> get_morphism_by_id(model_name: String, functor_name: String, case_id: String) -> Morphism <<<<<<<< Done Done Done
// >>>> set_morphism_by_id(model_name: String,functor_name: String,morphism_id: String,updated_morphism: Morphism) <<<<<<<< Done Done Done
// save(mut model: Box<dyn ILayer>, file_name: String)
// _get_all() -> Vec<Box<dyn ILayer>>
// get_by_name(file_name: String) -> Box<dyn ILayer>
// _remove(_file_name: String)

// Execution service
// create_order(&mut self, model_name: String) -> String <<<<<<<< Done Done Done
// run_order(&mut self, id: String) -> String <<<<<<<< Done Done Done
// get_order_by_id(&mut self, id: String) -> Arc<Mutex<Order>>
// set_order_instruction(&mut self, id: String, instruction: Point)
// get_order_state(&mut self, id: String) -> Point
// get_order_log(&mut self, id: String) -> Vec<Point> <<<<<<<< Done Done Done
