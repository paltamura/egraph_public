use std::{
    ops::AddAssign,
    sync::{Arc, Mutex},
    thread,
};

use crate::{layers::ILayer, model_service, types::Point};

pub struct Service {
    pub next_id: Arc<Mutex<u32>>,
    pub orders: Arc<Mutex<Vec<Arc<Mutex<Order>>>>>,
}

#[derive(Debug)]
pub struct Order {
    id: Mutex<String>,
    last_instruction: Mutex<Point>,
    last_state: Mutex<Point>,
    log: Mutex<Vec<(String, Point)>>,
    model: Mutex<Box<dyn ILayer>>,
}

impl Service {
    pub fn create_order(&mut self, model_name: String) -> String {
        println!("Ejecuto create_order con modelo: {}", model_name);
        let id = format!("OID-{:?}", self.next_id.lock().unwrap().clone());
        let model = model_service::get_by_name(model_name);
        let new_order = Order {
            id: Mutex::new(id.clone()),
            last_instruction: Mutex::new(Point(vec![])),
            last_state: Mutex::new(Point(vec![])),
            log: Mutex::new(vec![]),
            model: Mutex::new(model),
        };
        println!("order {:?}", &new_order);
        self.orders
            .lock()
            .unwrap()
            .push(Arc::new(Mutex::new(new_order)));
        self.next_id.lock().unwrap().add_assign(1);
        println!("id {:?}", &id);
        id
    }

    pub fn run_order(&mut self, id: String) -> String {
        let mut _arcorder = self.get_order_by_id(id.clone()).clone();
        thread::spawn(move || {
            let order = _arcorder.lock().unwrap();
            order
                .last_state
                .lock()
                .unwrap()
                .clone_from(&order.model.lock().unwrap().back());
            order
                .log
                .lock()
                .unwrap()
                .push(("STATE RECEIVED".to_string(), order.last_state.lock().unwrap().clone()));
            order
                .last_instruction
                .lock()
                .unwrap()
                .clone_from(&Point(vec!["open_position".to_string()]));
            order
                .log
                .lock()
                .unwrap()
                .push(("INSTRUCTION SEND".to_string(), order.last_instruction.lock().unwrap().clone()));
            order
                .model
                .lock()
                .unwrap()
                .forward(order.last_instruction.lock().unwrap().clone());
        });
        format!("executing order {}...", id)
    }

    fn get_order_by_id(&mut self, id: String) -> Arc<Mutex<Order>> {
        let a = self.orders.lock().unwrap();
        let o = a
            .iter()
            .filter(|i| i.lock().unwrap().id.lock().unwrap().eq(&id))
            .next()
            .unwrap();
        return o.clone();
    }

    pub fn set_order_instruction(&mut self, id: String, instruction: Point) {
        let order = self.get_order_by_id(id);
        order
            .lock()
            .unwrap()
            .last_instruction
            .lock()
            .unwrap()
            .clone_from(&instruction.clone());
        order
            .lock()
            .unwrap()
            .log
            .lock()
            .unwrap()
            .push(("INSTRUCTION SET".to_string(), instruction.clone()));
    }

    pub fn get_order_state(&mut self, id: String) -> Point {
        let order = self.get_order_by_id(id);
        let x = order.lock().unwrap().last_state.lock().unwrap().clone();
        x
    }

    pub fn get_order_log(&mut self, id: String) -> Vec<(String, Point)> {
        let order = self.get_order_by_id(id);
        let x = order.lock().unwrap().log.lock().unwrap().clone();
        x
    }
}