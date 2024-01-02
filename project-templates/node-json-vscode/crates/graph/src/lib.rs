cargo_component_bindings::generate!();
use std::cell::RefCell;

use crate::bindings::exports::component::tasklist::tasklist::{
    GuestTasklistModel, OwnTasklistModel, Position, Size, Task, Transition,
};
use cargo_component_bindings::rt::Resource;

pub struct TasklistModel {
    id: String,
    tasks: RefCell<Vec<Task>>,
    transitions: RefCell<Vec<Transition>>,
}
impl GuestTasklistModel for TasklistModel {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn tasks(&self) -> Vec<Task> {
        self.tasks.borrow_mut().clone()
    }

    fn transitions(&self) -> Vec<Transition> {
        self.transitions.borrow_mut().clone()
    }

    fn add_task(&self, position: Position) -> Task {
        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            name: "task-name".to_string(),
            position,
            size: None,
        };
        self.tasks.borrow_mut().push(task.clone());
        task
    }

    fn remove_task(&self, id: String) {
        if let Some(index) = self.tasks.borrow_mut().iter().position(|t| t.id == id) {
            self.tasks.borrow_mut().remove(index);
        }

        self.transitions
            .borrow_mut()
            .retain(|t| t.source_task_id != id && t.target_task_id != id);
    }

    fn resize_task(&self, id: String, size: Size) {
        if let Some(task) = self.tasks().iter_mut().find(|t| t.id == id) {
            task.size = Some(size);
        }
    }

    fn move_task(&self, id: String, position: Position) {
        if let Some(task) = self.tasks().iter_mut().find(|t| t.id == id) {
            task.position = position;
        }
    }

    fn add_transition(&self, source: String, target: String) -> Transition {
        let transition = Transition {
            id: uuid::Uuid::new_v4().to_string(),
            source_task_id: source,
            target_task_id: target,
        };
        self.transitions.borrow_mut().push(transition.clone());
        transition
    }

    fn remove_transition(&self, id: String) {
        let mut removed_count = 0;
        self.transitions.borrow_mut().retain(|t| {
            if t.source_task_id == id || t.target_task_id == id {
                removed_count += 1;
                false
            } else {
                true
            }
        });
    }

    fn export_to_json(&self) -> String {
        todo!()
    }

    fn import_from_json(&self, _: String) {
        todo!()
    }

    fn create_model_for_empty_file() -> OwnTasklistModel {
        let tasklistmodel: TasklistModel = TasklistModel {
            id: uuid::Uuid::new_v4().to_string(),
            tasks: vec![].into(),
            transitions: vec![].into(),
        };
        Resource::<TasklistModel>::new(tasklistmodel)
    }
}
