use crate::icons::*;
use crate::power_window::PowerOptions;
use crate::styles::{colored_button, context_menu_button, transparent_button};
use crate::windows_icons::get_lnk_icon;
use crate::Message;
use dirs::data_dir;
use iced::advanced::text::Wrapping;
use iced::widget::image::Handle;
use iced::widget::{button, canvas, column, container, image, row, rule, scrollable, space, text, Button, Column, Grid, Text};
use iced::{window, Alignment, Color, ContentFit, Element, Length, Padding, Point, Size, Task};
use iced_aw::context_menu::ContextMenu;
use serde_json::{from_str, to_string_pretty};
use std::any::Any;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum StartMessage {
    Init(Arc<Mutex<BTreeMap<PathBuf,Handle>>>),
    Resize(Option<Size>),
    ItemMessage(StartItemMessage),
    SwitchToTab(StartMenuTab),
    PinToTiles(PathBuf),
    UnpinFromTiles(PathBuf),
}
#[derive(Debug, Clone)]
pub enum StartMenuTab {
    Tiles,
    Applications,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, Clone)]
struct StartMenuSettings {
    tiles: Vec<PathBuf>,
}
impl StartMenuSettings {
    fn new() -> StartMenuSettings {
        StartMenuSettings {
            tiles: Default::default(),
        }
    }
}

fn get_dir_contents(path: PathBuf, target: &mut BTreeMap<String, StartItem>) {
    for item in path.read_dir().unwrap() {
        if let Ok(item) = item {
            if item.path().is_file() {
                if item.path().extension().unwrap() != "ini" {
                    let new_item = StartItem::new(item.path());
                    let name = new_item.name.clone();
                    target.insert(name, new_item);
                }
            } else {
                let new_item = StartItem::new(item.path());
                let name = new_item.name.clone();
                target.insert(name, new_item);
            }
        }
    };
}

pub struct StartMenu {
    pub id: window::Id,
    content: BTreeMap<String, StartItem>,
    sorted: Vec<String>,
    tab: StartMenuTab,
    settings: StartMenuSettings,
}
impl StartMenu {
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
        settings.position = window::Position::Specific(Point::new(2.0,37.0));
        settings.size = Size::new(400.0,600.0);
        let (id,open_task) = window::open(settings);
        (Self {
            id,
            content: BTreeMap::new(),
            sorted: Vec::new(),
            tab: StartMenuTab::Tiles,
            settings: StartMenuSettings::new(),
        },
         open_task)
    }
    pub fn update(&mut self, message: StartMessage) -> Task<Message> {
        match message {
            StartMessage::Init(app_image_cache) => {
                let mut content: BTreeMap<String, StartItem> = BTreeMap::new();
                let system_programs_path = PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\");
                let user_programs_path = data_dir().unwrap().join(r"Microsoft\Windows\Start Menu\Programs\");
                let start_settings_file = data_dir().unwrap().join(r"Frostwin\Start_Settings.json");
                if !start_settings_file.exists() {
                    std::fs::write(&start_settings_file, "").unwrap();
                } else {
                    self.settings = from_str(std::fs::read_to_string(start_settings_file).unwrap().as_str()).unwrap();
                }

                get_dir_contents(user_programs_path, &mut content);
                get_dir_contents(system_programs_path, &mut content);
                self.content = content;
                let mut keys: Vec<String> = self.content.keys().cloned().collect();
                alphanumeric_sort::sort_str_slice(&mut keys);
                self.sorted = keys;
                for (_,entry) in self.content.iter_mut() {
                    entry.prep(app_image_cache.clone())
                }
                window::monitor_size(self.id).map(|size|Message::StartMenu(StartMessage::Resize(size)))
            }
            StartMessage::Resize(resize) => {
                if let Some(size) = resize {
                    let base_size = size.width * 0.0005;
                    let window_margin = 4.0 * base_size;
                    let w_resize = window::resize(self.id, Size::new(600.0 * base_size, 800.0 * base_size) );
                    let w_move = window::move_to(self.id,Point::new(window_margin, 50.0 * base_size));
                    Task::batch([w_resize, w_move])
                } else {
                    Task::none()
                }
            }
            StartMessage::ItemMessage(message) => {
                match message {
                    StartItemMessage::Toggle(path) => {
                        let mut path = path.clone();
                        if path.len() > 0 {
                            let sub_dir = path.pop().unwrap();
                            self.content.get_mut(&sub_dir).unwrap().update(StartItemMessage::Toggle(path));
                        };
                        Task::none()
                    }
                    StartItemMessage::Launch(path) => {
                        Command::new("explorer").args([path]).spawn().unwrap();
                        Task::done(Message::WindowClose(self.id))
                    }
                }

            },
            StartMessage::SwitchToTab(tab) => {
                self.tab = tab;
                Task::none()
            },
            StartMessage::PinToTiles(path) => {
                let start_settings_file = data_dir().unwrap().join(r"Frostwin\Start_Settings.json");
                self.settings.tiles.push(path);
                let settings = to_string_pretty(&self.settings).unwrap();
                std::fs::write(&start_settings_file, settings).unwrap();
                Task::none()
            }
            StartMessage::UnpinFromTiles(path) => {
                let start_settings_file = data_dir().unwrap().join(r"Frostwin\Start_Settings.json");
                let mut new_tiles_list: Vec<PathBuf> = Vec::new();
                for old_path in self.settings.tiles.iter() {
                    if *old_path != path {
                        new_tiles_list.push(old_path.clone());
                    }
                }
                self.settings.tiles = new_tiles_list;
                let settings = to_string_pretty(&self.settings).unwrap();
                std::fs::write(&start_settings_file, settings).unwrap();
                Task::none()
            }
        }
    }
    pub fn view(&self, icon_cache: Arc<Mutex<BTreeMap<FrostwinIcons,Box<dyn Any>>>>, app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>,base_size: f32) -> Element<'_, Message> {
        let text_height = 30.0 * base_size;
        let spacing = 2.0 * base_size;
        let mut tab_content: Column<Message> = Column::new();
        let header: Text;
        match self.tab {
            StartMenuTab::Tiles => {
                header = text!("Tiles").size(text_height * 1.3);
                let mut tiles_grid: Grid<Message> = Grid::new();
                for path in self.settings.tiles.iter() {
                    let app_image_lock = app_image_cache.lock().unwrap();
                    let icon: Element<'_,Message> = if app_image_lock.contains_key(path) {
                        image(app_image_lock.get(path).unwrap()).height(Length::Fixed(text_height * 2.0)).width(Length::Fixed(text_height * 2.0)).content_fit(ContentFit::Fill).into()
                    } else {
                        canvas(EmptyApp {id: format!("Tiles,{}",path.to_string_lossy()), cache: icon_cache.clone()}).height(Length::Fixed(text_height * 2.0)).width(Length::Fixed(text_height * 2.0)).into()
                    };
                    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                    let tile_button: Button<Message> = Button::new(
                        column![
                            icon,
                            text!("{}",name).size(text_height * 0.5).align_x(Alignment::Center).wrapping(Wrapping::WordOrGlyph)
                        ].align_x(Alignment::Center)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .spacing(0.0)
                            .padding(spacing)
                    ).style(transparent_button)
                    .on_press(Message::StartMenu(StartMessage::ItemMessage(StartItemMessage::Launch(path.clone()))));
                    let context_menu = ContextMenu::new(
                        tile_button,
                        || {
                            container(
                                column![
                                button(text!("Unpin")).style(context_menu_button).on_press(Message::StartMenu(StartMessage::UnpinFromTiles(path.clone()))),
                            ]
                            ).style(container::bordered_box).into()
                        }
                    );
                    tiles_grid = tiles_grid.push(context_menu);
                }
                tab_content = tab_content.push(tiles_grid.spacing(spacing));
            },
            StartMenuTab::Applications => {
                header = text!("Applications").size(text_height * 1.3);
                for key in self.sorted.iter() {
                    let mut path: Vec<String> = Vec::new();
                    path.push(key.clone());
                    tab_content = tab_content.push(self.content.get(key).unwrap().view(icon_cache.clone(),app_image_cache.clone(),base_size.clone(), path))
                }
            }
        };
        column![
            row![
                column![
                    button("Tiles").on_press(Message::StartMenu(StartMessage::SwitchToTab(StartMenuTab::Tiles)))
                    .style(transparent_button)
                    .width(Length::Fill),
                    button("Applications").on_press(Message::StartMenu(StartMessage::SwitchToTab(StartMenuTab::Applications)))
                    .style(transparent_button)
                    .width(Length::Fill),
                ].width(Length::FillPortion(3))
                .height(Length::Fill),
                rule::vertical(spacing),
                column![
                    header.align_y(Alignment::Center)
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::FillPortion(1)),
                    rule::horizontal(spacing),
                    scrollable(
                        tab_content.width(Length::Fill)
                        .padding(spacing)
                        .spacing(spacing)
                    ).width(Length::FillPortion(9))
                    .height(Length::FillPortion(9)),
                ].width(Length::FillPortion(9))
                .height(Length::Fill),
            ].width(Length::Fill)
            .height(Length::FillPortion(7)),
            rule::horizontal(spacing),
            row![
                space().width(Length::Fixed(text_height)),
                space().width(Length::Fill),
                button(
                    canvas(LockButton {id: "StartMenu".to_string(),cache: icon_cache.clone()})
                ).height(Length::Fixed(text_height))
                .width(Length::Fixed(text_height))
                .padding(0.0)
                .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.2, 0.2, 0.7)))
                .on_press(Message::OpenPowerWindow(PowerOptions::Lock)),
                button(
                    canvas(RestartButton {id: "StartMenu".to_string(),cache: icon_cache.clone()})
                ).height(Length::Fixed(text_height))
                .width(Length::Fixed(text_height))
                .padding(0.0)
                .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.2, 0.7, 0.2)))
                .on_press(Message::OpenPowerWindow(PowerOptions::Reboot)),
                button(
                    canvas(PowerButton {id: "StartMenu".to_string(),cache: icon_cache.clone()})
                ).height(Length::Fixed(text_height))
                .width(Length::Fixed(text_height))
                .padding(0.0)
                .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.7, 0.2, 0.2)))
                .on_press(Message::OpenPowerWindow(PowerOptions::Shutdown)),
                space().width(Length::Fixed(text_height)),
            ].width(Length::Fill)
            .height(Length::FillPortion(1))
            .align_y(Alignment::Center)
            .spacing(spacing)
            .padding(spacing),
        ].into()
    }
}

#[derive(Debug, Clone)]
pub enum StartItemMessage {
    Toggle(Vec<String>),
    Launch(PathBuf),
}

struct StartItem {
    name: String,
    content: Option<BTreeMap<String,Self>>,
    sorted: Option<Vec<String>>,
    path: PathBuf,
    open: bool,
}

impl StartItem {
    pub fn new(path: PathBuf) -> Self {
        let mut name = path.file_name().unwrap().to_str().unwrap().to_string();
        let mut content: Option<BTreeMap<String,Self>> = None;
        if path.is_dir() {
            let mut new_content: BTreeMap<String,Self> = BTreeMap::new();
            get_dir_contents(path.clone(),&mut new_content);
            content = Some(new_content);
        } else if path.is_file() && path.extension().unwrap() == "lnk" {
            name = path.file_stem().unwrap().to_str().unwrap().to_string();
        }
        Self {
            name,
            content,
            sorted: None,
            path,
            open: false,
        }
    }
    pub fn prep(&mut self, app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>) {
        if let Some(content) = self.content.as_mut() {
            let mut keys: Vec<String> = content.keys().cloned().collect();
            alphanumeric_sort::sort_str_slice(&mut keys);
            self.sorted = Some(keys);
            for (_, item) in content.iter_mut() {
                item.prep(app_image_cache.clone());
            };
        } else {
            let mut app_icons_lock = app_image_cache.lock().unwrap();
            if !app_icons_lock.contains_key(&self.path.clone()) {
                if let Some((data,width,height)) = get_lnk_icon(self.path.clone()) {
                    let icon_handle = Handle::from_rgba(width, height, data);
                    app_icons_lock.insert(self.path.clone(), icon_handle);
                }
            }
        }
    }
    pub fn update(&mut self, message: StartItemMessage) {
        match message {
            StartItemMessage::Toggle(path) => {
                let mut path = path.clone();
                if path.len() == 0 {
                    self.open = !self.open;
                } else if path.len() > 0 {
                    if let Some(content) = self.content.as_mut() {
                        let sub_dir = path.pop().unwrap();
                        content.get_mut(&sub_dir).unwrap().update(StartItemMessage::Toggle(path));
                    }
                }
            }
            _ => {}
        }
    }
    pub fn view(&self, icon_cache: Arc<Mutex<BTreeMap<FrostwinIcons,Box<dyn Any>>>>, app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>, base_size: f32, path: Vec<String>) -> Element<'_, Message> {
        let text_half_height = 15.0 * base_size;
        let spacing = 2.0 * base_size;
        let mut head: Column<Message> = Column::new();
        if let Some(content) = self.content.as_ref() && let Some(keys) = self.sorted.as_ref() {
            if !content.is_empty() && !keys.is_empty() {
                head = head.push(
                    button(
                        row![
                            canvas(TreeDot {id: path.join("/"), cache: icon_cache.clone()}).height(text_half_height).width(text_half_height),
                            canvas(FolderButton {id: path.join("/"), cache: icon_cache.clone()}).height(text_half_height).width(text_half_height),
                            space().width(Length::Fixed(spacing)),
                            text!("{}",self.name).size(text_half_height).height(Length::Fixed(text_half_height)).align_y(Alignment::Center),
                        ].align_y(Alignment::Center)
                    ).on_press(Message::StartMenu(StartMessage::ItemMessage(StartItemMessage::Toggle(path.clone()))))
                        .style(transparent_button),
                );
                if self.open {
                    let mut children: Column<Message> = Column::new();
                    for key in keys.iter() {
                        let mut new_path: Vec<String> = path.clone();
                        new_path.insert(0,key.clone());
                        children = children.push(
                            content.get(key).unwrap().view(icon_cache.clone(), app_image_cache.clone(), base_size.clone(), new_path)
                        );
                    };
                    head = head.push(
                        children.spacing(spacing).padding(Padding::from([0.0,text_half_height])),
                    );
                    head = head.push(
                        space().height(Length::Fixed(spacing * 2.0)),
                    );
                }
            }
            head.spacing(spacing).into()
        } else if self.path.extension().unwrap() != "ini" {
            let app_image_lock = app_image_cache.lock().unwrap();
            let icon: Element<'_,Message> = if app_image_lock.contains_key(&self.path) {
                image(app_image_lock.get(&self.path).unwrap()).height(text_half_height).width(text_half_height).content_fit(ContentFit::Fill).into()
            } else {
                canvas(EmptyApp {id: path.join("/"), cache: icon_cache.clone()}).height(text_half_height).width(text_half_height).into()
            };
            let context_menu = ContextMenu::new(
                button(
                    row![
                        canvas(TreeDot {id: path.join("/"), cache: icon_cache.clone()}).height(text_half_height).width(text_half_height),
                        icon,
                        space().width(Length::Fixed(spacing)),
                        text!("{}",self.name).size(text_half_height).height(Length::Fixed(text_half_height)).align_y(Alignment::Center)
                    ].align_y(Alignment::Center)
                ).on_press(Message::StartMenu(StartMessage::ItemMessage(StartItemMessage::Launch(self.path.clone()))))
                    .style(transparent_button),
                || {
                    container(
                        column![
                                button(text!("Pin to Tiles")).style(context_menu_button).on_press(Message::StartMenu(StartMessage::PinToTiles(self.path.clone())))
                            ]
                    ).style(container::bordered_box).into()
                }
            );
            head = head.push(
                context_menu
            );
            head.spacing(spacing).into()
        } else {
            head.into()
        }
    }
}