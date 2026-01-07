use iced::widget::{
    button, center_x, center_y, column, container, image, row, rule,
    scrollable, text, text_input,
};
use iced::{Alignment, Background, Color, Element, Length, Size, Theme};

const FONT_SIZE: u32 = 13;
const MODAL_WIDTH: f32 = 350.0;
const MODAL_HEIGHT: f32 = 370.0;
const LABEL_WIDTH: f32 = 75.0;
const TEXTBOX_WIDTH: f32 = 200.0;

#[derive(Default)]
struct Counter {
    hostname: String,
    username: String,
    password: String,
    password_visible: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    HostnameInput(String),
    HostnameSubmit,
    UsernameInput(String),
    PasswordInput(String),
    ToggleHiddenPassword,
}

fn main() -> iced::Result {
    // let base_theme = Theme::;
    iced::application(Counter::new, Counter::update, Counter::view)
        .title("Iced Matrix Login Test")
        .window_size(Size::new(800.0, 600.0))
        .theme(Theme::Dark)
        .run()
}

impl Counter {
    pub fn new() -> Self {
        Self {
            hostname: String::from("matrix.org"),
            username: String::new(),
            password: String::new(),
            password_visible: false,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::HostnameInput(string) => {
                self.hostname = string;
            }
            Message::UsernameInput(string) => {
                self.username = string;
            }
            Message::PasswordInput(string) => {
                self.password = string;
            }
            Message::ToggleHiddenPassword => {
                self.password_visible = !self.password_visible;
            }
            _ => (),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // We use a column: a simple vertical layout
        let content: Element<_> = column![
            center_x(text("Login").size(20)),
            row![
                text("Homeserver:").size(FONT_SIZE).width(LABEL_WIDTH),
                text_input("Homeserver", &self.hostname)
                    .on_input(Message::HostnameInput)
                    .size(FONT_SIZE)
                    .width(TEXTBOX_WIDTH),
                button(
                    image(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/res/search.png"
                    ))
                    .width(14)
                )
                .on_press(Message::HostnameSubmit),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            rule::horizontal(1),
            center_x(text("Login with password:").size(FONT_SIZE)),
            row![
                text("Username:").size(FONT_SIZE).width(LABEL_WIDTH),
                text_input("Username", &self.username)
                    .on_input(Message::UsernameInput)
                    .size(FONT_SIZE)
                    .width(TEXTBOX_WIDTH)
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                text("Password:").size(FONT_SIZE).width(LABEL_WIDTH),
                text_input("Password", &self.password)
                    .on_input(Message::PasswordInput)
                    .size(FONT_SIZE)
                    .secure(!self.password_visible)
                    .width(TEXTBOX_WIDTH),
                button(
                    image(concat!(env!("CARGO_MANIFEST_DIR"), "/res/eye.png"))
                        .width(14)
                )
                .on_press(Message::ToggleHiddenPassword)
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            center_x(
                button(text("Login").size(FONT_SIZE))
                    .on_press(Message::Increment)
            ),
            rule::horizontal(1),
            center_x(text("Login with SSO:").size(FONT_SIZE)),
            center_x(
                button(text("Open in browser").size(FONT_SIZE))
                    .on_press(Message::Increment)
            ),
        ]
        .max_width(MODAL_WIDTH)
        .height(MODAL_HEIGHT)
        .spacing(15)
        .padding(10)
        .into();

        container(center_x(center_y(container(
            container(scrollable(content))
                .width(Length::Shrink)
                .height(Length::Shrink)
                .style(|theme| {
                    let palette = theme.extended_palette();
                    container::Style {
                        background: Some(Background::Color(
                            palette.background.base.color,
                        )),
                        border: iced::Border {
                            radius: 5.0.into(),
                            width: 1.0,
                            color: palette.background.strong.color,
                        },
                        ..Default::default()
                    }
                }),
        ))))
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_linear_rgba(
                0.0, 0.0, 0.0, 0.2,
            ))),
            ..Default::default()
        })
        .into()
    }
}
