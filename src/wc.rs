use crate::{functor::{Morphism, Functor}, types::Point};

//Conf
static IP: &str = "localhost";
static PORT: &str = "8000";

// Modelado
pub fn build_and_save(modeltype: String, modelname: String) -> String {
    let url_ = format!(
        "http://{}:{}/buildandsave/{}/{}",
        IP, PORT, modeltype, modelname
    );
    let res: String = reqwest::blocking::get(url_)
        .expect("msg")
        .text()
        .expect("msg");
    res
}

pub fn get_functors(modelname: String) -> Vec<Functor> {
    let url_ = format!("http://{}:{}/getfunctors/{}", IP, PORT, modelname);
    let res: Vec<Functor> = reqwest::blocking::get(url_)
        .expect("msg")
        .json()
        .expect("msg");
    res
}

pub fn get_all_morphisms(modelname: String, functorname: String) -> Vec<Morphism> {
    let url_ = format!(
        "http://{}:{}/getallmorphisms/{}/{}",
        IP, PORT, modelname, functorname
    );
    let res: Vec<Morphism> = reqwest::blocking::get(url_)
        .expect("msg")
        .json()
        .expect("msg");
    res
}

pub fn get_morphism(modelname: String, functorname: String, morphismid: String) -> Morphism {
    let url_ = format!(
        "http://{}:{}/getmorphism/{}/{}/{}",
        IP, PORT, modelname, functorname, morphismid
    );
    let res: Morphism = reqwest::blocking::get(url_)
        .expect("msg")
        .json()
        .expect("msg");
    res
}

pub fn set_morphism(
    modelname: String,
    functorname: String,
    morphismid: String,
    morphism: Morphism,
) -> String {
    let url_ = format!(
        "http://{}:{}/setmorphism/{}/{}/{}",
        IP, PORT, modelname, functorname, morphismid
    );
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(url_)
        .json(&morphism)
        .send();
    let r = res.expect("msg").text().expect("msg");
    r
}

// Ejecución
pub fn create_order(modelname: String) -> String {
    let url_ = format!("http://{}:{}/createorder/{}", IP, PORT, modelname);
    let res = reqwest::blocking::get(url_)
        .expect("msg")
        .text()
        .expect("msg");
    res
}

pub fn execute_order(orderid: String) -> String {
    let url_ = format!("http://{}:{}/executeorder/{}", IP, PORT, orderid);
    let res = reqwest::blocking::get(url_)
        .expect("msg")
        .text()
        .expect("msg");
    res
}

pub fn get_order_log(orderid: String) -> Vec<(String, Point)> {
    let url_ = format!("http://{}:{}/getorderlog/{}", IP, PORT, orderid);
    let res: Vec<(String, Point)> = reqwest::blocking::get(url_)
        .expect("msg")
        .json()
        .expect("msg");
    res
}
