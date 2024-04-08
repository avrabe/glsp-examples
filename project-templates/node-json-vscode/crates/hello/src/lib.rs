mod bindings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{cell::RefCell, io::Read};
use url::Url;

use bindings::exports::component::{
    self,
    hello::hello::{Guest, GuestHello, Hello},
};

impl Guest for Hello {
    type Hello = HelloComponent;
}
pub struct HelloComponent {
    calls: RefCell<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    id: String,
    tasks: Vec<Task>,
    transitions: Vec<Transition>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    name: String,
    position: Position,
    size: Size,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Size {
    width: f64,
    height: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transition {
    id: String,
    #[serde(rename = "sourceTaskId")]
    source_task_id: String,
    #[serde(rename = "targetTaskId")]
    target_task_id: String,
}

impl GuestHello for HelloComponent {
    fn calls(&self) -> String {
        self.calls.replace_with(|&mut old| old + 1);
        format!("hello {} times", self.calls.borrow())
    }

    fn execute(&self, filename: String) -> String {
        let path = Url::parse(&filename).unwrap().to_file_path().unwrap();
        let mut file = fs::File::open(path).unwrap();
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer);
        let deserializer = &mut serde_json::Deserializer::from_str(&buffer);
        let result: Result<TaskList, _> = serde_path_to_error::deserialize(deserializer);
        result.unwrap().id
    }

    fn create_world() -> component::hello::hello::Hello {
        let hello = HelloComponent {
            calls: RefCell::new(0),
        };
        component::hello::hello::Hello::new(hello)
    }
}

bindings::export!(Hello with_types_in bindings);
