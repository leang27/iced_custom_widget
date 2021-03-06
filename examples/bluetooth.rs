use iced::{
    button, executor, scrollable, text_input, Align, Application, Button, Checkbox, Column,
    Command, Container, Element, HorizontalAlignment, Length, Row, Rule, Scrollable, Settings,
    Space, Subscription, Text, TextInput, VerticalAlignment,
};
use iced_custom_widget as icw;
use iced_native::window::Event;
use iced_native::Event::Window;
use icw::components::{Icon, Toggler};
use styles::{ButtonStyle, ContainerStyle, InputStyle, RuleStyle};
#[derive(Default, Debug, Clone)]
pub struct KBleutooth {
    is_enable: bool,
    is_allowed: bool,
    is_shown: bool,
    is_input: bool,
    is_shown_settings: bool,
    edit_dev: button::State,
    show_settings: button::State,
    refresh: button::State,
    device_name: String,
    dev_name: text_input::State,
    dev_name_val: String,
    bluetooth_settings: BluetoothSettings,
    btn_refresh: button::State,
    vector_bluetooths: Vec<(BluetoothDevType, String, BluetoothStatus)>,
    scroll_area: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum BluetoothStatus {
    Connected,
    Connecting,
    NoConnected,
    DisConnected,
}
#[derive(Debug, Clone)]
pub enum BluetoothDevType {
    SmartPhone,
    Computer,
    Headphone,
    Unknown,
}
impl Default for BluetoothDevType {
    fn default() -> Self {
        BluetoothDevType::SmartPhone
    }
}

#[derive(Debug, Clone)]
pub enum KBleutoothMsg {
    DevEdited,
    DevEditedVal(String),
    DevEditedSubmmit,
    DevEnabled(bool),
    DevAllowed(bool),
    DevRefreshed,
    DevSettingsShown,
    DevShowNameless(bool),
    CloseApp,
    Escape,
    BluetoothSettingsMsg(BluetoothSettingsMsg),
    WindowResize((u32, u32)),
    FileDrop(std::path::PathBuf),
}

impl Application for KBleutooth {
    type Executor = executor::Default;
    type Message = KBleutoothMsg;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<KBleutoothMsg>) {
        let simpler_code = |b_type: BluetoothDevType, b_ssid: &str, b_status: BluetoothStatus| {
            (b_type, b_ssid.to_string(), b_status)
        };
        let mut init_vec_state: Vec<(BluetoothDevType, String, BluetoothStatus)> = Vec::new();
        for _i in 1..=10 {
            init_vec_state.push(simpler_code(
                BluetoothDevType::Computer,
                "Mi Smart Band 5",
                BluetoothStatus::NoConnected,
            ));
        }
        (
            Self {
                vector_bluetooths: init_vec_state,
                is_input: false,
                device_name: "sna-koompi".to_string(),
                bluetooth_settings: BluetoothSettings::new(),
                ..KBleutooth::default()
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Bluetooth")
    }
    fn update(&mut self, message: KBleutoothMsg) -> Command<Self::Message> {
        use KBleutoothMsg::*;
        match message {
            DevEnabled(is_enable) => {
                self.is_enable = is_enable;
                Command::none()
            }
            DevAllowed(is_allow) => {
                self.is_allowed = is_allow;
                Command::none()
            }
            DevShowNameless(data) => {
                self.is_shown = data;
                Command::none()
            }
            CloseApp => {
                println!("Applicaiton close:");
                Command::none()
            }
            BluetoothSettingsMsg(msg) => {
                self.bluetooth_settings.update(msg);
                Command::none()
            }
            DevEditedSubmmit => {
                self.is_input = !self.is_input;
                self.device_name = self.dev_name_val.to_string();
                Command::none()
            }
            DevSettingsShown => {
                self.is_shown_settings = !self.is_shown_settings;
                Command::none()
            }
            DevEdited => {
                self.is_input = !self.is_input;
                Command::none()
            }
            DevEditedVal(val) => {
                self.dev_name_val = val;
                Command::none()
            }
            WindowResize((w, h)) => {
                println!("width: {} & height: {}", w, h);
                if w <= 603 {
                } else {
                }
                Command::none()
            }
            FileDrop(path) => {
                println!("path: {:?}", path.as_path());
                Command::none()
            }

            Escape => {
                println!("Escape key pressed: ");
                self.is_shown_settings = !self.is_shown_settings;
                Command::none()
            }
            _ => Command::none(),
        }
    }
    fn subscription(&self) -> Subscription<KBleutoothMsg> {
        iced_native::subscription::events_with(|event, status| {
            if let iced_native::event::Status::Captured = status {
                return None;
            }

            match event {
                Window(Event::FileDropped(path)) => Some(KBleutoothMsg::FileDrop(path)),
                Window(Event::Resized { width, height }) => {
                    Some(KBleutoothMsg::WindowResize((width, height)))
                }
                iced_native::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) => match key_code {
                    iced::keyboard::KeyCode::W => {
                        if modifiers.control {
                            Some(KBleutoothMsg::CloseApp)
                        } else {
                            None
                        }
                    }
                    iced::keyboard::KeyCode::Escape => Some(KBleutoothMsg::Escape),
                    _ => None,
                },
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<KBleutoothMsg> {
        let inner_layout = Container::new(
            Column::new()
                .spacing(10)
                .push(
                    Row::new()
                        .push(
                            Row::new()
                                .width(Length::FillPortion(1))
                                .align_items(Align::Center)
                                .spacing(4)
                                .push(Text::new(&self.device_name))
                                .push(if self.is_input {
                                    Row::new().push(
                                        TextInput::new(
                                            &mut self.dev_name,
                                            "",
                                            &self.dev_name_val,
                                            KBleutoothMsg::DevEditedVal,
                                        )
                                        .on_submit(KBleutoothMsg::DevEditedSubmmit)
                                        .padding(6)
                                        .style(InputStyle::InkBorder),
                                    )
                                } else {
                                    Row::new().push(
                                        Button::new(&mut self.edit_dev, Icon::new('\u{f304}'))
                                            .on_press(KBleutoothMsg::DevEdited)
                                            .style(ButtonStyle::Transparent),
                                    )
                                }),
                        )
                        .push(
                            Toggler::new(
                                self.is_enable,
                                String::from(""),
                                KBleutoothMsg::DevEnabled,
                            )
                            .width(Length::FillPortion(1)),
                        ),
                )
                .push(Rule::horizontal(10).style(RuleStyle {}))
                .push(if self.is_enable {
                    Row::new()
                        .push(Text::new(
                            "Allow other Bluetooth devices to find this device",
                        ))
                        .push(Toggler::new(
                            self.is_allowed,
                            String::from(""),
                            KBleutoothMsg::DevAllowed,
                        ))
                } else {
                    Row::new().push(Text::new(
                        "Enable Bluetooth for devices (Mouse, Keyboard, Headphone)",
                    ))
                }),
        )
        .width(Length::Fill)
        .padding(10)
        .style(ContainerStyle::LightGrayCircle);
        let know_devices = Column::new()
            .spacing(10)
            .push(Text::new("My Devices").size(24))
            .push(
                Column::new()
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .push(
                        Row::new()
                            .spacing(6)
                            .push(Icon::new('\u{f10b}'))
                            .push(Text::new("Linux"))
                            .push(Space::with_width(Length::Fill))
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .spacing(4)
                                    .push(Text::new("Not Connected"))
                                    .push(
                                        Button::new(&mut self.show_settings, Icon::new('\u{f105}'))
                                            .on_press(KBleutoothMsg::DevSettingsShown)
                                            .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
                                    ),
                            ),
                    ),
            );
        let other_devices = Column::new()
            .spacing(10)
            .push(Text::new("Other Devices").size(24))
            .push(
                Row::new()
                    .push(Checkbox::new(
                        self.is_shown,
                        "Show Bluetooth devices without names",
                        KBleutoothMsg::DevShowNameless,
                    ))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Button::new(&mut self.btn_refresh, Icon::new('\u{f021}'))
                            .on_press(KBleutoothMsg::DevRefreshed)
                            .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
                    ),
            )
            .push(self.vector_bluetooths.iter_mut().fold(
                Column::new().padding(10).spacing(16),
                |column, (b_type, b_ssid, b_status)| {
                    column.push(
                        Row::new()
                            .align_items(Align::Center)
                            .spacing(4)
                            .push(
                                Icon::new(match b_type {
                                    BluetoothDevType::Computer => '\u{f108}',
                                    BluetoothDevType::Headphone => '\u{f3cd}',
                                    BluetoothDevType::SmartPhone => '\u{f58f}',
                                    BluetoothDevType::Unknown => '\u{f17c}',
                                })
                                .size(24),
                            )
                            .push(Text::new(b_ssid.as_str()))
                            .push(Space::with_width(Length::Fill))
                            .push(Text::new(match b_status {
                                BluetoothStatus::Connected => "Connected",
                                BluetoothStatus::Connecting => "Connecting",
                                BluetoothStatus::DisConnected => "Disconnected",
                                BluetoothStatus::NoConnected => "Not connected",
                            })),
                    )
                },
            ));
        let scroll_conent = Scrollable::new(&mut self.scroll_area)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .push(
                Column::new()
                    .spacing(20)
                    .push(inner_layout)
                    .push(if self.is_enable {
                        Column::new().push(know_devices).push(other_devices)
                    } else {
                        Column::new()
                    }),
            );
        let embbeded_layout = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                scroll_conent
                    .padding(10)
                    .scroller_width(4)
                    .scrollbar_width(4),
            )
            .push(if self.is_shown_settings {
                self.bluetooth_settings
                    .view()
                    .map(move |msg| KBleutoothMsg::BluetoothSettingsMsg(msg))
            } else {
                Space::with_width(Length::Shrink).into()
            });
        let inner_container = Container::new(embbeded_layout)
            .style(ContainerStyle::White)
            .padding(10);
        Container::new(inner_container)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(ContainerStyle::LightGray)
            .into()
    }
}

#[derive(Default, Debug, Clone)]
pub struct BluetoothSettings {
    connected_host: text_input::State,
    connected_host_val: String,
    disconn_btn: button::State,
    ignore_dev: button::State,
    send_file: button::State,
    hide_btn: button::State,
}
#[derive(Debug, Clone)]
pub enum BluetoothSettingsMsg {
    BluetothNameChanged(String),
    Disconnected,
    Ignoranced,
    SendFile,
    HideSettings,
    SubmitChanged,
}

impl BluetoothSettings {
    fn new() -> Self {
        Self {
            connected_host_val: String::from("sna-koompi"),
            ..Default::default()
        }
    }
    fn update(&mut self, msg: BluetoothSettingsMsg) {
        match msg {
            BluetoothSettingsMsg::BluetothNameChanged(val) => {
                self.connected_host_val = val;
            }
            BluetoothSettingsMsg::Disconnected => {}
            BluetoothSettingsMsg::Ignoranced => {}
            BluetoothSettingsMsg::SendFile => {}
            BluetoothSettingsMsg::HideSettings => {}
            BluetoothSettingsMsg::SubmitChanged => {
                println!("data submit");
            }
        }
    }
    fn view(&mut self) -> Element<BluetoothSettingsMsg> {
        let blue_settings_layout = Column::new()
            .spacing(10)
            .padding(10)
            .height(Length::Fill)
            .push(
                Button::new(&mut self.hide_btn, Icon::new('\u{f104}'))
                    .on_press(BluetoothSettingsMsg::HideSettings)
                    .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
            )
            .push(
                Column::new()
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .push(Text::new("Connected Host Bluetooth").size(16)),
            )
            .push(
                TextInput::new(
                    &mut self.connected_host,
                    &self.connected_host_val,
                    "",
                    BluetoothSettingsMsg::BluetothNameChanged,
                )
                .on_submit(BluetoothSettingsMsg::SubmitChanged)
                .padding(6)
                .style(InputStyle::InkBorder),
            )
            .push(
                Button::new(
                    &mut self.disconn_btn,
                    Text::new("Disconnect")
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Fill)
                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                .on_press(BluetoothSettingsMsg::Disconnected),
            )
            .push(
                Button::new(
                    &mut self.ignore_dev,
                    Text::new("Ignore this device")
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Fill)
                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                .on_press(BluetoothSettingsMsg::Ignoranced),
            )
            .push(
                Button::new(
                    &mut self.send_file,
                    Text::new("Send Files")
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Fill)
                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                .on_press(BluetoothSettingsMsg::SendFile),
            );
        Container::new(blue_settings_layout)
            .center_x()
            .center_y()
            .width(Length::FillPortion(1))
            .style(ContainerStyle::LightGrayCircle)
            .into()
    }
}

pub fn init() -> iced::Result {
    KBleutooth::run(Settings::default())
}
fn main() {
    match init() {
        Ok(()) => println!("run sucessfully"),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("Hello World from Koompi Bluetooth");
}

mod styles {
    use iced::{button, container, progress_bar, rule, text_input, Background, Color, Vector};
    pub enum ButtonStyle {
        Default,
        Circular(u8, u8, u8, f32),
        BigCircular(u8, u8, u8, f32),
        CircleRadius(u8, u8, u8, f32, f32, Color),
        Transparent,
    }

    impl button::StyleSheet for ButtonStyle {
        fn active(&self) -> button::Style {
            button::Style {
                shadow_offset: Vector::new(0.0, 0.0),
                background: match self {
                    ButtonStyle::Default => Some(Background::Color([0.87, 0.87, 0.87].into())),
                    ButtonStyle::Circular(c1, c2, c3, p)
                    | ButtonStyle::CircleRadius(c1, c2, c3, p, _, _)
                    | ButtonStyle::BigCircular(c1, c2, c3, p) => {
                        Some(Background::Color(Color::from_rgba8(*c1, *c2, *c3, *p)))
                    }
                    ButtonStyle::Transparent => Some(Background::Color(Color::TRANSPARENT)),
                },
                border_radius: match self {
                    ButtonStyle::Default | ButtonStyle::Circular(_, _, _, _) => 4.0,
                    ButtonStyle::BigCircular(_, _, _, _) => 25.0,
                    ButtonStyle::Transparent => 0.0,
                    ButtonStyle::CircleRadius(_, _, _, _, r, _) => *r,
                },
                border_width: 0.0,
                border_color: [0.7, 0.7, 0.7].into(),
                text_color: match self {
                    ButtonStyle::Default
                    | ButtonStyle::BigCircular(_, _, _, _)
                    | ButtonStyle::Circular(_, _, _, _) => Color::WHITE,
                    ButtonStyle::Transparent => Color::BLACK,
                    ButtonStyle::CircleRadius(_, _, _, _, _, color) => *color,
                },
            }
        }
    }

    pub enum ContainerStyle {
        Custom,
        InkColor,
        LightGray,
        White,
        LightGrayCircle,
        Black,
    }
    impl container::StyleSheet for ContainerStyle {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: None,
                background: match self {
                    ContainerStyle::Custom => {
                        Some(Background::Color(Color::from_rgba8(223, 228, 234, 1.0)))
                    }
                    ContainerStyle::InkColor => {
                        Some(Background::from(Color::from_rgba8(206, 214, 224, 1.0)))
                    }
                    ContainerStyle::LightGray => {
                        Some(Background::from(Color::from_rgba8(215, 219, 221, 1.0)))
                    }
                    ContainerStyle::White => {
                        Some(Background::from(Color::from_rgba8(255, 255, 255, 1.0)))
                    }
                    ContainerStyle::LightGrayCircle => {
                        Some(Background::from(Color::from_rgba8(215, 219, 221, 0.5)))
                    }
                    ContainerStyle::Black => Some(Background::from(Color::BLACK)),
                },
                border_radius: match self {
                    ContainerStyle::Custom
                    | ContainerStyle::LightGrayCircle
                    | ContainerStyle::White
                    | ContainerStyle::InkColor
                    | ContainerStyle::Black => 10.0,
                    ContainerStyle::LightGray => 0.0,
                },
                border_width: 0.0,
                border_color: Color::from_rgba8(255, 255, 255, 1.0),
            }
        }
    }
    pub enum SliderStyle {
        Default,
        Circle(u8, u8, u8, f32, f32),
        BigCircle(u8, u8, u8, f32, f32),
        WhiteGrayCircle(u8, u8, u8, f32, f32),
    }
    impl progress_bar::StyleSheet for SliderStyle {
        fn style(&self) -> progress_bar::Style {
            progress_bar::Style {
                background: Background::Color(Color::from_rgb(0.6, 0.6, 0.6)),
                bar: match self {
                    SliderStyle::WhiteGrayCircle(r, b, g, alpha, _)
                    | SliderStyle::Circle(r, b, g, alpha, _)
                    | SliderStyle::BigCircle(r, b, g, alpha, _) => {
                        Background::Color(Color::from_rgba8(*r, *b, *g, *alpha))
                    }
                    SliderStyle::Default => Background::Color(Color::from_rgb(0.3, 0.9, 0.3)),
                },
                border_radius: match self {
                    SliderStyle::WhiteGrayCircle(_, _, _, _, r)
                    | SliderStyle::BigCircle(_, _, _, _, r)
                    | SliderStyle::Circle(_, _, _, _, r) => *r,
                    SliderStyle::Default => 5.0,
                },
            }
        }
    }

    pub struct RuleStyle {}

    impl rule::StyleSheet for RuleStyle {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: Color::WHITE,
                width: 1,
                radius: 0.0,
                fill_mode: rule::FillMode::Percent(100.0),
            }
        }
    }

    pub enum InputStyle {
        Default,
        CircularBorder,
        InkBorder,
    }
    impl text_input::StyleSheet for InputStyle {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: Background::Color(Color::from_rgba8(215, 219, 221, 0.5)),
                border_radius: 8.0,
                border_width: 0.0,
                border_color: Color::from_rgb(0.7, 0.7, 0.7),
            }
        }
        fn focused(&self) -> text_input::Style {
            text_input::Style {
                border_color: Color::from_rgb(0.5, 0.5, 0.5),
                background: Background::from(Color::from_rgba8(215, 219, 221, 0.5)),
                border_width: match self {
                    InputStyle::Default => 1.0,
                    InputStyle::CircularBorder | InputStyle::InkBorder => 2.0,
                },
                ..self.active()
            }
        }
        fn placeholder_color(&self) -> Color {
            Color::from_rgb(0.7, 0.7, 0.7)
        }
        fn value_color(&self) -> Color {
            Color::from_rgba8(86, 101, 115, 1.0)
        }
        fn selection_color(&self) -> Color {
            Color::from_rgba(1.0, 1.0, 1.0, 1.0)
        }
    }
}
