mod chat;
mod loading_spinner;
mod login;
mod restore;

use iced::Task;
use iced::Theme;
use iced::window;

pub const APP_NAME: &str = "Iced Matrix Client";

enum Screen {
    Restore(restore::App),
    Login(login::App),
    Chat(chat::App),
}

#[derive(Clone)]
enum Message {
    Restore(restore::Message),
    Login(login::Message),
    Chat(chat::Message),
}

struct App {
    screen: Screen,
}

impl App {
    fn new() -> Self {
        Self {
            screen: Screen::Login(login::App::new()),
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        match &self.screen {
            Screen::Restore(restore) => restore.view().map(Message::Restore),
            Screen::Login(login) => login.view().map(Message::Login),
            Screen::Chat(chat) => chat.view().map(Message::Chat),
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match (&mut self.screen, message) {
            (Screen::Restore(restore), Message::Restore(msg)) => {
                let action = restore.update(msg);
                match action {
                    restore::Action::None => (),
                    restore::Action::Task(task) => {
                        return task.map(Message::Restore);
                    }
                }
            }
            (Screen::Login(login), Message::Login(msg)) => {
                match login.update(msg) {
                    login::Action::None => (),
                    login::Action::Task(task) => {
                        return task.map(Message::Login);
                    }
                    login::Action::LoggedIn(client) => {
                        todo!();
                    }
                }
            }
            (Screen::Chat(chat), Message::Chat(msg)) => {
                let action = chat.update(msg);
                match action {
                    chat::Action::None => (),
                    chat::Action::Task(task) => {
                        return task.map(Message::Chat);
                    }
                }
            }
            _ => {}
        }

        Task::none()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(APP_NAME)
        .window(window::Settings {
            maximized: true,
            ..Default::default()
        })
        .theme(Theme::Dark)
        .run()
}
