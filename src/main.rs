use iced::{button, Button, Column, Container, Element, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

#[derive(Default)]
struct Hello {
    btn_on: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    BtnClicked,
}

impl Sandbox for Hello {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("BUTTON")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Message> {
        let btn = Button::new(&mut self.btn_on, Text::new("hoge")).on_press(Message::BtnClicked);
        let content = Column::new().push(btn);
        Container::new(content).into()
    }
}
