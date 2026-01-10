use crate::power_window::PowerOptions;
use crate::styles::{colored_button, context_menu_button, transparent_button, window_style};
use crate::windows_icons::get_lnk_icon;
use crate::Message;
use dirs::data_dir;
use iced::advanced::text::Wrapping;
use iced::widget::image::Handle;
use iced::widget::{button, column, container, image, row, rule, scrollable, space, text, Button, Column, Grid, Text};
use iced::{window, Alignment, Color, ContentFit, Element, Length, Padding, Point, Size, Task};
use iced_aw::context_menu::ContextMenu;
use serde_json::{from_str, to_string_pretty};
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

fn get_dir_contents(path: PathBuf, target: &mut BTreeMap<String, StartItem>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(path)? {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    let is_ini = path.extension()
                        .map(|ext| ext == "ini")
                        .unwrap_or(false);

                    if is_ini {
                        continue;
                    }
                }

                match StartItem::new(path) {
                    Ok(start_item) => {
                        target.insert(start_item.name.clone(), start_item);
                    }
                    Err(e) => {
                        eprintln!("Error getting start item: {}", e);
                    }
                };
            }
            Err(e) => {
                eprintln!("Error getting dir entry: {}", e);
            }
        }
    }

    Ok(())
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
                match data_dir() {
                    Some(data_dir) => {
                        let user_programs_path = data_dir.join(r"Microsoft\Windows\Start Menu\Programs\");
                        let start_settings_file = data_dir.join(r"Frostwin\Start_Settings.json");
                        match std::fs::read_to_string(start_settings_file.clone()) {
                            Ok(content) => {
                                match from_str::<StartMenuSettings>(&content) {
                                    Ok(settings) => {
                                        self.settings = settings;
                                    }
                                    Err(e) => {
                                        println!("Error loading start menu settings: {:?}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error opening start menu settings: {:?}", e);
                                match e.kind() {
                                    std::io::ErrorKind::NotFound => {
                                        // Ensure the folder exists before creating the file
                                        if let Some(parent) = start_settings_file.parent() {
                                            let _ = std::fs::create_dir_all(parent);
                                        }
                                        let _ = std::fs::write(&start_settings_file, "{}");
                                    }

                                    std::io::ErrorKind::PermissionDenied => {
                                        eprintln!("Permission denied. Attempting to fix file attributes...");
                                        if let Ok(metadata) = std::fs::metadata(&start_settings_file) {
                                            let mut perms = metadata.permissions();
                                            if perms.readonly() {
                                                perms.set_readonly(false);
                                                // If we can fix permissions, try to write a fresh file
                                                if std::fs::set_permissions(&start_settings_file, perms).is_ok() {
                                                    let _ = std::fs::write(&start_settings_file, "{}");
                                                }
                                            }
                                        }
                                    }

                                    _ => eprintln!("Critical I/O error: {}", e),
                                }
                            }
                        }
                        match get_dir_contents(user_programs_path, &mut content) {
                            Err(e) => {
                                eprintln!("Error loading user start programs: {:?}", e);
                            }
                            _ => {}
                        };
                        match get_dir_contents(system_programs_path, &mut content) {
                            Err(e) => {
                                eprintln!("Error loading system start programs: {:?}", e);
                            }
                            _ => {}
                        };
                    }
                    None => {}
                }

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
                    StartItemMessage::Toggle(mut path) => {
                        if let Some(sub_dir) = path.pop() {
                            if let Some(item) = self.content.get_mut(&sub_dir) {
                                item.update(StartItemMessage::Toggle(path));
                            } else {
                                eprintln!("Error: Directory key '{}' not found in content.", sub_dir);
                            }
                        }
                        Task::none()
                    }

                    StartItemMessage::Launch(path) => {
                        match Command::new("explorer").args([&path]).spawn() {
                            Ok(_) => {
                                Task::done(Message::WindowClose(self.id))
                            }
                            Err(e) => {
                                eprintln!("Failed to launch explorer for path {}: {}", path.to_string_lossy(), e);
                                Task::none()
                            }
                        }
                    }
                }
            },
            StartMessage::SwitchToTab(tab) => {
                self.tab = tab;
                Task::none()
            },
            StartMessage::PinToTiles(path) => {
                match data_dir() {
                    Some(data_dir) => {
                        let start_settings_file = data_dir.join(r"Frostwin\Start_Settings.json");
                        self.settings.tiles.push(path);
                        match to_string_pretty(&self.settings) {
                            Ok(settings) => {
                                match std::fs::write(&start_settings_file, settings) {
                                    Err(e) => {
                                        eprintln!("Error writing start menu settings: {:?}", e);
                                    }
                                    _ => {}
                                };
                            }
                            Err(e) => {
                                eprintln!("Error serializing start menu settings: {:?}", e);
                            }
                        };
                    }
                    None => {}
                }
                Task::none()
            }
            StartMessage::UnpinFromTiles(path) => {
                match data_dir() {
                    Some(data_dir) => {
                        let start_settings_file = data_dir.join(r"Frostwin\Start_Settings.json");
                        let mut new_tiles_list: Vec<PathBuf> = Vec::new();
                        for old_path in self.settings.tiles.iter() {
                            if *old_path != path {
                                new_tiles_list.push(old_path.clone());
                            }
                        }
                        self.settings.tiles = new_tiles_list;
                        match to_string_pretty(&self.settings) {
                            Ok(settings) => {
                                match std::fs::write(&start_settings_file, settings) {
                                    Err(e) => {
                                        eprintln!("Error writing start menu settings: {:?}", e);
                                    }
                                    _ => {}
                                };
                            }
                            Err(e) => {
                                eprintln!("Error serializing start menu settings: {:?}", e);
                            }
                        };
                    }
                    None => {}
                }
                Task::none()
            }
        }
    }
    pub fn view(&self, app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>,base_size: f32) -> Element<'_, Message> {
        let text_height = 30.0 * base_size;
        let spacing = 2.0 * base_size;
        let mut tab_content: Column<Message> = Column::new();
        let header: Text;
        let (
            lock_button,
            restart_button,
            shutdown_button,
            empty_app,
        ) = match (app_image_cache.lock(),data_dir()) {
            (Ok(app_image_lock),Some(data_dir)) => {
                let data_folder = data_dir.join("Frostwin");
                let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
                (
                    app_image_lock.get(&data_folder.join("icons/power/Lock.png")).unwrap_or(&error_handle).clone(),
                    app_image_lock.get(&data_folder.join("icons/power/Restart.png")).unwrap_or(&error_handle).clone(),
                    app_image_lock.get(&data_folder.join("icons/power/Shutdown.png")).unwrap_or(&error_handle).clone(),
                    app_image_lock.get(&data_folder.join("icons/EmptyApp.png")).unwrap_or(&error_handle).clone(),
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
                )
            }
        };
        match self.tab {
            StartMenuTab::Tiles => {
                header = text!("Tiles").size(text_height * 1.3);
                let mut tiles_grid: Grid<Message> = Grid::new();
                for path in self.settings.tiles.iter() {
                    match app_image_cache.lock() {
                        Ok(app_image_lock) => {
                            let icon: Element<'_,Message> = if app_image_lock.contains_key(path) {
                                if let Some(app_image) = app_image_lock.get(path) {
                                    image(app_image).height(Length::Fixed(text_height * 2.0)).width(Length::Fixed(text_height * 2.0)).content_fit(ContentFit::Fill).into()
                                } else {
                                    eprintln!("Error getting app_image");
                                    image(empty_app.clone()).into()
                                }
                            } else {
                                image(empty_app.clone()).into()
                            };
                            let name = if let Some(app_name) = path.file_stem() {
                                if let Some(app_name_str) = app_name.to_str() {
                                    app_name_str.to_string()
                                } else {
                                    eprintln!("Error: failed to get App name into str");
                                    "".to_string()
                                }
                            } else {
                                eprintln!("Error: failed to get App name from OsStr");
                                "".to_string()
                            };
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
                        Err(e) => {
                            eprintln!("Error getting app_image lock: {:?}", e);
                        }
                    }
                }
                tab_content = tab_content.push(tiles_grid.spacing(spacing));
            },
            StartMenuTab::Applications => {
                header = text!("Applications").size(text_height * 1.3);
                for key in self.sorted.iter() {
                    let mut path: Vec<String> = Vec::new();
                    path.push(key.clone());
                    if let Some(item) = self.content.get(key) {
                        tab_content = tab_content.push(item.view(app_image_cache.clone(),base_size.clone(), path))
                    }
                }
            }
        };
        container(
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
                        image(lock_button)
                    ).height(Length::Fixed(text_height))
                    .width(Length::Fixed(text_height))
                    .padding(0.0)
                    .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.2, 0.2, 0.7)))
                    .on_press(Message::OpenPowerWindow(PowerOptions::Lock)),
                    button(
                        image(restart_button)
                    ).height(Length::Fixed(text_height))
                    .width(Length::Fixed(text_height))
                    .padding(0.0)
                    .style(|theme, status| colored_button(theme, status, Color::from_rgb(0.2, 0.7, 0.2)))
                    .on_press(Message::OpenPowerWindow(PowerOptions::Reboot)),
                    button(
                        image(shutdown_button)
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
            ]
        ).style(window_style).into()
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
    pub fn new(path: PathBuf) -> Result<Self,String> {
        if let Some(file_name) = path.file_name() && let Some(name) = file_name.to_str() {
            let mut name = name.to_string();
            let mut content: Option<BTreeMap<String,Self>> = None;
            if path.is_dir() {
                let mut new_content: BTreeMap<String,Self> = BTreeMap::new();
                match get_dir_contents(path.clone(),&mut new_content) {
                    Err(e) => {
                        return Err(format!("Error getting directory contents: {}", e));
                    }
                    _ => {}
                };
                content = Some(new_content);
            } else if path.is_file() && let Some(extension) = path.extension() && extension == "lnk" {
                if let Some(name_osstr) = path.file_stem() && let Some(name_str) = name_osstr.to_str() {
                    name = name_str.to_string();
                }
            }
            Ok(Self {
                name,
                content,
                sorted: None,
                path,
                open: false,
            })
        } else {
            Err(format!("Error getting name from path: {:?}", path))
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
            match app_image_cache.lock() {
                Ok(mut app_image_lock) => {
                    if !app_image_lock.contains_key(&self.path.clone()) {
                        if let Some((data,width,height)) = get_lnk_icon(self.path.clone()) {
                            let icon_handle = Handle::from_rgba(width, height, data);
                            app_image_lock.insert(self.path.clone(), icon_handle);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error accessing app_image_cache: {}", e);
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
                        if let Some(sub_dir) = path.pop() {
                            if let Some(item) = content.get_mut(&sub_dir) {
                                item.update(StartItemMessage::Toggle(path));
                            }
                        };
                    }
                }
            }
            _ => {}
        }
    }
    pub fn view(&self, app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>, base_size: f32, path: Vec<String>) -> Element<'_, Message> {
        let text_half_height = 15.0 * base_size;
        let spacing = 2.0 * base_size;
        let mut head: Column<Message> = Column::new();
        let (
            empty_app,
            folder,
            tree_dot,
        ) = match (app_image_cache.lock(),data_dir()) {
            (Ok(app_image_lock),Some(data_dir)) => {
                let data_folder = data_dir.join("Frostwin");
                let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
                (
                    app_image_lock.get(&data_folder.join("icons/EmptyApp.png")).unwrap_or(&error_handle).clone(),
                    app_image_lock.get(&data_folder.join("icons/Folder.png")).unwrap_or(&error_handle).clone(),
                    app_image_lock.get(&data_folder.join("icons/TreeDot.png")).unwrap_or(&error_handle).clone(),
                )
            }
            (Err(error),_) => {
                eprintln!("Error getting app image cache: {}", error);
                let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
                (
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
                )
            }
        };
        if let Some(content) = self.content.as_ref() && let Some(keys) = self.sorted.as_ref() {
            if !content.is_empty() && !keys.is_empty() {
                head = head.push(
                    button(
                        row![
                            image(tree_dot).height(text_half_height).width(text_half_height),
                            image(folder).height(text_half_height).width(text_half_height),
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
                        if let Some(item) = content.get(key) {
                            children = children.push(
                                item.view(app_image_cache.clone(), base_size.clone(), new_path)
                            );
                        }
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
        } else if let Some(extension) = self.path.extension() && extension != "ini" {
            let icon: Element<'_,Message> =
            match app_image_cache.lock() {
                Ok(app_image_lock) => {
                    if app_image_lock.contains_key(&self.path) {
                        if let Some(image) = app_image_lock.get(&self.path) {
                            iced::widget::image(image).height(text_half_height).width(text_half_height).content_fit(ContentFit::Fill).into()
                        } else {
                            image(empty_app).height(text_half_height).width(text_half_height).into()
                        }
                    } else {
                        image(empty_app).height(text_half_height).width(text_half_height).into()
                    }
                }
                Err(e) => {
                    eprintln!("Error getting app_image_cache: {}", e);
                    image(empty_app).height(text_half_height).width(text_half_height).into()
                }
            };
            let context_menu = ContextMenu::new(
                button(
                    row![
                        image(tree_dot).height(text_half_height).width(text_half_height),
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