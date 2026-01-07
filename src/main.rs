use iced::widget::{
    button, center_x, center_y, column, container, row, rule, scrollable, text,
    text_input,
};
use iced::{Alignment, Background, Color, Element, Length, Size, Theme};

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

fn main() -> iced::Result {
    // let base_theme = Theme::;
    iced::application(Counter::default, Counter::update, Counter::view)
        .title("Iced Matrix Login Test")
        .window_size(Size::new(800.0, 600.0))
        .theme(Theme::Dark)
        .run()
}

impl Counter {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // We use a column: a simple vertical layout
        let content: Element<_> = column![
            center_x(text("Login").size(20)),
            row![
                text("Homeserver:").size(12),
                text_input("matrix.org", "matrix.org").size(12),
                button(text("🔍").size(12)).on_press(Message::Increment)
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            rule::horizontal(1),
            center_x(text("Login with password:").size(12)),
            row![
                text("Username:").size(12),
                text_input("Username", "").size(12)
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                text("Password:").size(12),
                text_input("Password", "").size(12),
                button(text("👁").size(12)).on_press(Message::Increment)
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            center_x(
                button(text("Login").size(12)).on_press(Message::Increment)
            ),
            rule::horizontal(1),
            center_x(text("Login with SSO:").size(12)),
            center_x(
                button(text("Open in browser").size(12))
                    .on_press(Message::Increment)
            ),
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            // button("+").on_press(Message::Increment),
            // We show the value of the counter here
            // text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            // button("-").on_press(Message::Decrement),
        ]
        .max_width(350)
        .height(370)
        .spacing(10)
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
