use ratatui::style::Color;
pub trait Theme {
    fn background_color(&self) -> Color;
    fn foreground_color(&self) -> Color;
    fn surface_low_color(&self) -> Color;
}

pub struct Iceberg;
impl Theme for Iceberg {
    fn background_color(&self) -> Color {
        Color::Rgb(27, 31, 35)
    }
    fn foreground_color(&self) -> Color {
        Color::Rgb(238, 255, 255)
    }
    fn surface_low_color(&self) -> Color {
        Color::Rgb(38, 52, 64)
    }
}