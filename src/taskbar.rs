use crate::styles::transparent_button;
use crate::sys_util::WifiStatus;
use crate::Message;
use base64::Engine;
use chrono::offset::Local;
use iced::widget::image::Allocation;
use iced::widget::{button, column, container, image, row, space, text, tooltip, Button, Column, Row};
use iced::{window, Alignment, Element, Length, Padding, Point, Size, Task};
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use iced::advanced::image::Handle;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::Shell::{SHAppBarMessage, ABE_TOP, ABM_NEW, ABM_QUERYPOS, ABM_SETPOS, APPBARDATA};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, GetWindowPlacement, SetForegroundWindow, SetWindowPos, ShowWindow, HWND_NOTOPMOST, SM_CXSCREEN, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SW_RESTORE, SW_SHOWMINIMIZED, WINDOWPLACEMENT};
use x_win::{get_open_windows, get_window_icon, WindowInfo};
use crate::raw_icons::{battery_icon, sound_icon, start_icon, wifi_icon};

#[derive(Debug, Clone)]
pub enum TaskbarMessage {
    Init,
    Resize(Option<Size>),
    FocusWindow(u32),
    Tick,
    Allocate(u32,(Option<Allocation>,WindowInfo)),
    None
}
pub struct Taskbar {
    pub id: window::Id,
    tasks: BTreeMap<u32,(Option<Allocation>,WindowInfo)>,
}
impl Taskbar {
    pub fn new() -> (Self,Task<window::Id>) {
        let mut settings = window::Settings::default();
        settings.decorations = false;
        settings.resizable = false;
        settings.min_size = None;
        settings.max_size = None;
        settings.icon = None;
        settings.transparent = true;
        settings.closeable = false;
        settings.minimizable = false;
        settings.level = window::Level::Normal;
        settings.size = Size::new(0.0,0.0);
        settings.position = window::Position::Specific(Point::new(0.0,0.0));
        let (id,open_task) = window::open(settings);
        (Self {
            id,
            tasks: BTreeMap::new(),
        },
        open_task)
    }
    pub fn update(&mut self, message: TaskbarMessage) -> Task<Message> {
        match message {
            TaskbarMessage::Init => {
                Task::batch([
                    window::run(self.id, |window| {
                        match window.window_handle() {
                            Ok(window_handle) => {
                                let raw_handle = window_handle.as_raw();
                                match raw_handle {
                                    window::raw_window_handle::RawWindowHandle::Win32(handle) => {
                                        let win_handle:HWND = HWND{ 0: handle.hwnd.get() as *mut c_void };
                                        let mut abd = APPBARDATA {
                                            cbSize: size_of::<APPBARDATA>() as u32,
                                            hWnd: win_handle,
                                            uCallbackMessage: 0, // Define a custom message ID if you want callbacks
                                            uEdge: ABE_TOP, // Dock to top
                                            ..Default::default()
                                        };
                                        unsafe {
                                            SHAppBarMessage(ABM_NEW, &mut abd);

                                            // 2. Query for position
                                            // Define the ideal coordinates (entire width of screen, specific height)
                                            let screen_width = GetSystemMetrics(SM_CXSCREEN);
                                            let base_size = screen_width as f32 * 0.0005;
                                            abd.rc = RECT {
                                                left: 0,
                                                top: 0,
                                                right: screen_width,
                                                bottom: (50.0 * base_size) as i32,
                                            };

                                            // Ask the system if this space is available
                                            SHAppBarMessage(ABM_QUERYPOS, &mut abd);

                                            // 3. Set the position
                                            // After QUERYPOS, the system might have modified abd.rc to fit.
                                            // Now we tell the system we are officially claiming it.
                                            SHAppBarMessage(ABM_SETPOS, &mut abd);

                                            match SetWindowPos(
                                                win_handle,
                                                Some(HWND_NOTOPMOST),
                                                0, 0, 0, 0,
                                                SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE
                                            ) {
                                                Err(e) => {
                                                    eprintln!("Error setting taskbar position at system level: {}", e);
                                                }
                                                _ => {}
                                            };
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Err(e) => {
                                eprintln!("Error getting window handle for taskbar: {}", e);
                            }
                        }
                        Message::None
                    }),
                    window::monitor_size(self.id).map(|size|Message::Taskbar(TaskbarMessage::Resize(size)))
                ])
            }
            TaskbarMessage::Resize(resize) => {
                if let Some(size) = resize {
                    let base_size = size.width * 0.0005;
                    let window_margin = 4.0 * base_size;
                    let w_resize = window::resize(self.id, Size::new(size.width - (8.0 * base_size), 42.0 * base_size) );
                    let w_move = window::move_to(self.id,Point::new(window_margin, window_margin));
                    Task::batch([w_resize, w_move])
                } else {
                    Task::none()
                }
            }
            TaskbarMessage::FocusWindow(window_id) => {
                unsafe {
                    let win_handle = HWND(window_id as isize as *mut c_void);

                    // 1. Prepare the WINDOWPLACEMENT struct
                    let mut placement = WINDOWPLACEMENT::default();
                    placement.length = size_of::<WINDOWPLACEMENT>() as u32;

                    // 2. Check the current state
                    if GetWindowPlacement(win_handle, &mut placement).is_ok() {
                        if placement.showCmd == SW_SHOWMINIMIZED.0 as u32 {
                            // 3. If minimized, restore it
                            let _ = ShowWindow(win_handle, SW_RESTORE);
                        }
                    }

                    // 4. Bring to the foreground
                    let _ = SetForegroundWindow(win_handle);
                }
                Task::none()
            }
            TaskbarMessage::Tick => {
                let old_windows = self.tasks.clone();
                self.tasks.clear();
                let mut tasks: Vec<Task<Message>> = Vec::new();
                match get_open_windows() {
                    Ok(user_windows) => {
                        for window in user_windows {
                            if old_windows.contains_key(&window.id) {
                                self.tasks.insert(window.id,old_windows[&window.id].clone());
                            } else {
                                match get_window_icon(&window.clone()) {
                                    Ok(window_icon) => {
                                        let base64_data = window_icon.data.split(',').nth(1).unwrap_or("");
                                        match base64::engine::general_purpose::STANDARD.decode(base64_data) {
                                            Ok(bytes) => {
                                                println!("Successfully decoded {} bytes", bytes.len());
                                                let image_handle = image::Handle::from_bytes(bytes);
                                                tasks.push(image::allocate(image_handle).map(move |result|{
                                                    match result {
                                                        Ok(allocation) => Message::Taskbar(TaskbarMessage::Allocate(window.id,(Some(allocation),window.clone()))),
                                                        Err(e) => {
                                                            println!("Error: image alloc: {}",e);
                                                            Message::Taskbar(TaskbarMessage::Allocate(window.id,(None,window.clone())))
                                                        }
                                                    }
                                                }))
                                            }
                                            Err(e) => {
                                                eprintln!("Error decoding base64: {}",e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error getting window icon: {}", e);
                                    }
                                }
                            }
                        };
                    }
                    Err(e) => {
                        eprintln!("Error getting open windows: {:?}", e);
                    }
                }

                if tasks.is_empty() {
                    Task::none()
                } else {
                    Task::batch(tasks)
                }
            },
            TaskbarMessage::Allocate(window_id, data) => {
                self.tasks.insert(window_id,data);
                Task::none()
            }
            _ => Task::none()
        }
    }
    pub fn view(&self, app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>, start_state: bool, panel_state: bool,base_size: f32,battery: Option<(f32,bool)>,wifi_status: WifiStatus,system_volume: f32,volume_muted: bool) -> Element<'_, Message> {
        let text_half_height = 15.0 * base_size;
        let spacing = 2.0 * base_size;
        let clock: Column<Message> =
            column![
                    text!("{}",Local::now().format("%I:%M %p")).height(Length::FillPortion(1)).size(text_half_height).center(),
                    text!("{}",Local::now().format("%m/%d/%Y")).height(Length::FillPortion(1)).size(text_half_height).center(),
                ].height(Length::Fill);
        let mut tasks: Row<Message> = Row::new();
        for (allocation, window) in self.tasks.values() {
            if window.info.exec_name != "frostwin" && window.title != "Task Switching"{let mut button_content = Row::new();
                if let Some(alloc) = allocation {
                    button_content = button_content.push(image(alloc.handle()).height(Length::Fixed(24.0 * base_size)).width(Length::Fixed(24.0 * base_size)));
                }
                tasks = tasks.push(
                    tooltip(
                        Button::new(
                            button_content,
                        )
                            .height(Length::Fill)
                            .width(Length::Fixed(42.0 * base_size))
                            .on_press(Message::Taskbar(TaskbarMessage::FocusWindow(window.id)))
                    ,
                        container(column![
                        text!("{}", window.title)
                    ]).style(container::rounded_box),
                        tooltip::Position::FollowCursor
                    )
                );
            }
        }
        let battery_icon: Element<Message> = if let Some((battery_level,charging)) = battery {
            
            tooltip(
                image(battery_icon(app_image_cache.clone(), charging, battery_level))
                    .width(Length::Fixed(24.0 * base_size))
                    .height(Length::Fixed(24.0 * base_size)),
                container(column![
                        text!("{}%", battery_level.round()),
                    ]).style(container::rounded_box),
                tooltip::Position::FollowCursor
            ).into()
        } else {
            space().width(Length::Fixed(0.0)).into()
        };

        row![
            button(
                container(
                    image(start_icon(app_image_cache.clone(), start_state))
                        .width(Length::Fixed(24.0 * base_size))
                        .height(Length::Fixed(24.0 * base_size)),
                ).width(Length::Fill).height(Length::Fill).align_y(Alignment::Center).align_x(Alignment::Center)
            ).on_press(if start_state {Message::None} else {Message::OpenStartMenu})
                .height(Length::Fill)
                .width(Length::Fixed(42.0 * base_size))
                .padding(0.0)
                .style(transparent_button),
            tasks.spacing(spacing),
            space().width(Length::Fill),
            button(row![
                tooltip(
                    image(sound_icon(app_image_cache.clone(), system_volume, volume_muted))
                        .width(Length::Fixed(24.0 * base_size))
                        .height(Length::Fixed(24.0 * base_size)),
                    container(column![
                        text!("{}%", (system_volume * 100.0).round()),
                    ]).style(container::rounded_box),
                        tooltip::Position::FollowCursor
                ),
                tooltip(
                    image(wifi_icon(app_image_cache.clone(),wifi_status.clone()))
                    .width(Length::Fixed(24.0 * base_size))
                    .height(Length::Fixed(24.0 * base_size)),
                    container(column![
                        text!("{}", match wifi_status.clone() {
                            WifiStatus::Connected(ssid,_) => {
                                ssid.to_string()
                            }
                            WifiStatus::Ethernet => {"Ethernet".to_string()}
                            WifiStatus::Disconnected => {"Not Connected".to_string()}
                        }),
                    ]).style(container::rounded_box),
                        tooltip::Position::FollowCursor
                ),
                battery_icon,
                clock.padding(Padding::from([0.0,spacing]))
            ].align_y(Alignment::Center))
                .on_press(if panel_state {Message::None} else {Message::OpenPanelMenu})
                .style(transparent_button),
            space().width(Length::Fixed(spacing)),
        ].spacing(spacing)
            .width(Length::Fill)
            .align_y(Alignment::Center)
            .into()
    }
}