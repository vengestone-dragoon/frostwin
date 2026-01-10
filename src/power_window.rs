use crate::styles::colored_button;
use crate::Message;
use iced::widget::{button, canvas, column, container, image, row, text};
use iced::{window, Alignment, Color, Element, Length, Size, Task};
use std::any::Any;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use dirs::data_dir;
use iced::advanced::image::Handle;
use iced::futures::future::err;
use crate::raw_icons::start_icon;
use crate::sys_util::{lock, logoff, windows_power};

#[derive(Debug, Clone)]
pub enum PowerOptions {
    Shutdown,
    Reboot,
    LogOff,
    Lock,
}

#[derive(Debug, Clone)]
pub enum PowerMenuMessage {
    Tick,
    Execute(PowerOptions),
    Cancel,
    Init,
}

pub struct PowerWindow {
    pub id: window::Id,
    time: Instant,
    target: PowerOptions
}
impl PowerWindow {
    pub fn new(target: PowerOptions) -> (Self,Task<window::Id>) {
        let mut settings = window::Settings::default();
        settings.decorations = false;
        settings.resizable = false;
        settings.min_size = None;
        settings.max_size = None;
        settings.icon = None;
        settings.transparent = true;
        settings.closeable = false;
        settings.minimizable = false;
        settings.fullscreen = false;
        settings.position = window::Position::Centered;
        settings.size = Size::new(500.0,300.0);
        settings.level = window::Level::AlwaysOnTop;
        let (id,open_task) = window::open(settings);
        (Self {
            id,
            time: Instant::now(),
            target,
        },
         open_task)
    }
    pub fn update(&mut self, message: PowerMenuMessage) -> Task<Message> {
        match message {
            PowerMenuMessage::Init => {
                Task::none()
            }
            PowerMenuMessage::Execute(option) => {
                match option {
                    PowerOptions::Shutdown => {
                        let result = windows_power(false);
                        match result {
                            Ok(_) => {
                                iced::exit()
                            }
                            Err(error) => {
                                println!("shutdown error: {}", error);
                                Task::none()
                            }
                        }
                    }
                    PowerOptions::LogOff => {
                        let result: windows::core::Result<()> = logoff();
                        match result {
                            Ok(_) => {
                                iced::exit()
                            }
                            Err(error) => {
                                println!("logoff error: {}",error);
                                Task::none()
                            }
                        }
                    }
                    PowerOptions::Lock => {
                        let result = lock();
                        match result {
                            Ok(_) => {
                                Task::none()
                            }
                            Err(error) => {
                                println!("lock error: {}",error);
                                Task::none()
                            }
                        }
                    }
                    PowerOptions::Reboot => {
                        let result = windows_power(true);
                        match result {
                            Ok(_) => {
                                iced::exit()
                            }
                            Err(error) => {
                                println!("reboot error: {}", error);
                                Task::none()
                            }
                        }
                    }
                }
            }
            PowerMenuMessage::Tick => {
                if self.time.elapsed().as_secs() > 30 {
                    Task::done(Message::PowerMenu(PowerMenuMessage::Execute(self.target.clone())))
                } else {
                    Task::none()
                }
            }
            PowerMenuMessage::Cancel => {
                window::close(self.id).map(Message::WindowClosed)
            }
        }
    }
    pub fn view(&self, app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>) -> Element<'_, Message> {
        let elapsed = self.time.elapsed().as_secs();
        let time_remaining = 30_u64.saturating_sub(elapsed);
        let task =
        match self.target.clone() {
            PowerOptions::Shutdown => {
                "Shutting down"
            }
            PowerOptions::Lock => {
                "Locking"
            }
            PowerOptions::Reboot => {
                "Rebooting"
            }
            PowerOptions::LogOff => {
                "Logging Off"
            }
        };
        let (
            lock_button,
            logoff_button,
            restart_button,
            shutdown_button,
            cancel_button,
        ) = match (app_image_cache.lock(),data_dir()) {
            (Ok(app_image_lock),Some(data_dir)) => {
                let data_folder = data_dir.join("Frostwin");
                let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
                (
                    app_image_lock.get(&data_folder.join("icons/power/Lock.png")).unwrap_or(&error_handle).clone(),
                    app_image_lock.get(&data_folder.join("icons/power/Logoff.png")).unwrap_or(&error_handle).clone(),
                    app_image_lock.get(&data_folder.join("icons/power/Restart.png")).unwrap_or(&error_handle).clone(),
                    app_image_lock.get(&data_folder.join("icons/power/Shutdown.png")).unwrap_or(&error_handle).clone(),
                    app_image_lock.get(&data_folder.join("icons/power/Cancel.png")).unwrap_or(&error_handle).clone(),
                )
            }
            (Err(error),_) => {
                eprintln!("Error getting app image cache: {}", error);
                let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
                (
                    error_handle.clone(),
                    error_handle.clone(),
                    error_handle.clone(),
                    error_handle.clone(),
                    error_handle.clone(),
                )
            }
            (_,None) => {
                eprintln!("Error getting data directory");
                let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
                (
                    error_handle.clone(),
                    error_handle.clone(),
                    error_handle.clone(),
                    error_handle.clone(),
                    error_handle.clone(),
                )
            }
        };
        container(
            column![
                container(
                    image(start_icon(app_image_cache.clone(),true)).width(Length::Fixed(124.0)).height(Length::Fixed(124.0)),
                ).align_x(Alignment::Center).align_y(Alignment::Center).width(Length::Fill).height(Length::FillPortion(4)),
                row![
                    text!("{} in {}s", task, time_remaining),
                ].align_y(Alignment::Center).height(Length::FillPortion(2)),
                row![
                    button(
                        column![
                            image(lock_button).width(Length::Fill).height(Length::Fill),
                        ]
                    ).on_press(Message::PowerMenu(PowerMenuMessage::Execute(PowerOptions::Lock)))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(0)
                    .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.2,0.2,0.7))),
                    button(
                        column![
                            image(logoff_button).width(Length::Fill).height(Length::Fill),
                        ]
                    ).on_press(Message::PowerMenu(PowerMenuMessage::Execute(PowerOptions::LogOff)))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(0)
                    .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.7,0.5,0.4))),
                    button(
                        column![
                            image(restart_button).width(Length::Fill).height(Length::Fill),
                        ]
                    ).on_press(Message::PowerMenu(PowerMenuMessage::Execute(PowerOptions::Reboot)))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(0)
                    .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.2,0.7,0.2))),
                    button(
                        column![
                            image(shutdown_button).width(Length::Fill).height(Length::Fill),
                        ]
                    ).on_press(Message::PowerMenu(PowerMenuMessage::Execute(PowerOptions::Shutdown)))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(0)
                    .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.7,0.2,0.2))),
                    button(
                        column![
                            image(cancel_button).width(Length::Fill).height(Length::Fill),
                        ]
                    ).on_press(Message::PowerMenu(PowerMenuMessage::Cancel))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(0)
                    .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.4,0.4,0.4))),
                ].spacing(2.0).height(Length::FillPortion(3))
            ].align_x(Alignment::Center),
        ).style(container::transparent).align_x(Alignment::Center).align_y(Alignment::Center).width(Length::Fill).height(Length::Fill).into()

    }
}
