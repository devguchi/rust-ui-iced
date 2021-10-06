use iced::{button, Button, Column, Container, Element, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

#[derive(Default)]
struct Hello {
    hoge_count: i32,
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

    fn update(&mut self, message: Message) {
        match message {
            Message::BtnClicked => {
                self.hoge_count += 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let hoge_count = Text::new(self.hoge_count.to_string()).size(50);
        let btn = Button::new(&mut self.btn_on, Text::new("hoge")).on_press(Message::BtnClicked);
        let content = Column::new().padding(20).push(hoge_count).push(btn);
        Container::new(content).into()
    }
}
