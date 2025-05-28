use egui::Color32;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black,
    DarkGray,
}

// https://lospec.com/palette-list/vanilla-milkshake
impl Into<Color32> for Color {
    fn into(self) -> Color32 {
        match self {
            Color::Black => Color32::from_rgb(40, 40, 46),
            Color::DarkGray => Color32::from_rgb(108, 86, 113),
        }
    }
}
