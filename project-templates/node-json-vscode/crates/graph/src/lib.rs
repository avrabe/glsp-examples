cargo_component_bindings::generate!();

use bindings::Guest;
//use uuid::Uuid;
use crate::bindings::exports::component::graph::tasklist;
use crate::bindings::exports::component::graph::tasklist::{Task, Position};
struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }

    fn validate_task_name(name: String) -> bool {
        name.len() > 0
    }
}

impl tasklist::Guest for Component {
    fn tasks() -> Vec<Task> {
        vec![]
    }

    fn add_task(position: Position) -> Task {
        let id = "ddd".to_string();
        Task {
            id,
            name: "task".to_string(),
            position,
            size: None,
        }
    }
}



