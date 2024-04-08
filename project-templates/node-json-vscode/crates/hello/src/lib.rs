mod bindings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::{cell::RefCell, io::Read};
use url::Url;
use uuid::Uuid;

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
struct Size {
    width: f32,
    height: f32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transition {
    id: String,
    #[serde(rename = "sourceTaskId")]
    source_task_id: String,
    #[serde(rename = "targetTaskId")]
    target_task_id: String,
}
impl Default for TaskList {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tasks: Some(vec![]),
            transitions: Some(vec![]),
        }
    }
}

impl TaskListComponent {
    fn default() -> Self {
        Self {
            calls: RefCell::new(TaskList::default()),
        }
    }
}

impl GuestTasklistModel for TaskListComponent {
    fn id(&self) -> String {
        self.calls.borrow().id.clone()
    }

    fn tasks(&self) -> Vec<component::tasklist::tasklist::Task> {
        let tl = self.calls.borrow_mut();

        let foo = tl
            .tasks
            .as_deref()
            .unwrap_or_default()
            .into_iter()
            .map(|task| component::tasklist::tasklist::Task {
                id: task.id.clone(),
                name: task.name.clone(),
                position: component::tasklist::tasklist::Position {
                    x: task.position.x,
                    y: task.position.y,
                },
                size: Some(component::tasklist::tasklist::Size {
                    width: task.size.width,
                    height: task.size.height,
                }),
            });
        let b = foo.collect();
        b
    }

    fn transitions(&self) -> Vec<component::tasklist::tasklist::Transition> {
        let tl = self.calls.borrow_mut();
        let foo = tl
            .transitions
            .as_deref()
            .unwrap_or_default()
            .into_iter()
            .map(|transition| component::tasklist::tasklist::Transition {
                id: transition.id.clone(),
                source_task_id: transition.source_task_id.clone(),
                target_task_id: transition.target_task_id.clone(),
            });
        let b = foo.collect();
        b
    }

    fn add_task(
        &self,
        position: component::tasklist::tasklist::Position,
    ) -> component::tasklist::tasklist::Task {
        let mut tl = self.calls.borrow_mut();
        let id = Uuid::new_v4().to_string();
        let label = format!("New Task {}", tl.tasks.as_ref().unwrap().len());
        if let Some(tasks) = &mut tl.tasks {
            let task = Task {
                id: id.clone(),
                name: label.clone(),
                position: Position {
                    x: position.x,
                    y: position.y,
                },
                size: Size {
                    width: 100.0,
                    height: 100.0,
                },
            };
            tasks.push(task);
        }

        let task = component::tasklist::tasklist::Task {
            id: id,
            name: label,
            position,
            size: None,
        };
        task
    }

    fn remove_task(&self, task_id: String) {
        let mut tl = self.calls.borrow_mut();
        if let Some(tasks) = &mut tl.tasks {
            tasks.retain(|task| task.id != task_id);
        }
    }

    fn resize_task(&self, task_id: String, size: component::tasklist::tasklist::Size) {
        let mut tl = self.calls.borrow_mut();
        if let Some(tasks) = &mut tl.tasks {
            for task in tasks {
                if task.id == task_id {
                    task.size = Size {
                        width: size.width,
                        height: size.height,
                    };
                    break;
                }
            }
        }
    }

    fn move_task(&self, task_id: String, position: component::tasklist::tasklist::Position) {
        let mut tl = self.calls.borrow_mut();
        if let Some(tasks) = &mut tl.tasks {
            for task in tasks {
                if task.id == task_id {
                    task.position = Position {
                        x: position.x,
                        y: position.y,
                    };
                    break;
                }
            }
        }
    }

    fn set_task_name(&self, task_id: String, label: String) {
        let mut tl = self.calls.borrow_mut();
        if let Some(tasks) = &mut tl.tasks {
            for task in tasks {
                if task.id == task_id {
                    task.name = label;
                    break;
                }
            }
        }
    }

    fn add_transition(
        &self,
        source_task_id: String,
        target_task_id: String,
    ) -> component::tasklist::tasklist::Transition {
        let id = Uuid::new_v4().to_string();

        let mut tl = self.calls.borrow_mut();
        if let Some(transitions) = &mut tl.transitions {
            let transition = Transition {
                id: id.clone(),
                source_task_id: source_task_id.clone(),
                target_task_id: target_task_id.clone(),
            };
            transitions.push(transition);
        }
        let transition = component::tasklist::tasklist::Transition {
            id,
            source_task_id,
            target_task_id,
        };
        transition
    }

    fn remove_transition(&self, transition_id: String) {
        let mut tl = self.calls.borrow_mut();
        if let Some(transitions) = &mut tl.transitions {
            transitions.retain(|transition| transition.id != transition_id);
        }
    }

    fn save_to_file(&self, filename: String) {
        let path = Url::parse(&filename).unwrap().to_file_path().unwrap();
        unsafe {
            let mut file = fs::File::create(&path).unwrap();
            let tl = self.calls.try_borrow_unguarded().unwrap();
            let tls = serde_json::to_string(&tl).unwrap();
            println!("Serialized task list to: {}", path.display());
            println!("Wrote {} bytes", tls.len());
            println!("{tls}");
            file.write(tls.as_bytes()).unwrap();
        }
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
                self.calls.replace(TaskList::default());
            }
        }
    }

    fn create_model_for_empty_file() -> TasklistModel {
        let tl = TaskListComponent::default();
        component::tasklist::tasklist::TasklistModel::new(tl)
    }
}

bindings::export!(TasklistModel with_types_in bindings);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let component = TaskListComponent::default();
        let id = component.id();

        assert!(!id.is_empty());
    }

    #[test]
    fn test_tasks() {
        let component = TaskListComponent::default();
        let tasks = component.tasks();

        assert!(tasks.is_empty());
    }

    #[test]
    fn test_transitions() {
        let component = TaskListComponent::default();
        let transitions = component.transitions();

        assert!(transitions.is_empty());
    }

    #[test]
    fn test_add_task() {
        let component = TaskListComponent::default();
        let task = component.add_task(component::tasklist::tasklist::Position { x: 0.0, y: 0.0 });

        assert_eq!(component.tasks().len(), 1);
        assert_eq!(component.tasks()[0].id, task.id);
    }

    #[test]
    fn test_remove_task() {
        let component = TaskListComponent::default();
        let task = component.add_task(component::tasklist::tasklist::Position { x: 0.0, y: 0.0 });
        component.remove_task(task.id.clone());

        assert!(component.tasks().is_empty());
    }

    #[test]
    fn test_resize_task() {
        let component = TaskListComponent::default();
        let task = component.add_task(component::tasklist::tasklist::Position { x: 0.0, y: 0.0 });
        component.resize_task(
            task.id.clone(),
            component::tasklist::tasklist::Size {
                width: 2.0,
                height: 3.0,
            },
        );

        let f = component.tasks()[0].size.unwrap();
        let b = Some(component::tasklist::tasklist::Size {
            width: 2.0,
            height: 3.0,
        })
        .unwrap();
        assert_eq!(f.height, b.height);
        assert_eq!(f.width, b.width);
    }

    #[test]
    fn test_move_task() {
        let component = TaskListComponent::default();
        let task =
            component.add_task(component::tasklist::tasklist::Position { x: 100.0, y: 100.0 });
        component.move_task(
            task.id.clone(),
            component::tasklist::tasklist::Position { x: 0.0, y: 0.0 },
        );

        assert_eq!(component.tasks()[0].position.x, 0.0);
        assert_eq!(component.tasks()[0].position.x, 0.0);
    }

    #[test]
    fn test_add_transition() {
        let component = TaskListComponent::default();
        let task1 = component.add_task(component::tasklist::tasklist::Position { x: 0.0, y: 0.0 });
        let task2 = component.add_task(component::tasklist::tasklist::Position { x: 1.0, y: 1.0 });
        let transition = component.add_transition(task1.id.clone(), task2.id.clone());

        assert_eq!(component.transitions().len(), 1);
        assert_eq!(component.transitions()[0].id, transition.id);
    }

    #[test]
    fn test_remove_transition() {
        let component = TaskListComponent::default();
        let task1 = component.add_task(component::tasklist::tasklist::Position { x: 0.0, y: 0.0 });
        let task2 = component.add_task(component::tasklist::tasklist::Position { x: 1.0, y: 1.0 });
        let transition = component.add_transition(task1.id.clone(), task2.id.clone());
        component.remove_transition(transition.id.clone());

        assert!(component.transitions().is_empty());
    }
}
