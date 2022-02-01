use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Domain(pub String, pub Vec<Dimension>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dimension(pub String, pub Vec<String>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Point(pub Vec<String>);

impl Point {
    pub fn get_point_signature(&self) -> String {
        serde_json::to_string(self).expect("Error intentando generar signature")
    }

    // pub fn get_point_signature2(&self) -> String {
    //     let mut s = "".to_string();
    //     for v in self.0.clone() {
    //         s = format!("{}|{:<1}", s, v);
    //     }
    //     s
    // }
}

impl Domain {
    pub(crate) fn get_empty_point(&self) -> Point {
        let dimoutlen = self.1.len();
        let out_point_model = vec!["empty".to_string(); dimoutlen];
        let out_point = Point(out_point_model);
        out_point
    }
}
