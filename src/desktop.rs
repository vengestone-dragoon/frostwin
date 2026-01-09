use crate::Message;
use iced::widget::row;
use iced::{window, Element, Task};
use std::ffi::c_void;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, WS_EX_NOACTIVATE};

#[derive(Debug, Clone)]
pub enum DesktopMessage {
    Init,
    KeepAtBottom,
}
pub struct Desktop {
    pub id: window::Id
}
impl Desktop {
    pub fn new() -> (Self,Task<window::Id>) {
        let mut settings = window::Settings::default();
        settings.decorations = false;
        settings.resizable = false;
        settings.min_size = None;
        settings.max_size = None;
        settings.icon = None;
        settings.transparent = false;
        settings.closeable = false;
        settings.minimizable = false;
        settings.fullscreen = true;
        settings.level = window::Level::AlwaysOnBottom;
        let (id,open_task) = window::open(settings);
        (Self {
            id
        },
         open_task)
    }
    pub fn update(&mut self, message: DesktopMessage) -> Task<Message> {
        match message {
            DesktopMessage::Init => {
                window::run(self.id, |window| {
                    match window.window_handle() {
                        Ok(window_handle) => {
                            let raw_handle = window_handle.as_raw();
                            match raw_handle {
                                window::raw_window_handle::RawWindowHandle::Win32(handle) => {
                                    let win_handle:HWND = HWND{ 0: handle.hwnd.get() as *mut c_void };

                                    unsafe {
                                        let current_style = GetWindowLongPtrW(win_handle, GWL_EXSTYLE) as u32;
                                        let new_style = current_style | WS_EX_NOACTIVATE.0;
                                        SetWindowLongPtrW(win_handle, GWL_EXSTYLE, new_style as isize);
                                        match SetWindowPos(
                                            win_handle,
                                            Some(HWND_BOTTOM),
                                            0, 0, 0, 0,
                                            SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE
                                        ) {
                                            Err(e) => {
                                                eprintln!("Error setting desktop position at system level: {}", e);
                                            }
                                            _ => {}
                                        };
                                    }

                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            eprintln!("Error getting desktop window handle: {}", e);
                        }
                    }
                    Message::None
                })
            }
            DesktopMessage::KeepAtBottom => {
                window::run(self.id, |window| {
                    match window.window_handle() {
                        Ok(window_handle) => {
                            let raw_handle = window_handle.as_raw();
                            match raw_handle {
                                window::raw_window_handle::RawWindowHandle::Win32(handle) => {
                                    let win_handle:HWND = HWND{ 0: handle.hwnd.get() as *mut c_void };
                                    unsafe {
                                        match SetWindowPos(
                                            win_handle,
                                            Some(HWND_BOTTOM),
                                            0, 0, 0, 0,
                                            SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE
                                        ) {
                                            Err(e) => {
                                                eprintln!("Error setting desktop position at system level: {}", e);
                                            }
                                            _ => {}
                                        };
                                    }
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            eprintln!("Error getting desktop window handle: {}", e);
                        }
                    }
                    Message::None
                })
            }
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        row![].into()
    }
}