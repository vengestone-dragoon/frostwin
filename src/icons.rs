use crate::sys_util::WifiStatus;
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{Geometry, LineCap, Stroke};
use iced::{border, Point, Radians, Size};
use iced::{Color, Rectangle, Renderer, Theme};
use std::any::Any;
use std::collections::BTreeMap;
use std::f32::consts::PI;
use std::sync::Mutex;

pub struct SmartCache<S>
where
    S: PartialEq + Clone
{
    cache: canvas::Cache,
    prev_state: Option<S>,
}

impl<S> SmartCache<S>
where
    S: PartialEq + Clone
{
    pub fn new() -> Self {
        Self {
            cache: canvas::Cache::default(),
            prev_state: None,
        }
    }

    /// Access the cache, automatically clearing it if the state has changed
    pub fn get(&mut self, current_state: S) -> &canvas::Cache {
        if let Some(prev) = &self.prev_state {
            if prev != &current_state {
                self.cache.clear();
                self.prev_state = Some(current_state);
            }
        } else {
            // First run
            self.prev_state = Some(current_state);
        }
        &self.cache
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub enum FrostwinIcons {
    StartIcon(String),
    DropDownIcon(String),
    PowerButton(String),
    RestartButton(String),
    LockButton(String),
    LogoffButton(String),
    CancelButton(String),
    FolderButton(String),
    TreeDot(String),
    EmptyApp(String),
    BatteryIcon(String),
    WifiIcon(String),
    VolumeIcon(String),
}

pub struct StartLogo {
    pub id: String,
    pub open: bool,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons,Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for StartLogo {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::StartIcon(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(self.open).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let half_point = base_size * 52.0;
            let square_border = border::radius(base_size * 8.0);
            let square_size = Size::from([base_size * 46.0, base_size * 46.0]);
            let square1 = canvas::Path::rounded_rectangle(
                Point::from([base_size * 2.0,base_size * 2.0]),
                square_size,
                square_border);
            let square2 = canvas::Path::rounded_rectangle(
                Point::from([half_point,base_size * 2.0]),
                square_size,
                square_border);
            let square3 = canvas::Path::rounded_rectangle(
                Point::from([base_size * 2.0,half_point]),
                square_size,
                square_border);
            let square4 = canvas::Path::rounded_rectangle(
                Point::from([half_point,half_point]),
                square_size,
                square_border);
            if self.open {
                frame.fill(&square1,Color::from_rgb(0.9, 0.2, 0.2));
                frame.fill(&square2,Color::from_rgb(0.2, 0.9, 0.2));
                frame.fill(&square3,Color::from_rgb(0.2, 0.2, 0.9));
                frame.fill(&square4,Color::from_rgb(0.9, 0.9, 0.2));
            } else {
                frame.fill(&square1,Color::from_rgb(0.3, 0.5, 0.9));
                frame.fill(&square2,Color::from_rgb(0.3, 0.5, 0.9));
                frame.fill(&square3,Color::from_rgb(0.3, 0.5, 0.9));
                frame.fill(&square4,Color::from_rgb(0.3, 0.5, 0.9));
            }
        });
        vec![geometry]
    }
}
pub struct DropDownIcon {
    pub id: String,
    pub open: bool,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons,Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for DropDownIcon {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::DropDownIcon(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(self.open).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let stroke = Stroke::default().with_color(Color::from_rgb(0.9, 0.9, 0.9)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);
            let path = canvas::Path::new(|p| {
                if self.open {
                    p.move_to(Point::from([15.0 * base_size,33.3 * base_size]));
                    p.line_to(Point::from([50.0 * base_size,66.6 * base_size]));
                    p.line_to(Point::from([85.0 * base_size,33.3 * base_size]));
                } else {
                    p.move_to(Point::from([15.0 * base_size,66.6 * base_size]));
                    p.line_to(Point::from([50.0 * base_size,33.3 * base_size]));
                    p.line_to(Point::from([85.0 * base_size,66.6 * base_size]));
                }
            });
            frame.stroke(&path, stroke);
        });
        vec![geometry]
    }
}

pub struct PowerButton {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for PowerButton {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::PowerButton(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(true).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let stroke = Stroke::default().with_color(Color::from_rgb(0.9, 0.9, 0.9)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);
            let circle = canvas::Path::circle(
                Point::from([50.0 * base_size, 50.0 * base_size]),
                33.3 * base_size,
            );
            let line = canvas::Path::line(
                Point::from([50.0 * base_size, 4.0 * base_size]),
                Point::from([50.0 * base_size, 33.3 * base_size]),
            );
            frame.stroke(&circle, stroke);
            frame.stroke(&line, stroke);
        });

        vec![geometry]
    }
}

pub struct RestartButton {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for RestartButton {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::RestartButton(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(true).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let stroke = Stroke::default().with_color(Color::from_rgb(0.9, 0.9, 0.9)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);
            let circle = canvas::Path::circle(
                Point::from([50.0 * base_size, 50.0 * base_size]),
                33.3 * base_size,
            );
            let line = canvas::Path::line(
                Point::from([58.0 * base_size, 4.0 * base_size]),
                Point::from([42.0 * base_size, 16.7 * base_size])
            );
            let line2 = canvas::Path::line(
                Point::from([42.0 * base_size, 16.7 * base_size]),
                Point::from([58.0 * base_size, 33.3 * base_size])
            );
            frame.stroke(&circle, stroke);
            frame.stroke(&line, stroke);
            frame.stroke(&line2, stroke);
        });

        vec![geometry]
    }
}
pub struct LockButton {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for LockButton {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::LockButton(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(true).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let stroke = Stroke::default().with_color(Color::from_rgb(0.9, 0.9, 0.9)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);
            let arc = canvas::Path::new(|builder| {
                builder.arc(Arc {
                    center: Point::from([50.0 * base_size,50.0 * base_size]),
                    radius: 33.3 * base_size,
                    start_angle: Radians(PI),
                    end_angle: Radians(2.0 * PI)
                })
            });
            let rectangle = canvas::Path::rectangle(
                Point::from([16.7 * base_size, 50.0 * base_size]),
                Size::from([66.6 * base_size,33.3 * base_size]),
            );
            let line = canvas::Path::line(
                Point::from([50.0 * base_size, 58.0 * base_size]),
                Point::from([50.0 * base_size, 82.0 * base_size])
            );
            frame.stroke(&arc, stroke);
            frame.stroke(&rectangle, stroke);
            frame.stroke(&line, stroke);
        });

        vec![geometry]
    }
}

pub struct LogoffButton {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for LogoffButton {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::LogoffButton(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(true).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let stroke = Stroke::default().with_color(Color::from_rgb(0.9, 0.9, 0.9)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);

            let arrow = canvas::Path::new(|builder| {
                builder.move_to(Point::from([86.0 * base_size, 33.3 * base_size]));
                builder.line_to(Point::from([50.0 * base_size, 33.3 * base_size]));
                builder.line_to(Point::from([50.0 * base_size, 14.0 * base_size]));
                builder.line_to(Point::from([14.0 * base_size, 50.0 * base_size]));
                builder.line_to(Point::from([50.0 * base_size, 86.0 * base_size]));
                builder.line_to(Point::from([50.0 * base_size, 66.6 * base_size]));
                builder.line_to(Point::from([86.0 * base_size, 66.6 * base_size]));
                builder.close()
            }
            );
            frame.stroke(&arrow, stroke);
        });

        vec![geometry]
    }
}

pub struct CancelButton {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for CancelButton {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::CancelButton(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(true).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let stroke = Stroke::default().with_color(Color::from_rgb(0.9, 0.9, 0.9)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);
            let circle = canvas::Path::circle(
                Point::from([50.0 * base_size, 50.0 * base_size]),
                33.3 * base_size,
            );
            let line1 = canvas::Path::line(
                Point::from([26.5 * base_size, 26.5 * base_size]),
                Point::from([73.5 * base_size, 73.5 * base_size])
            );
            let line2 = canvas::Path::line(
                Point::from([73.5 * base_size, 26.5 * base_size]),
                Point::from([26.5 * base_size, 73.5 * base_size])
            );
            frame.stroke(&circle, stroke);
            frame.stroke(&line1, stroke);
            frame.stroke(&line2, stroke);
        });

        vec![geometry]
    }
}

pub struct FolderButton {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for FolderButton {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::FolderButton(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(true).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let rectangle = canvas::Path::rectangle(
                Point::from([2.0 * base_size, 8.0 * base_size]),
                Size::from([96.0 * base_size,90.0 * base_size]),
            );
            let line = canvas::Path::line(
                Point::from([70.0 * base_size, 4.0 * base_size]),
                Point::from([90.0 * base_size, 4.0 * base_size])
            );
            frame.fill(&rectangle, Color::from_rgb(0.7, 0.7, 0.2));
            frame.stroke(&line, Stroke::default().with_line_cap(LineCap::Round).with_width(base_size * 16.0).with_color(Color::from_rgb(0.7,0.7,0.2)));
        });
        vec![geometry]
    }
}
pub struct TreeDot {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for TreeDot {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::TreeDot(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(true).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let circle = canvas::Path::circle(
                Point::from([50.0 * base_size, 50.0 * base_size]),
                25.0 * base_size,
            );
            frame.fill(&circle, Color::from_rgb(0.9, 0.9, 0.9));
        });
        vec![geometry]
    }
}
pub struct EmptyApp {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
}
impl<Message> canvas::Program<Message> for EmptyApp {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<bool> = cache_lock.entry(FrostwinIcons::EmptyApp(self.id.clone())).or_insert(
            Box::new(SmartCache::<bool>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get(true).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let rectangle = canvas::Path::rounded_rectangle(
                Point::from([2.0 * base_size, 8.0 * base_size]),
                Size::from([96.0 * base_size,90.0 * base_size]),
                border::radius(base_size * 15.0)
            );
            let image = canvas::Path::rectangle(
                Point::from([8.0 * base_size, 16.0 * base_size]),
                Size::from([40.0 * base_size,40.0 * base_size]),
            );
            frame.fill(&rectangle, Color::from_rgb(0.9, 0.9, 0.9));
            frame.fill(&image, Color::from_rgb(0.2, 0.2, 0.7));
        });
        vec![geometry]
    }
}
pub struct BatteryIcon {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
    pub charging: bool,
    pub level: f32,
}
impl<Message> canvas::Program<Message> for BatteryIcon {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer,_theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<(bool,f32)> = cache_lock.entry(FrostwinIcons::BatteryIcon(self.id.clone())).or_insert(
            Box::new(SmartCache::<(bool,f32)>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let battery_percentage = 0.68 * (self.level / 5.0).round() * 5.0;
        let geometry = icon_cache.get((self.charging,battery_percentage)).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let stroke = Stroke::default().with_color(Color::from_rgb(0.9, 0.9, 0.9)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);
            let rectangle1 = canvas::Path::rounded_rectangle(
                Point::from([12.0 * base_size,14.0 * base_size]),
                Size::from([76.0 * base_size,80.0 * base_size]),
                border::radius(base_size * 15.0)
            );
            let rectangle2 = canvas::Path::rounded_rectangle(
                Point::from([25.0 * base_size,5.0 * base_size]),
                Size::from([50.0 * base_size,8.0 * base_size]),
                border::radius(base_size * 5.0)
            );
            let power_level = canvas::Path::rounded_rectangle(
                Point::from([18.0 * base_size, (88.0 - battery_percentage) * base_size]),
                Size::from([64.0 * base_size, battery_percentage * base_size]),
                border::radius(base_size * 9.0)
            );
            let charging_indicator = canvas::Path::new(|builder| {
                builder.move_to(Point::from([50.0 * base_size, 10.0 * base_size]));
                builder.line_to(Point::from([25.0 * base_size, 50.0 * base_size]));
                builder.line_to(Point::from([50.0 * base_size, 55.0 * base_size]));
                builder.line_to(Point::from([50.0 * base_size, 98.0 * base_size]));
                builder.line_to(Point::from([75.0 * base_size, 50.0 * base_size]));
                builder.line_to(Point::from([50.0 * base_size, 45.0 * base_size]));
                builder.close()
            });
            if self.charging {
                frame.fill(&power_level, Color::from_rgb(0.2, 0.9, 0.2));
                frame.fill(&charging_indicator, Color::from_rgb(0.9, 0.9, 0.9));
            } else {
                if battery_percentage > 15.0 {
                    frame.fill(&power_level, Color::from_rgb(0.9, 0.9, 0.9));
                } else {
                    frame.fill(&power_level, Color::from_rgb(0.9, 0.2, 0.2));
                }
            }
            frame.stroke(&rectangle1, stroke);
            frame.fill(&rectangle2, Color::from_rgb(0.9, 0.9, 0.9));
            frame.stroke(&rectangle2, stroke);
        });

        vec![geometry]
    }
}
pub struct WifiIcon {
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons, Box<dyn Any>>>>,
    pub status: WifiStatus,
}
impl<Message> canvas::Program<Message> for WifiIcon {
    type State = ();
    fn draw(&self, _state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry<Renderer>> {
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<i32> = cache_lock.entry(FrostwinIcons::WifiIcon(self.id.clone())).or_insert(
            Box::new(SmartCache::<i32>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let strength = match self.status.clone() {
            WifiStatus::Connected(_, strength) => {
                if strength >= 90 {
                    4
                } else if strength >= 65 {
                    3
                } else if strength >= 40 {
                    2
                } else if strength >= 15 {
                    1
                } else {
                    0
                }
            }
            WifiStatus::Ethernet => {
                -1
            }
            WifiStatus::Disconnected => {
                -2
            }
        };
        let geometry = icon_cache.get(strength).draw(renderer, bounds.size(), |frame| {

            let base_size = frame.height() / 100.0;
            let stroke_strong = Stroke::default().with_color(Color::from_rgb(0.9, 0.9, 0.9)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);
            let stroke_weak = Stroke::default().with_color(Color::from_rgb(0.4, 0.4, 0.4)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);
            let slice4 = canvas::Path::new(|builder| {
                builder.arc(Arc {
                    center: Point::from([50.0 * base_size,110.0 * base_size]),
                    radius: 96.0 * base_size,
                    start_angle: Radians(1.35 * PI),
                    end_angle: Radians(1.65 * PI)
                })
            });
            let slice3 = canvas::Path::new(|builder| {
                builder.arc(Arc {
                    center: Point::from([50.0 * base_size,110.0 * base_size]),
                    radius: 72.0 * base_size,
                    start_angle: Radians(1.35 * PI),
                    end_angle: Radians(1.65 * PI)
                })
            });
            let slice2 = canvas::Path::new(|builder| {
                builder.arc(Arc {
                    center: Point::from([50.0 * base_size,110.0 * base_size]),
                    radius: 48.0 * base_size,
                    start_angle: Radians(1.35 * PI),
                    end_angle: Radians(1.65 * PI)
                })
            });
            let slice1 = canvas::Path::new(|builder| {
                builder.arc(Arc {
                    center: Point::from([50.0 * base_size,110.0 * base_size]),
                    radius: 24.0 * base_size,
                    start_angle: Radians(1.35 * PI),
                    end_angle: Radians(1.65 * PI)
                })
            });
            canvas::Path::new(|builder| {
                builder.move_to(Point::from([6.0 * base_size, 14.0 * base_size]));
                builder.line_to(Point::from([94.0 * base_size, 86.0 * base_size]));
                builder.move_to(Point::from([94.0 * base_size, 14.0 * base_size]));
                builder.line_to(Point::from([6.0 * base_size, 86.0 * base_size]));
            });
            let ethernet = canvas::Path::new(|builder| {
                builder.move_to(Point::from([12.0 * base_size,86.0 * base_size]));
                builder.line_to(Point::from([12.0 * base_size,86.0 * base_size]));
            });
            let disconnected = canvas::Path::new(|builder| {
                builder.arc(Arc {
                    center: Point::from([50.0 * base_size,50.0 * base_size]),
                    radius: 44.0 * base_size,
                    start_angle: Radians(0.0),
                    end_angle: Radians(2.0 * PI)
                });
                builder.arc(Arc {
                    center: Point::from([0.0 * base_size,50.0 * base_size]),
                    radius: 70.0 * base_size,
                    start_angle: Radians(1.77 * PI),
                    end_angle: Radians(2.23 * PI)
                });
                builder.arc(Arc {
                    center: Point::from([100.0 * base_size,50.0 * base_size]),
                    radius: 70.0 * base_size,
                    start_angle: Radians(1.23 * PI),
                    end_angle: Radians(0.77 * PI)
                });
                builder.move_to(Point::from([10.0 * base_size,33.3 * base_size]));
                builder.line_to(Point::from([90.0 * base_size,33.3 * base_size]));
                builder.move_to(Point::from([10.0 * base_size,66.6 * base_size]));
                builder.line_to(Point::from([90.0 * base_size,66.6 * base_size]));
            });
            if strength == -2 {
                frame.stroke(&disconnected, stroke_weak);
            } else if strength == -1 {
                frame.stroke(&ethernet, stroke_strong);
            } else if strength >= 0  {
                frame.stroke(&slice4, if strength >= 4 { stroke_strong } else { stroke_weak });
                frame.stroke(&slice3, if strength >= 3 { stroke_strong } else { stroke_weak });
                frame.stroke(&slice2, if strength >= 2 { stroke_strong } else { stroke_weak });
                frame.stroke(&slice1, if strength >= 1 { stroke_strong } else { stroke_weak });
            }
        });
        vec![geometry]
    }
}

pub struct VolumeIcon{
    pub id: String,
    pub cache: std::sync::Arc<Mutex<BTreeMap<FrostwinIcons,Box<dyn Any>>>>,
    pub volume: f32,
    pub muted: bool
}
impl<Message> canvas::Program<Message> for VolumeIcon {
    type State = ();
    fn draw(&self, _state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry<Renderer>> {
        let volume = (1.0 + ((self.volume - 0.15) * 4.0)).floor() as usize;
        let mut cache_lock = self.cache.lock().unwrap();
        let icon_cache: &mut SmartCache<(bool,usize)> = cache_lock.entry(FrostwinIcons::VolumeIcon(self.id.clone())).or_insert(
            Box::new(SmartCache::<(bool,usize)>::new()) as Box<dyn Any>
        ).downcast_mut().unwrap();
        let geometry = icon_cache.get((self.muted,volume)).draw(renderer, bounds.size(), |frame| {
            let base_size = frame.height() / 100.0;
            let stroke = Stroke::default().with_color(Color::from_rgb(0.9, 0.9, 0.9)).with_width(8.0 * base_size).with_line_cap(LineCap::Round);
            let speaker = canvas::Path::new(|builder| {
                builder.move_to(Point::from([5.0 * base_size,33.3 * base_size]));
                builder.line_to(Point::from([5.0 * base_size,66.6 * base_size]));
                builder.line_to(Point::from([18.0 * base_size,66.6 * base_size]));
                builder.line_to(Point::from([33.3 * base_size,86.0 * base_size]));
                builder.line_to(Point::from([33.3 * base_size,14.0 * base_size]));
                builder.line_to(Point::from([18.0 * base_size,33.3 * base_size]));
                builder.close()
            });
            let volume = canvas::Path::new(|builder| {
                let base = Arc {
                    center: Point::from([0.0 * base_size, 50.0 * base_size]),
                    radius: 0.0,
                    start_angle: Radians(1.85 * PI),
                    end_angle: Radians(2.15 * PI)
                };
                if volume >= 1 {
                    builder.arc(Arc {radius: 51.0 * base_size, ..base})
                }
                if volume >= 2 {
                    builder.arc(Arc {radius: 66.0 * base_size, ..base})
                }
                if volume >= 3 {
                    builder.arc(Arc {radius: 81.0 * base_size, ..base})
                }
                if volume >= 4 {
                    builder.arc(Arc {radius: 96.0 * base_size, ..base})
                }
            });
            let mute = canvas::Path::new(|builder| {
                builder.move_to(Point::from([43.3 * base_size, 14.0 * base_size]));
                builder.line_to(Point::from([96.0 * base_size, 86.0 * base_size]));
                builder.move_to(Point::from([96.0 * base_size, 14.0 * base_size]));
                builder.line_to(Point::from([43.3 * base_size, 86.0 * base_size]));
            });
            frame.stroke(&speaker, stroke);
            if self.muted {
                frame.stroke(&mute, stroke);
            } else {
                frame.stroke(&volume, stroke);
            }
        });
        vec![geometry]
    }
}