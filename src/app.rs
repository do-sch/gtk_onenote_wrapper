use crate::res;
use crate::window::Window;
use gio::*;
use gtk::*;

#[derive(Debug, Clone)]
pub struct App {
    app: gtk::Application,
    window: Window
}

impl App {
    pub fn new() -> Result<Self, &'static str> {
        // Create a GTK application
        let app = gtk::Application::new(res::APP_ID, gio::ApplicationFlags::FLAGS_NONE).unwrap();
        let window = Window::new();
        let new_app = App { app, window };

        glib::set_application_name(res::APP_NAME);
        glib::set_prgname(Some("onenote_gtk_wrapper"));

        new_app.setup_gactions();
        new_app.setup_signals();

        Ok(new_app)
    }

    fn add_gaction<F>(&self, name: &str, action: F)
    where
        for<'r, 's> F: Fn(&'r gio::SimpleAction, &'s Option<glib::Variant>) + 'static,
    {
        let simple_action = gio::SimpleAction::new(name, None);
        simple_action.connect_activate(action);
        self.app.add_action(&simple_action);
    }

    fn setup_gactions(&self) {
        // Quit
        let app = &self.app;
        self.add_gaction("quit", clone!(app => move |_, _| app.quit()));
        self.app.set_accels_for_action("app.quit", &["<primary>q"]);

        // About
//        let about

    }

    fn setup_signals(&self) {
        let window = &self.window.window;
        self.app.connect_activate(clone!(window => move |app| app.add_window(&window)));
    }

    pub fn run(&self, args: &[String]) {
        self.app.run(args);
    }

}
