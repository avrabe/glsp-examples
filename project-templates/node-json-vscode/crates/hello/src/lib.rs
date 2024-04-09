mod bindings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{cell::RefCell, io::Read};
use url::Url;

use bindings::exports::component::{
    self,
    tasklist::tasklist::{Guest, GuestTasklistModel, TasklistModel},
};

impl Guest for TasklistModel {
    type TasklistModel = TaskListComponent;
}
pub struct TaskListComponent {
    calls: RefCell<TaskList>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    id: String,
    tasks: Option<Vec<Task>>,
    transitions: Option<Vec<Transition>>,
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

impl GuestTasklistModel for TaskListComponent {
    fn id(&self) -> String {
        self.calls.borrow().id.clone()
    }

    fn tasks(&self) -> Vec<component::tasklist::tasklist::Task> {
        todo!()
    }

    fn transitions(&self) -> Vec<component::tasklist::tasklist::Transition> {
        todo!()
    }

    fn add_task(
        &self,
        position: component::tasklist::tasklist::Position,
    ) -> component::tasklist::tasklist::Task {
        todo!()
    }

    fn remove_task(&self, task_id: String) {
        todo!()
    }

    fn resize_task(&self, task_id: String, size: component::tasklist::tasklist::Size) {
        todo!()
    }

    fn move_task(&self, task_id: String, position: component::tasklist::tasklist::Position) {
        todo!()
    }

    fn add_transition(
        &self,
        source_task_id: String,
        target_task_id: String,
    ) -> component::tasklist::tasklist::Transition {
        todo!()
    }

    fn remove_transition(&self, transition_id: String) {
        todo!()
    }

    fn save_to_file(&self, filename: String) {
        let _ = filename;
        todo!()
    }

    fn load_from_file(&self, filename: String) {
        let path = Url::parse(&filename).unwrap().to_file_path().unwrap();
        let mut file = fs::File::open(path).unwrap();
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer);
        let deserializer = &mut serde_json::Deserializer::from_str(&buffer);
        let result: Result<TaskList, _> = serde_path_to_error::deserialize(deserializer);
        match result {
            Ok(task_list) => {
                self.calls.replace(task_list);
            }
            Err(err) => {
                println!("Error deserializing task list, return new empty: {:?}", err);
                self.calls.replace(TaskList {
                    id: "12344".to_string(),
                    tasks: None,
                    transitions: None,
                });
            }
        }
    }

    fn create_model_for_empty_file() -> TasklistModel {
        let tl = TaskListComponent {
            calls: RefCell::new(TaskList {
                id: "12344".to_string(),
                tasks: None,
                transitions: None,
            }),
        };
        component::tasklist::tasklist::TasklistModel::new(tl)
    }
}

bindings::export!(TasklistModel with_types_in bindings);
