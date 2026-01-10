use iced::widget::{button, container, slider};
use iced::{border, Background, Border, Color, Shadow, Theme, Vector};
use iced::border::Radius;

// This function returns a custom style for our button
pub fn transparent_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
        text_color: Color::WHITE,
        snap: true,
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.9, 0.9, 0.9, 0.4))),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.9, 0.9, 0.9, 0.4))),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color::from_rgba(0.5, 0.5, 0.5, 0.5),
            ..base
        },
    }
}
pub fn colored_button(_theme: &Theme, status: button::Status, color: Color) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(color)),
        border: Border {
            radius: 5.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
        text_color: Color::WHITE,
        snap: true,
    };
    let hovered_color = Color::from_rgba((0.2 + color.r).min(1.0), (0.2 + color.g).min(1.0), (0.2 + color.b).min(1.0), color.a);
    let pressed_color = Color::from_rgba((0.3 + color.r).min(1.0), (0.3 + color.g).min(1.0), (0.3 + color.b).min(1.0), color.a);
    let disabled_color = Color::from_rgba((color.r - 3.0).max(0.0), (color.g - 3.0).max(0.0), (color.b - 3.0).max(0.0), color.a);
    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(hovered_color)),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(pressed_color)),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(disabled_color)),
            text_color: Color::from_rgba(0.5, 0.5, 0.5, 0.5),
            ..base
        },
    }
}

pub fn my_slider(_theme: &Theme, status: slider::Status) -> slider::Style {
    let base = slider::Style{
        rail: slider::Rail {
            backgrounds: (Background::Color(Color::from_rgb(0.9,0.9,0.9)), Background::Color(Color::from_rgb(0.4,0.4,0.4))),
            width: 8.0,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: border::radius(4.0),
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Rectangle { width: 8, border_radius: border::radius(4.0) },
            background: Background::Color(Color::from_rgb(0.9,0.9,0.9)),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    };
    match status {
        slider::Status::Active => base,
        slider::Status::Hovered => base,
        slider::Status::Dragged => base,
    }
}

pub fn context_menu_button(theme: &Theme, _status: button::Status) -> button::Style {
    transparent_button(theme, button::Status::Active)
}

pub fn window_style(theme: &Theme) -> container::Style {
    container::Style {
        text_color: theme.palette().text.into(),
        background: Background::Color(theme.palette().background).into(),
        border: Border {
            color: Color::from_rgba(
                (theme.palette().background.r - 0.3).max(0.0),
                (theme.palette().background.g - 0.3).max(0.0),
                (theme.palette().background.b - 0.3).max(0.0),
                (theme.palette().background.a - 0.3).max(0.0),
            ),
            width: 2.0,
            radius: Radius::new(8),
        },
        shadow: Shadow {
            color: Default::default(),
            offset: Default::default(),
            blur_radius: 0.0,
        },
        snap: true,
    }
}