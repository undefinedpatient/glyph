use ratatui::style::{Color, Style};
pub trait Theme {
    fn background_color(&self) -> Color;
    fn foreground_color(&self) -> Color;
    fn surface_low_color(&self) -> Color;
    fn bold(&self) -> Style;
    fn italic(&self) -> Style;
    fn underline(&self) -> Style;
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
    fn bold(&self) -> Style{
        Style::default().bold()
    }
    fn italic(&self) -> Style {
        Style::default().italic()
    }
    fn underline(&self) -> Style {
        Style::default().underlined()
    }
}
