use ratatui::style::{Color, Modifier, Style};
pub trait Theme {
    /// Background of the whole application, only use once.
    fn background(&self) -> Color;

    /// Font Color.
    fn font(&self) -> Color;

    /// This determines the object being rendered on a surface.
    fn on_surface(&self) -> Color;

    /// This determines the background of the surface. Normally used in panel/pages.
    fn surface_low(&self) -> Color;

    /// This determines the background of the surface. Normally used in popups/dialogs.
    fn surface_high(&self) -> Color;

    /// Style of Markdown Headers.
    fn header(&self, level: u8) -> Style;

    /// Style of Markdown bolded text.
    fn bold(&self) -> Style;

    /// Style of Markdown italic text.
    fn italic(&self) -> Style;

    /// Style of Markdown underline text.
    fn underline(&self) -> Style;

    /// Style of Markdown strikethrough text.
    fn strikethrough(&self) -> Style;
}
pub struct Iceberg;
impl Theme for Iceberg {
    fn background(&self) -> Color {
        Color::Rgb(24, 27, 30)
    }
    fn font(&self) -> Color {
        Color::Rgb(199, 221, 246)
    }
    fn on_surface(&self) -> Color {
        Color::Rgb(189, 211, 236)
    }
    fn surface_low(&self) -> Color {
        Color::Rgb(34, 41, 47)
    }
    fn surface_high(&self) -> Color {
        Color::Rgb(117, 151, 181)
    }
    fn header(&self, level: u8) -> Style {
        match level {
            0 => {
                Style::default().add_modifier(Modifier::BOLD)
            }
            1 => {
                Style::default().add_modifier(Modifier::BOLD)
            }
            2 => {
                Style::default().add_modifier(Modifier::BOLD)
            }
            _ => {
                Style::default().add_modifier(Modifier::BOLD)
            }
        }
    }
    fn bold(&self) -> Style{
        Style::default().add_modifier(Modifier::BOLD)
    }
    fn italic(&self) -> Style {
        Style::default().add_modifier(Modifier::ITALIC)
    }
    fn underline(&self) -> Style {
        Style::default().add_modifier(Modifier::UNDERLINED)
    }
    fn strikethrough(&self) -> Style {
        Style::default().add_modifier(Modifier::CROSSED_OUT)
    }
}
