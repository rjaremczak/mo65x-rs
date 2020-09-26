use iced::{button, text_input};

struct TodoItem {
    name: String,
    done: bool,
}

impl TodoItem {
    fn new<S: Into<String>>(name: S, done: bool) -> Self {
        TodoItem {
            name: name.into(),
            done,
        }
    }
}

struct Todos {
    input: text_input::State,
    input_value: String,
    add_button: button::State,
    tasks: Vec<TodoItem>,
}

#[derive(Debug, Clone)]
pub enum Message {
    EditTaskText(String),
    CreateTask,
    SetTaskDone(usize, bool),
}

use iced::{Button, Checkbox, Column, Element, Row, Sandbox, Settings, Text, TextInput};

impl Sandbox for Todos {
    type Message = Message;

    fn new() -> Self {
        Todos {
            input: Default::default(),
            input_value: String::new(),
            add_button: Default::default(),
            tasks: vec![
                TodoItem::new("Write example", false),
                TodoItem::new("Be cool", true),
            ],
        }
    }

    fn title(&self) -> String {
        String::from("iced todo list")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::EditTaskText(val) => {
                self.input_value = val.clone();
            }
            Message::CreateTask => {
                let task_text = std::mem::replace(&mut self.input_value, String::new());
                self.tasks.push(TodoItem::new(task_text, false));
            }
            Message::SetTaskDone(index, done) => {
                self.tasks[index].done = done;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let new_item_row = Row::new()
            .push(
                TextInput::new(
                    &mut self.input,
                    "New todo",
                    &self.input_value,
                    Message::EditTaskText,
                )
                .on_submit(Message::CreateTask),
            )
            .push(
                Button::new(&mut self.add_button, Text::new("Add")).on_press(Message::CreateTask),
            );
        let mut result = Column::new().push(new_item_row);
        for (i, task) in self.tasks.iter().enumerate() {
            let task_row = Row::new().push(Text::new(&task.name)).push(Checkbox::new(
                task.done,
                "",
                move |done| Message::SetTaskDone(i, done),
            ));
            result = result.push(task_row);
        }
        result.into()
    }
}

pub fn run() {
    Todos::run(Settings::default());
}
