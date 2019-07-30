use gtk::{WidgetExt, HeaderBarExt, IconSize, ActionableExt, StyleContextExt, STYLE_CLASS_SUGGESTED_ACTION};

use crate::res;

#[derive(Debug, Clone)]
pub struct HeaderBar {
    pub header_bar: gtk::HeaderBar,
    home_button: gtk::Button,
    menu_button: gtk::MenuButton,
    zoom_level_label: gtk::Label
}

impl HeaderBar {

    pub fn new() -> Self {

        // Creates the main HeaderBar widget
        let header_bar = gtk::HeaderBar::new();

        // sets the text to display in the title section of the headerbar
        header_bar.set_title("OneNote");
        // enable subtitle
//        header_bar.set_has_subtitle(true);
        // enables the window controls within this headerbar
        header_bar.set_show_close_button(true);

        // button to return to notes
        let home_button = gtk::Button::new_from_icon_name("go-previous-symbolic", IconSize::Button.into());
        home_button.set_action_name("win.home");
        header_bar.pack_start(&home_button);

        // create Menu Button and Menu with builder
        let builder = gtk::Builder::new_from_string(res::MENU);
        let menu_button = builder.get_object::<gtk::MenuButton>("menu_button").unwrap();
        let zoom_level_label = builder.get_object::<gtk::Label>("zoom_level_label").unwrap();
        header_bar.pack_end(&menu_button);

        Self { header_bar, home_button, menu_button, zoom_level_label }
    }

    pub fn get_zoom_label(&self) -> &gtk::Label {
        &self.zoom_level_label
    }

    pub fn set_title(&self, title: &str) {
        &self.header_bar.set_title(title);
    }

    pub fn set_subtitle(&self, subtitle: &str) {
        &self.header_bar.set_subtitle(subtitle);
    }

    pub fn hide_button(&self) {
        &self.home_button.set_visible(false);
    }

    pub fn show_button(&self) {
        let style_context = &self.home_button.get_style_context().unwrap();
        style_context.remove_class(&STYLE_CLASS_SUGGESTED_ACTION);
        &self.home_button.set_visible(true);
    }

    pub fn suggest_button(&self) {
        let style_context = &self.home_button.get_style_context().unwrap();
        style_context.add_class(&STYLE_CLASS_SUGGESTED_ACTION);
        &self.home_button.set_visible(true);
    }

}