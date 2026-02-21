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

    /// Used for highlighting on surface_low
    fn surface_low_highlight(&self) -> Color;

    /// This determines the background of the surface. Normally used in popups/dialogs.
    fn surface_high(&self) -> Color;

    fn surface_high_highlight(&self) -> Color;

    /// Style of bolded text.
    fn bold(&self) -> Style;

    /// Style of italic text.
    fn italic(&self) -> Style;

    /// Style of underline text.
    fn underline(&self) -> Style;

    /// Style of strikethrough text.
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
        Color::Rgb(29, 36, 42)
    }

    fn surface_low_highlight(&self) -> Color {
        Color::Rgb(51, 58, 64)
    }

    fn surface_high(&self) -> Color {
        Color::Rgb(117, 151, 181)
    }
    fn surface_high_highlight(&self) -> Color {
        Color::Rgb(127, 161, 191)
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
