use cursive::theme::{BaseColor, Color, PaletteColor, Theme, Color::Rgb};
pub enum InterfaceTheme {
    Windows98
}
impl InterfaceTheme {
    pub fn get(interface_theme: InterfaceTheme) -> Theme {
        let mut theme = Theme::default();

        match interface_theme {
            InterfaceTheme::Windows98 => {
                theme.palette[PaletteColor::Background] = Rgb(0, 128, 128); // Turquoise (teal)
                theme.palette[PaletteColor::View] = Color::Dark(BaseColor::White);
                theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Black); // White text
                theme.palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Blue); // Classic title bar
                theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::Red);   // Highlight
                theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::White);

                theme
            }
        }
    }
}