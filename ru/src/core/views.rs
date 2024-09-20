use crate::core::menu::{Menu, MenuItem};

#[derive(Debug, Clone, Copy)]
pub enum MainMenu {
    Options,
    Quit,
}

pub fn main_menu() -> Menu<MainMenu, &'static str> {
    let mut options: Vec<MenuItem<MainMenu, &str>> = Vec::new();
    options.insert(0, MenuItem::new("Options", MainMenu::Options));
    options.insert(1, MenuItem::new("Quit", MainMenu::Quit));
    Menu::new(options)
}
