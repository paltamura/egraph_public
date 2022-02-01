use crate::{layers::ILayer, model_service, models, types::Point};

#[test]
fn model_build() {
    _model_build();
}

#[test]
fn model_save() {
    _model_save(_model_build());
}

#[test]
fn model_read() {
    _model_read();
}

#[test]
fn model_back_forward() {
    _model_back_forward(_model_read());
}

fn _model_build() -> Box<dyn ILayer> {
    let model: Box<dyn ILayer> = models::model_1::build();
    model
}

fn _model_save(model: Box<dyn ILayer>) {
    model_service::save(model, "model1".to_string());
}

fn _model_read() -> Box<dyn ILayer> {
    model_service::get_by_name("model1".to_string())
}

fn _model_back_forward(mut model: Box<dyn ILayer>) {
    let states = model.back();
    println!("Estados > {:?}", states);
    let instructions = Point(vec!["open_position".to_string()]);
    model.forward(instructions.clone());
    println!("Instrucciones > {:?}", instructions.clone());
}

#[test]
fn get_functors() {
    let functors = model_service::get_functors_names("model1".to_string());
    for f in functors {
        println!("{:?}", f);
    }
}

#[test]
fn model_service_set_morphism_by_id() {
    let mut morphism_to_edit = model_service::get_morphism_by_id(
        "model1".to_string(),
        "input_layer_inst_functor".to_string(),
        "0".to_string(),
    );
    morphism_to_edit.comment = "Editado".to_string();
    morphism_to_edit.pass_mark = true;
    model_service::set_morphism_by_id(
        "model1".to_string(),
        "input_layer_inst_functor".to_string(),
        "0".to_string(),
        morphism_to_edit,
    )
}
