use iced::Alignment;
use iced::Background;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Task;
use iced::Theme;
use iced::window;

use iced::widget::Column;
use iced::widget::button;
use iced::widget::center_x;
use iced::widget::center_y;
use iced::widget::container;
use iced::widget::image;
use iced::widget::row;
use iced::widget::rule;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;

use matrix_sdk::Client;
// use matrix_sdk::Room;
// use matrix_sdk::RoomState;
// use matrix_sdk::config::SyncSettings;
use matrix_sdk::ruma::api::client::session::get_login_types::v3::LoginType;
// use matrix_sdk::ruma::events::room::message::MessageType;
// use matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent;

use url::Url;

const FONT_SIZE: u32 = 13;
const MODAL_WIDTH: f32 = 350.0;
const MODAL_HEIGHT: f32 = 370.0;
const LABEL_WIDTH: f32 = 75.0;
const TEXTBOX_WIDTH: f32 = 200.0;

#[derive(Default)]
struct App {
    hostname: String,
    username: String,
    password: String,
    password_visible: bool,
    homeserver_state: HomeserverState,
    // client: Option<Client>,
}

#[derive(Clone)]
pub struct AuthTypes {
    password: bool,
    sso: bool,
}

#[derive(Default, Clone)]
enum HomeserverState {
    #[default]
    Idle,
    Connecting,
    GettingAuthTypes,
    AuthTypes(AuthTypes),
    Error(String),
}

#[derive(Clone)]
pub enum Message {
    HostnameInput(String),
    HostnameSubmit,
    UsernameInput(String),
    PasswordInput(String),
    ToggleHiddenPassword,
    ClientConnecton(Result<Client, String>),
    AuthTypes(Result<AuthTypes, String>),
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Iced Matrix Login Test")
        .window(window::Settings {
            maximized: true,
            ..Default::default()
        })
        .theme(Theme::Dark)
        .run()
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                hostname: String::from("matrix.org"),
                username: String::new(),
                password: String::new(),
                password_visible: false,
                homeserver_state: HomeserverState::default(),
            },
            Task::done(Message::HostnameSubmit),
        )
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
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
            Message::HostnameSubmit => {
                println!("Hostname submit");
                self.homeserver_state = HomeserverState::Connecting;
                return Task::perform(
                    connect_to_client(self.hostname.clone()),
                    Message::ClientConnecton,
                );
            }
            Message::ClientConnecton(result) => match result {
                Ok(client) => {
                    // self.client = Some(client)
                    self.homeserver_state = HomeserverState::GettingAuthTypes;
                    return Task::perform(
                        get_auth_types(client),
                        Message::AuthTypes,
                    );
                }
                Err(error) => {
                    self.homeserver_state = HomeserverState::Error(error)
                }
            },
            Message::AuthTypes(result) => match result {
                Ok(auth_types) => {
                    self.homeserver_state =
                        HomeserverState::AuthTypes(auth_types)
                }
                Err(error) => {
                    self.homeserver_state = HomeserverState::Error(error)
                }
            },
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let submit_hostname_button = button(
            image(concat!(env!("CARGO_MANIFEST_DIR"), "/res/search.png"))
                .width(14),
        );
        let hostname_textbox = text_input("Homeserver", &self.hostname)
            .on_input(Message::HostnameInput)
            .size(FONT_SIZE)
            .width(TEXTBOX_WIDTH);

        let mut items: Vec<Element<Message>> = Vec::new();

        items.push(center_x(text("Login").size(20)).into());
        items.push(
            row![
                text("Homeserver:").size(FONT_SIZE).width(LABEL_WIDTH),
                match self.homeserver_state {
                    HomeserverState::Connecting
                    | HomeserverState::GettingAuthTypes =>
                        hostname_textbox,
                    _ => hostname_textbox
                        .on_submit(Message::HostnameSubmit),
                },
                match self.homeserver_state {
                    HomeserverState::Connecting
                    | HomeserverState::GettingAuthTypes =>
                        submit_hostname_button,
                    _ => submit_hostname_button
                        .on_press(Message::HostnameSubmit),
                }
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .into(),
        );
        items.push(rule::horizontal(1).into());

        match self.homeserver_state {
            HomeserverState::Idle => (),
            HomeserverState::Connecting => {
                items.push(text("Constructing client").size(FONT_SIZE).into());
            }
            HomeserverState::GettingAuthTypes => {
                items.push(
                    text("Requesting available login methods from homeserver")
                        .size(FONT_SIZE)
                        .into(),
                );
            }
            HomeserverState::Error(ref error) => {
                items.push(text(error).size(FONT_SIZE).into());
            }
            HomeserverState::AuthTypes(ref auth_types) => {
                if auth_types.password {
                    items.push(
                        center_x(text("Login with password:").size(FONT_SIZE))
                            .into(),
                    );
                    // Username box
                    items.push(
                        row![
                            text("Username:")
                                .size(FONT_SIZE)
                                .width(LABEL_WIDTH),
                            text_input("Username", &self.username)
                                .on_input(Message::UsernameInput)
                                .size(FONT_SIZE)
                                .width(TEXTBOX_WIDTH)
                        ]
                        .spacing(10)
                        .align_y(Alignment::Center)
                        .into(),
                    );
                    // Password box
                    items.push(
                        row![
                            text("Password:")
                                .size(FONT_SIZE)
                                .width(LABEL_WIDTH),
                            text_input("Password", &self.password)
                                .on_input(Message::PasswordInput)
                                .size(FONT_SIZE)
                                .secure(!self.password_visible)
                                .width(TEXTBOX_WIDTH),
                            button(
                                image(concat!(
                                    env!("CARGO_MANIFEST_DIR"),
                                    "/res/eye.png"
                                ))
                                .width(14)
                            )
                            .on_press(Message::ToggleHiddenPassword)
                        ]
                        .spacing(10)
                        .align_y(Alignment::Center)
                        .into(),
                    );

                    items.push(
                        center_x(button(text("Login").size(FONT_SIZE))).into(),
                    );
                    if auth_types.sso {
                        items.push(rule::horizontal(1).into());
                    }
                }
                if auth_types.sso {
                    items.push(
                        center_x(text("Login with SSO:").size(FONT_SIZE))
                            .into(),
                    );
                    items.push(
                        center_x(button(
                            text("Open in browser").size(FONT_SIZE),
                        ))
                        .into(),
                    );
                }
            }
        };

        let content = Column::with_children(items)
            .max_width(MODAL_WIDTH)
            .height(MODAL_HEIGHT)
            .spacing(15)
            .padding(10);

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
                        shadow: iced::Shadow {
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                            blur_radius: 0.0,
                            offset: iced::Vector { x: 10.0, y: 10.0 },
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

async fn connect_to_client(hostname: String) -> Result<Client, String> {
    let homeserver_address = format!("https://{}", hostname);
    let Ok(homeserver_url) = Url::parse(&homeserver_address) else {
        return Err("Failed to parase homeserver URL".to_string());
    };

    let Ok(client) = Client::new(homeserver_url).await else {
        return Err("Could not create client".to_string());
    };

    Ok(client)
}

async fn get_auth_types(client: Client) -> Result<AuthTypes, String> {
    let Ok(login_types) = client.matrix_auth().get_login_types().await else {
        return Err(format!(
            "{} \n\n{}",
            "Could not retrieve login methods from homeserver.",
            "Check the spelling of the homeserver and your internet connection."
        ));
    };

    let mut auth_types = AuthTypes {
        password: false,
        sso: false,
    };
    for login_type in login_types.flows {
        match login_type {
            LoginType::Password(_) => auth_types.password = true,
            LoginType::Sso(sso) => {
                if sso.identity_providers.is_empty() {
                    auth_types.sso = true;
                }
            }
            _ => {}
        }
    }
    Ok(auth_types)
}
