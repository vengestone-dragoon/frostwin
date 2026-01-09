#![windows_subsystem = "windows"]

mod taskbar;
mod desktop;
mod start_menu;
mod panel_menu;
mod icons;
mod styles;
mod sys_util;
mod power_window;
mod windows_icons;

use crate::desktop::{Desktop, DesktopMessage};
use crate::icons::FrostwinIcons;
use crate::panel_menu::{PanelMenu, PanelMessage};
use crate::power_window::{PowerMenuMessage, PowerOptions, PowerWindow};
use crate::start_menu::{StartMenu, StartMessage};
use crate::sys_util::{get_battery_info, get_sound_state, get_wifi_status, set_sound_state, WifiStatus};
use crate::taskbar::{Taskbar, TaskbarMessage};
use dirs::data_dir;
use iced::time::{self, milliseconds};
use iced::widget::column;
use iced::widget::image::Handle;
use iced::{window, Size, Subscription, Task};
use std::any::Any;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() -> iced::Result {
    iced::daemon::daemon(AppMain::new, AppMain::update, AppMain::view)
        .subscription(AppMain::subscription)
        .title(AppMain::title)
        .theme(AppMain::theme)
        .scale_factor(AppMain::scale_factor)
        .run()
}
#[derive(Debug, Clone)]
pub enum Message {
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    WindowLostFocus(window::Id),
    WindowCaughtFocus(window::Id),
    WindowClose(window::Id),
    OpenStartMenu,
    MetaPressed,
    OpenPanelMenu,
    OpenPowerWindow(PowerOptions),
    Taskbar(TaskbarMessage),
    Desktop(DesktopMessage),
    StartMenu(StartMessage),
    PanelMenu(PanelMessage),
    PowerMenu(PowerMenuMessage),
    VolumeChange(f32),
    VolumeMute,
    ResizeContext(Size),
    Tick(Instant),
    None,
}
struct AppMain {
    icon_cache: Arc<Mutex<BTreeMap<FrostwinIcons,Box<dyn Any>>>>,
    app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>,
    taskbar: Taskbar,
    desktop: Desktop,
    start_menu: Option<StartMenu>,
    panel_menu: Option<PanelMenu>,
    power_window: Option<PowerWindow>,
    battery: Option<(f32,bool)>,
    base_size: f32,
    wifi_status: WifiStatus,
    system_volume: f32,
    volume_muted: bool,
}
impl AppMain {
    pub fn new() -> (Self,Task<Message>) {
        if let Some(mut data_folder) = data_dir() {
            data_folder.push("Frostwin");
            if let Err(e) = std::fs::create_dir_all(&data_folder) {
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        eprintln!("Error: Permission denied creating directory {:?}", data_folder);
                    }
                    std::io::ErrorKind::AlreadyExists => {
                        eprintln!("Error: A file already exists at the folder path {:?}", data_folder);
                    }
                    _ => {
                        eprintln!("Error creating data directory: {}", e);
                    }
                }
            }
        } else {
            eprintln!("Error: Could not determine the system data directory.");
        }
        let icon_cache: Arc<Mutex<BTreeMap<FrostwinIcons,Box<dyn Any>>>> = Arc::new(Mutex::new(BTreeMap::new()));
        let app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>> = Arc::new(Mutex::new(BTreeMap::new()));
        let (desktop,open_desktop) = Desktop::new();
        let (taskbar,open_taskbar) = Taskbar::new();
        (
            Self {
                icon_cache,
                app_image_cache,
                taskbar,
                desktop,
                start_menu: None,
                panel_menu: None,
                power_window: None,
                battery: None,
                base_size: 1.0,
                wifi_status: WifiStatus::Disconnected,
                system_volume: 0.0,
                volume_muted: false,
            },
            Task::batch([
                open_taskbar.map(Message::WindowOpened),
                open_desktop.map(Message::WindowOpened)
            ])
        )
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(id) => {
                if id == self.taskbar.id {
                    self.taskbar.update(TaskbarMessage::Init)
                } else if id == self.desktop.id {
                    self.desktop.update(DesktopMessage::Init)
                } else if let Some(start_menu) = self.start_menu.as_mut() && id == start_menu.id {
                    start_menu.update(StartMessage::Init(self.app_image_cache.clone()))
                } else if let Some(panel_menu) = self.panel_menu.as_mut() && id == panel_menu.id {
                    panel_menu.update(PanelMessage::Init)
                } else if let Some(power_window) = self.power_window.as_mut() && id == power_window.id {
                    power_window.update(PowerMenuMessage::Init)
                } else {
                    Task::none()
                }
            }
            Message::WindowClosed(id) => {
                if id == self.desktop.id {
                    let (desktop,open_desktop) = Desktop::new();
                    self.desktop = desktop;
                    open_desktop.map(Message::WindowOpened)
                } else if id == self.taskbar.id {
                    let (taskbar,open_taskbar) = Taskbar::new();
                    self.taskbar = taskbar;
                    open_taskbar.map(Message::WindowOpened)
                } else if let Some(start_menu) = self.start_menu.as_ref() && id == start_menu.id {
                    self.start_menu = None;
                    Task::none()
                } else if let Some(panel_menu) = self.panel_menu.as_ref() && id == panel_menu.id {
                    self.panel_menu = None;
                    Task::none()
                } else if let Some(power_window) = self.power_window.as_ref() && id == power_window.id {
                    self.power_window = None;
                    Task::none()
                } else {
                    Task::none()
                }

            }
            Message::WindowClose(id) => {
                if let Some(start_menu) = self.start_menu.as_ref() && id == start_menu.id {
                    window::close(start_menu.id).map(Message::WindowClosed)
                } else {
                    Task::none()
                }
            }
            Message::WindowLostFocus(id) => {
                if let Some(start_menu) = self.start_menu.as_ref() && id == start_menu.id {
                    window::close(start_menu.id).map(Message::WindowClosed)
                } else if let Some(panel_menu) = self.panel_menu.as_ref() && id == panel_menu.id {
                    window::close(panel_menu.id).map(Message::WindowClosed)
                } else if let Some(power_window) = self.power_window.as_mut() && id == power_window.id {
                    power_window.update(PowerMenuMessage::Cancel)
                } else {
                    Task::none()
                }
            }
            Message::WindowCaughtFocus(id) => {
                if id == self.desktop.id {
                    self.desktop.update(DesktopMessage::KeepAtBottom)
                } else {
                    Task::none()
                }
            }
            Message::Taskbar(message) => self.taskbar.update(message),
            Message::Desktop(message) => self.desktop.update(message),
            Message::OpenStartMenu => {
                let (start_menu,open_start_menu) = StartMenu::new();
                self.start_menu = Some(start_menu);
                open_start_menu.map(Message::WindowOpened)
            }
            Message::OpenPanelMenu => {
                let (panel_menu,open_panel_menu) = PanelMenu::new();
                self.panel_menu = Some(panel_menu);
                open_panel_menu.map(Message::WindowOpened)
            },
            Message::OpenPowerWindow(option) => {
                let (power_window,open_power_window) = PowerWindow::new(option);
                self.power_window = Some(power_window);
                open_power_window.map(Message::WindowOpened)
            }
            Message::StartMenu(message) => {
                if let Some(start_menu) = self.start_menu.as_mut() {
                    start_menu.update(message)
                } else {
                    Task::none()
                }
            },
            Message::PanelMenu(message) => {
                if let Some(panel_menu) = self.panel_menu.as_mut() {
                    panel_menu.update(message)
                } else {
                    Task::none()
                }
            },
            Message::PowerMenu(message) => {
                if let Some(power_window) = self.power_window.as_mut() {
                    power_window.update(message)
                } else {
                    Task::none()
                }
            }
            Message::VolumeChange(value) => {
                set_sound_state(value, self.volume_muted);
                Task::none()
            }
            Message::VolumeMute => {
                set_sound_state(self.system_volume, !self.volume_muted);
                Task::none()
            }
            Message::Tick(_) => {
                self.battery = match get_battery_info() {
                    Ok(data) => {
                        Some(data)
                    }
                    Err(e) => {
                        eprintln!("Error getting battery info: {}", e);
                        None
                    }
                };
                self.wifi_status = get_wifi_status();
                (self.system_volume, self.volume_muted) = match get_sound_state() {
                    Ok(data) => {
                        data
                    }
                    Err(e) => {
                        eprintln!("Error getting volume data: {}", e);
                        (0.0, false)
                    }
                };
                let power_task = if let Some(power_window) = self.power_window.as_mut() {
                    power_window.update(PowerMenuMessage::Tick)
                } else {Task::none()};
                let taskbar_task = self.taskbar.update(TaskbarMessage::Tick);
                Task::batch(vec![power_task,taskbar_task])
            }
            _ => Task::none()
        }
    }
    pub fn view(&self, window_id: window::Id) -> iced::Element<'_, Message> {
        if window_id == self.taskbar.id {
            let start_state = self.start_menu.is_some();
            let panel_state = self.panel_menu.is_some();
            self.taskbar.view(
                self.icon_cache.clone(),
                start_state,
                panel_state,
                self.base_size,
                self.battery,
                self.wifi_status.clone(),
                self.system_volume,
                self.volume_muted)
        } else if window_id == self.desktop.id {
            self.desktop.view()
        } else if let Some(start_menu) = self.start_menu.as_ref() && window_id == start_menu.id {
            start_menu.view(self.icon_cache.clone(),self.app_image_cache.clone(),self.base_size)
        } else if let Some(panel) = self.panel_menu.as_ref() && window_id == panel.id {
            panel.view(self.icon_cache.clone(),self.base_size,self.battery,self.wifi_status.clone(),self.system_volume,self.volume_muted)
        } else if let Some(power_window) = self.power_window.as_ref() && window_id == power_window.id {
            power_window.view(self.icon_cache.clone())
        } else {
            column![].into()
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions:Vec<Subscription<Message>> = Vec::new();
        subscriptions.push(
            window::events().map(|(id, event)| {
                match event {
                    window::Event::Closed => Message::WindowClosed(id),
                    window::Event::Unfocused => Message::WindowLostFocus(id),
                    window::Event::Focused => Message::WindowCaughtFocus(id),
                    _ => Message::None
                }
            })
        );
        subscriptions.push(
            time::every(milliseconds(100)).map(Message::Tick)
        );
        Subscription::batch(subscriptions)
    }
    pub fn title(&self, window_id: window::Id) -> String {
        if window_id == self.taskbar.id {
            "FrostWin Taskbar".to_string()
        } else if window_id == self.desktop.id {
            "FrostWin Desktop".to_string()
        } else if let Some(start_menu) = self.start_menu.as_ref() && window_id == start_menu.id {
            "FrostWin StartMenu".to_string()
        } else if let Some(panel) = self.panel_menu.as_ref() && window_id == panel.id {
            "FrostWin PanelMenu".to_string()
        } else {
            "FrostWin Unknown".to_string()
        }
    }
    pub fn theme(&self, _: window::Id) -> iced::Theme {
        iced::Theme::CatppuccinFrappe
    }
    pub fn scale_factor(&self, _: window::Id) -> f32 {
        1.0
    }
}
