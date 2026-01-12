use iced::Element;
use iced::Task;
use iced::widget::row;

pub struct App {}

#[derive(Clone)]
pub enum Message {}

pub enum Action {
    None,
    Task(Task<Message>),
}

impl App {
    pub fn update(&mut self, message: Message) -> Action {
        Action::None
    }

    pub fn view(&self) -> Element<'_, Message> {
        row![].into()
    }
}
