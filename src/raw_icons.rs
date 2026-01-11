use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use dirs::data_dir;
use iced::widget::image::Handle;
use crate::sys_util::WifiStatus;

pub const START_CLOSED_ICON: &'static [u8] = include_bytes!("images/icons/startIcon/StartIconClosed.png");
pub const START_OPEN_ICON: &'static [u8] = include_bytes!("images/icons/startIcon/StartIconOpen.png");

pub fn start_icon(app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>, open: bool) -> Handle {
    let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
    match (app_image_cache.lock(),data_dir()) {
        (Ok(app_image_cache),Some(data_dir)) => {
            let data_folder = data_dir.join("Frostwin");
            if open {
                app_image_cache.get(&data_folder.join("icons/startIcon/StartIconOpen.png")).unwrap_or(&error_handle).clone()
            } else {
                app_image_cache.get(&data_folder.join("icons/startIcon/StartIconClosed.png")).unwrap_or(&error_handle).clone()
            }
        }
        (Err(e),_) => {
            eprintln!("Error accessing app_image_cache: {}", e);
            error_handle
        }
        (_,None) => {
            eprintln!("Error getting data_dir");
            error_handle
        }
    }
}

pub const WIFI_ICON4_ICON: &'static [u8] = include_bytes!("images/icons/network/WifiIcon4.png");
pub const WIFI_ICON3_ICON: &'static [u8] = include_bytes!("images/icons/network/WifiIcon3.png");
pub const WIFI_ICON2_ICON: &'static [u8] = include_bytes!("images/icons/network/WifiIcon2.png");
pub const WIFI_ICON1_ICON: &'static [u8] = include_bytes!("images/icons/network/WifiIcon1.png");
pub const WIFI_ICON0_ICON: &'static [u8] = include_bytes!("images/icons/network/WifiIcon0.png");
pub const WIFI_NONE_ICON: &'static [u8] = include_bytes!("images/icons/network/WifiIconNone.png");
pub const ETHERNET_ICON: &'static [u8] = include_bytes!("images/icons/network/EthernetIcon.png");
pub const NO_INTERNET_ICON: &'static [u8] = include_bytes!("images/icons/network/NoInternet.png");

pub fn wifi_icon(app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>, wifi_status: WifiStatus) -> Handle {
    let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
    match (app_image_cache.lock(),data_dir()) {
        (Ok(app_image_cache),Some(data_dir)) => {
            let data_folder = data_dir.join("Frostwin");
            match wifi_status {
                WifiStatus::Disconnected => {
                    app_image_cache.get(&data_folder.join("icons/network/NoInternet.png")).unwrap_or(&error_handle).clone()
                }
                WifiStatus::Ethernet => {
                    app_image_cache.get(&data_folder.join("icons/network/EthernetIcon.png")).unwrap_or(&error_handle).clone()
                }
                WifiStatus::Connected(_, strength) => {
                    if strength >= 90 {
                        app_image_cache.get(&data_folder.join("icons/network/WifiIcon4.png")).unwrap_or(&error_handle).clone()
                    } else if strength >= 65 {
                        app_image_cache.get(&data_folder.join("icons/network/WifiIcon3.png")).unwrap_or(&error_handle).clone()
                    } else if strength >= 40 {
                        app_image_cache.get(&data_folder.join("icons/network/WifiIcon2.png")).unwrap_or(&error_handle).clone()
                    } else if strength >= 15 {
                        app_image_cache.get(&data_folder.join("icons/network/WifiIcon1.png")).unwrap_or(&error_handle).clone()
                    } else {
                        app_image_cache.get(&data_folder.join("icons/network/WifiIcon0.png")).unwrap_or(&error_handle).clone()
                    }
                }
            }
        }
        (Err(e),_) => {
            eprintln!("Error accessing app_image_cache: {}", e);
            error_handle
        }
        (_,None) => {
            eprintln!("Error getting data_dir");
            error_handle
        }
    }
}

pub const BATTERY100_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery100.png");
pub const BATTERY90_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery90.png");
pub const BATTERY80_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery80.png");
pub const BATTERY70_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery70.png");
pub const BATTERY60_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery60.png");
pub const BATTERY50_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery50.png");
pub const BATTERY40_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery40.png");
pub const BATTERY30_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery30.png");
pub const BATTERY20_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery20.png");
pub const BATTERY10_ICON: &'static [u8] = include_bytes!("images/icons/battery/Battery10.png");
pub const BATTERY_C100_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC100.png");
pub const BATTERY_C90_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC90.png");
pub const BATTERY_C80_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC80.png");
pub const BATTERY_C70_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC70.png");
pub const BATTERY_C60_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC60.png");
pub const BATTERY_C50_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC50.png");
pub const BATTERY_C40_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC40.png");
pub const BATTERY_C30_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC30.png");
pub const BATTERY_C20_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC20.png");
pub const BATTERY_C10_ICON: &'static [u8] = include_bytes!("images/icons/battery/BatteryC10.png");

pub fn battery_icon(app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>, charging: bool, battery_level: f32) -> Handle {
    let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
    match (app_image_cache.lock(),data_dir()) {
        (Ok(app_image_cache),Some(data_dir)) => {
            let data_folder = data_dir.join("Frostwin");
            if charging {
                if battery_level >= 95.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC100.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 85.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC90.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 75.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC80.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 65.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC70.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 55.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC60.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 45.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC50.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 35.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC40.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 25.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC30.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 15.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC20.png")).unwrap_or(&error_handle).clone()
                } else {
                    app_image_cache.get(&data_folder.join("icons/battery/BatteryC10.png")).unwrap_or(&error_handle).clone()
                }
            } else {
                if battery_level >= 95.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery100.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 85.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery90.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 75.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery80.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 65.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery70.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 55.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery60.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 45.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery50.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 35.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery40.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 25.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery30.png")).unwrap_or(&error_handle).clone()
                } else if battery_level >= 15.0 {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery20.png")).unwrap_or(&error_handle).clone()
                } else {
                    app_image_cache.get(&data_folder.join("icons/battery/Battery10.png")).unwrap_or(&error_handle).clone()
                }
            }
        }
        (Err(e),_) => {
            eprintln!("Error accessing app_image_cache: {}", e);
            error_handle
        }
        (_,None) => {
            eprintln!("Error getting data_dir");
            error_handle
        }
    }
}

pub const SOUND0_ICON: &'static [u8] = include_bytes!("images/icons/sound/Sound0.png");
pub const SOUND1_ICON: &'static [u8] = include_bytes!("images/icons/sound/Sound1.png");
pub const SOUND2_ICON: &'static [u8] = include_bytes!("images/icons/sound/Sound2.png");
pub const SOUND3_ICON: &'static [u8] = include_bytes!("images/icons/sound/Sound3.png");
pub const SOUND4_ICON: &'static [u8] = include_bytes!("images/icons/sound/Sound4.png");
pub const SOUND_M_ICON: &'static [u8] = include_bytes!("images/icons/sound/SoundM.png");

pub fn sound_icon(app_image_cache: Arc<Mutex<BTreeMap<PathBuf,Handle>>>, volume: f32, muted: bool) -> Handle {
    let error_handle = Handle::from_rgba(1,1,vec![255u8,0u8,0u8,255u8]);
    match (app_image_cache.lock(),data_dir()) {
        (Ok(app_image_cache),Some(data_dir)) => {
            let data_folder = data_dir.join("Frostwin");
            if muted {
                app_image_cache.get(&data_folder.join("icons/sound/SoundM.png")).unwrap_or(&error_handle).clone()
            } else {
                if volume >= 0.90 {
                    app_image_cache.get(&data_folder.join("icons/sound/Sound4.png")).unwrap_or(&error_handle).clone()
                } else if volume >= 0.65 {
                    app_image_cache.get(&data_folder.join("icons/sound/Sound3.png")).unwrap_or(&error_handle).clone()
                } else if volume >= 0.40 {
                    app_image_cache.get(&data_folder.join("icons/sound/Sound2.png")).unwrap_or(&error_handle).clone()
                } else if volume >= 0.15 {
                    app_image_cache.get(&data_folder.join("icons/sound/Sound1.png")).unwrap_or(&error_handle).clone()
                } else {
                    app_image_cache.get(&data_folder.join("icons/sound/Sound0.png")).unwrap_or(&error_handle).clone()
                }
            }
        }
        (Err(e),_) => {
            eprintln!("Error accessing app_image_cache: {}", e);
            error_handle
        }
        (_,None) => {
            eprintln!("Error getting data_dir");
            error_handle
        }
    }
}

pub const CANCEL_ICON: &'static [u8] = include_bytes!("images/icons/power/Cancel.png");
pub const LOCK_ICON: &'static [u8] = include_bytes!("images/icons/power/Lock.png");
pub const LOGOFF_ICON: &'static [u8] = include_bytes!("images/icons/power/Logoff.png");
pub const RESTART_ICON: &'static [u8] = include_bytes!("images/icons/power/Restart.png");
pub const SHUTDOWN_ICON: &'static [u8] = include_bytes!("images/icons/power/Shutdown.png");

pub const EMPTY_APP_ICON: &'static [u8] = include_bytes!("images/icons/EmptyApp.png");
pub const TREE_DOT_ICON: &'static [u8] = include_bytes!("images/icons/TreeDot.png");
pub const FOLDER_ICON: &'static [u8] = include_bytes!("images/icons/Folder.png");
pub const SETTINGS_ICON: &'static [u8] = include_bytes!("images/icons/Settings.png");

pub fn unpack_missing_icons(path: PathBuf) -> std::io::Result<()> {
    let assets: &[(&str, &[u8])] = &[
        ("icons/startIcon/StartIconClosed.png", START_CLOSED_ICON),
        ("icons/startIcon/StartIconOpen.png", START_OPEN_ICON),
        ("icons/network/WifiIcon4.png", WIFI_ICON4_ICON),
        ("icons/network/WifiIcon3.png", WIFI_ICON3_ICON),
        ("icons/network/WifiIcon2.png", WIFI_ICON2_ICON),
        ("icons/network/WifiIcon1.png", WIFI_ICON1_ICON),
        ("icons/network/WifiIcon0.png", WIFI_ICON0_ICON),
        ("icons/network/WifiIconNone.png", WIFI_NONE_ICON),
        ("icons/network/EthernetIcon.png", ETHERNET_ICON),
        ("icons/network/NoInternet.png", NO_INTERNET_ICON),
        ("icons/battery/Battery100.png", BATTERY100_ICON),
        ("icons/battery/Battery90.png", BATTERY90_ICON),
        ("icons/battery/Battery80.png", BATTERY80_ICON),
        ("icons/battery/Battery70.png", BATTERY70_ICON),
        ("icons/battery/Battery60.png", BATTERY60_ICON),
        ("icons/battery/Battery50.png", BATTERY50_ICON),
        ("icons/battery/Battery40.png", BATTERY40_ICON),
        ("icons/battery/Battery30.png", BATTERY30_ICON),
        ("icons/battery/Battery20.png", BATTERY20_ICON),
        ("icons/battery/Battery10.png", BATTERY10_ICON),
        ("icons/battery/BatteryC100.png", BATTERY_C100_ICON),
        ("icons/battery/BatteryC90.png", BATTERY_C90_ICON),
        ("icons/battery/BatteryC80.png", BATTERY_C80_ICON),
        ("icons/battery/BatteryC70.png", BATTERY_C70_ICON),
        ("icons/battery/BatteryC60.png", BATTERY_C60_ICON),
        ("icons/battery/BatteryC50.png", BATTERY_C50_ICON),
        ("icons/battery/BatteryC40.png", BATTERY_C40_ICON),
        ("icons/battery/BatteryC30.png", BATTERY_C30_ICON),
        ("icons/battery/BatteryC20.png", BATTERY_C20_ICON),
        ("icons/battery/BatteryC10.png", BATTERY_C10_ICON),
        ("icons/sound/Sound0.png", SOUND0_ICON),
        ("icons/sound/Sound1.png", SOUND1_ICON),
        ("icons/sound/Sound2.png", SOUND2_ICON),
        ("icons/sound/Sound3.png", SOUND3_ICON),
        ("icons/sound/Sound4.png", SOUND4_ICON),
        ("icons/sound/SoundM.png", SOUND_M_ICON),
        ("icons/power/Shutdown.png", SHUTDOWN_ICON),
        ("icons/power/Restart.png", RESTART_ICON),
        ("icons/power/Lock.png", LOCK_ICON),
        ("icons/power/Logoff.png", LOGOFF_ICON),
        ("icons/power/Cancel.png", CANCEL_ICON),
        ("icons/Folder.png", FOLDER_ICON),
        ("icons/TreeDot.png", TREE_DOT_ICON),
        ("icons/EmptyApp.png", EMPTY_APP_ICON),
        ("icons/Settings.png", SETTINGS_ICON),
    ];

    for (rel_path, data) in assets {
        let path = path.join(rel_path);

        // 2. Check if file already exists
        if !path.exists() {
            // 3. Ensure the parent directory exists (e.g., icons/battery/)
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // 4. Write the binary data to the file
            let mut file = std::fs::File::create(path)?;
            file.write_all(data)?;
            println!("Unpacked: {}", rel_path);
        }
    }

    Ok(())
}

pub fn load_frostwin_icons(
    root_path: &PathBuf,
    cache: Arc<Mutex<BTreeMap<PathBuf, Handle>>>
) -> std::io::Result<()> {
    fn visit_dirs(
        dir: &PathBuf,
        cache: &Arc<Mutex<BTreeMap<PathBuf, Handle>>>
    ) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, cache)?;
                } else {
                    let handle = Handle::from_path(&path);
                    let mut map = cache.lock().unwrap();
                    map.insert(path, handle);
                }
            }
        }
        Ok(())
    }

    visit_dirs(root_path, &cache)?;
    Ok(())
}