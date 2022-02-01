use crate::{
    functor::{Functor, Morphism},
    layers::ILayer,
    models,
};

pub fn build_and_save(template_name: String, model_name: String) {
    let model: Box<dyn ILayer> = models::build_model_by_name(template_name);
    save(model, model_name);
}

pub fn get_functors(model_name: String) -> Vec<Functor> {
    let mut model = get_by_name(model_name);
    let functors = model.get_functors_mut_ref();

    let mut ret = vec![];
    for fun in functors {
        ret.push(fun.to_owned());
    }
    ret
}

pub fn get_functors_names(model_name: String) -> Vec<String> {
    let mut model = get_by_name(model_name);
    let functors = model.get_functors_mut_ref();
    let mut functors_names = vec![];
    for functor in functors {
        functors_names.push(functor.name.clone());
    }
    functors_names
}

pub fn get_all_morphisms(model_name: String, functor_name: String) -> Vec<Morphism> {
    let functors = get_functors(model_name);
    for functor in functors {
        if functor.name.eq(&functor_name) {
            let fun = functor.clone();
            let mut ret = vec![];
            for item in fun.map {
                ret.push(item.1);
            }
            ret.sort_by(|a, b| a.case_id.parse::<i32>().unwrap().cmp(&b.case_id.parse::<i32>().unwrap()));
            return ret
        }
    }
    vec![]
}

pub fn get_pending_morphisms(model_name: String, functor_name: String) -> Vec<Morphism> {
    let all_morphisms = get_all_morphisms(model_name, functor_name);
    let mut ret = vec![];
    for morphism in all_morphisms {
        if !morphism.pass_mark {
            ret.push(morphism);
        }
    }
    ret.sort_by(|a, b| a.case_id.parse::<i32>().unwrap().cmp(&b.case_id.parse::<i32>().unwrap()));
    ret
}

pub fn get_morphism_by_id(model_name: String, functor_name: String, case_id: String) -> Morphism {
    let all_morphisms = get_all_morphisms(model_name, functor_name);
    let mut ret = vec![];
    for morphism in all_morphisms {
        if morphism.case_id.eq(&case_id) {
            ret.push(morphism);
        }
    }
    if ret.len() > 1 {
        panic!();
    }
    ret.get(0).unwrap().clone()
}

pub fn set_morphism_by_id(
    model_name: String,
    functor_name: String,
    morphism_id: String,
    updated_morphism: Morphism,
) {
    let mut model = get_by_name(model_name.clone());
    let functors_mut_ref = model.get_functors_mut_ref();
    for functor in functors_mut_ref {
        if functor.name.eq(&functor_name) {
            for item in &mut functor.map {
                if item.1.case_id.eq(&morphism_id) {
                    item.1.clone_from(&updated_morphism);
                }
            }
        }
    }
    save(model, model_name);
}

//save
pub fn save(mut model: Box<dyn ILayer>, file_name: String) {
    let path = "./model_builds/".to_string();
    let file = file_name.to_string();
    let full_path = format!("{}{}", &path, &file);
    model.to_file(full_path);
}

//get_all
pub fn _get_all() -> Vec<Box<dyn ILayer>> {
    todo!()
}

//get_by_name
pub fn get_by_name(file_name: String) -> Box<dyn ILayer> {
    let path = "./model_builds/".to_string();
    let file = file_name.to_string();
    let full_path = format!("{}{}", &path, &file);
    let mut model: Box<dyn ILayer> = models::build_model_by_name("model1".to_string());
    model.from_file(full_path);
    model
}

//remove
pub fn _remove(_file_name: String) {
    todo!()
}
