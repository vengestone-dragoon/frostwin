use std::any::Any;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};
use iced::{window, Alignment, Element, Length, Pixels, Point, Size, Task};
use iced::widget::{canvas, row, text, column, slider, button, space};
use iced::widget::text::Wrapping;
use crate::icons::{BatteryIcon, FrostwinIcons, VolumeIcon, WifiIcon};
use crate::Message;
use crate::styles::{my_slider, transparent_button};
use crate::sys_util::WifiStatus;

#[derive(Debug, Clone)]
pub enum PanelMessage {
    Init,
    Resize(Option<Size>),
}
pub struct PanelMenu {
    pub id: window::Id
}
impl PanelMenu {
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
        settings.level = window::Level::AlwaysOnTop;
        settings.position = window::Position::Specific(Point::new(1118.0,37.0));
        settings.size = Size::new(400.0,600.0);
        let (id,open_task) = window::open(settings);
        (Self {
            id
        },
         open_task)
    }
    pub fn update(&mut self, message: PanelMessage) -> Task<Message> {
        match message {
            PanelMessage::Init => {
                window::monitor_size(self.id).map(|size|Message::PanelMenu(PanelMessage::Resize(size)))
            }
            PanelMessage::Resize(resize) => {
                if let Some(size) = resize {
                    let margins = size.width * 0.0005;
                    let w_resize = window::resize(self.id, Size::new(600.0 * margins, 400.0 * margins) );
                    let w_move = window::move_to(self.id,Point::new(size.width - (margins * 604.0), margins * 50.0));
                    Task::batch([w_resize, w_move])
                } else {
                    Task::none()
                }
            }
        }
    }
    pub fn view(&self, icon_cache: Arc<Mutex<BTreeMap<FrostwinIcons,Box<dyn Any>>>>,base_size: f32,battery: Option<(f32,bool)>,wifi_status: WifiStatus, system_volume: f32, volume_muted: bool) -> Element<'_, Message> {
        let spacing = base_size * 2.0;
        let text_height = 30.0 * base_size;
        let battery_icon: Element<Message> = if let Some((battery_level,charging)) = battery {
            column![
                canvas(BatteryIcon {id:"Panel".to_string(), cache: icon_cache.clone(), charging,level: battery_level})
                            .width(Length::Fixed(36.0 * base_size))
                            .height(Length::Fixed(36.0 * base_size)),
                text!("{}%",battery_level.round())
            ].into()
        } else {
            space().height(Length::Fixed(0.0)).into()
        };
        row![
            column![
                row![
                    button(
                    canvas(VolumeIcon {id:"Panel".to_string(), cache: icon_cache.clone(), volume: system_volume, muted: volume_muted}).width(Length::Fill).height(Length::Fill),
                    ).width(Length::Fixed(36.0 * base_size))
                    .height(Length::Fixed(36.0 * base_size))
                    .style(transparent_button)
                    .padding(0.0)
                    .on_press(Message::VolumeMute),
                    slider(
                        RangeInclusive::new(0.0, 1.0),
                        system_volume,
                        |value|
                        Message::VolumeChange(value)
                    ).width(Length::Fill).step(0.01)
                    .height(Pixels(36.0 * base_size))
                    .style(move |theme,status| my_slider(theme,status)),
                    text!("{}%", (system_volume * 100.0).round()).width(Length::Fixed(text_height * 2.0)),
                ].align_y(Alignment::Center).spacing(spacing),
            ].width(Length::FillPortion(4)).height(Length::Fill).spacing(spacing),
            column![
                battery_icon,
                canvas(WifiIcon {id:"Panel".to_string(), cache: icon_cache.clone(), status: wifi_status.clone()})
                            .width(Length::Fixed(36.0 * base_size))
                            .height(Length::Fixed(36.0 * base_size)),
                text!("{}", match wifi_status.clone() {
                    WifiStatus::Connected(ssid,_) => {
                        ssid.to_string()
                    }
                    WifiStatus::Ethernet => {"Ethernet".to_string()}
                    WifiStatus::Disconnected => {"Not Connected".to_string()}
                }).wrapping(Wrapping::WordOrGlyph).align_x(Alignment::Center)
            ].width(Length::FillPortion(1)).height(Length::Fill).align_x(Alignment::Center).spacing(spacing),
        ].height(Length::Fill).width(Length::Fill).padding(spacing).spacing(spacing).into()
    }
}