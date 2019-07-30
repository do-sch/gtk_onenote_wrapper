use gtk::*;
use gio::{SimpleActionGroup, SimpleActionExt, ActionMapExt};
use webkit2gtk::WebViewExt;
use crate::res;
use crate::ui;


pub const HOME: &'static str = "https://www.onenote.com/notebooks";
const HIDE_BUTTON: &'static [&'static str] = &["https://www.onenote.com/notebooks", "https://www.onenote.com/signin", "https://www.onenote.com/hrd", "https://login.live.com/login.srf", "https://login.live.com/ppsecure", "https://onedrive.live.com/redir"];

#[derive(Debug, Clone)]
pub struct Window {
    pub window: gtk::Window,
    header_bar: ui::HeaderBar,
    view: webkit2gtk::WebView,
    view_action_group: gio::SimpleActionGroup
}

impl Window {
    // Create a new window and assign it to the given application
    pub fn new() -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let header_bar = ui::HeaderBar::new();
        let view = ui::webview_new();
        let view_action_group = gio::SimpleActionGroup::new();

        window.set_default_size(800, 450);
        window.set_wmclass("gtk-onenote-wrapper", "OneNote Online GTK");
        window.set_position(gtk::WindowPosition::Center);
        window.set_title(res::APP_NAME);
        window.set_titlebar(&header_bar.header_bar);

        window.add(&view);
        window.insert_action_group("win", &view_action_group);

        window.show_all();

        // Dynamic HeaderBar
        header_bar.hide_button();
        view.connect_load_changed(clone!(header_bar => move | view, event | {
        match event {
            webkit2gtk::LoadEvent::Finished => {
                let path = view.get_uri().unwrap();
                let title = view.get_title().unwrap();

                if HIDE_BUTTON.iter().any( | p | path.starts_with(p)) {
                    header_bar.hide_button();
                } else if path.starts_with("https://onedrive.live.com/edit.aspx") {

                view.run_javascript(
                    "var elst = document.getElementById('sdx_ow_div').style;\
                     elst.height = 'auto';\
                     elst.top = '-50px';\
                     elst.bottom = '0px';",
                    None,
                    | _| {}
                );

                    header_bar.show_button();
                } else {

                    header_bar.suggest_button();

                }

                header_bar.set_title(title.as_str());
//                header_bar.set_subtitle(path.as_str())
                println!("{}", path.as_str());
            }
            webkit2gtk::LoadEvent::Committed => {
                if view.get_uri().unwrap().starts_with("https://www.onenote.com/notebooks?auth=1") {
                    view.run_javascript(
                        "document.addEventListener('DOMContentLoaded', function(){\
                             document.getElementById('h_bar').style.display='none';\
                             document.querySelector('.bodyContent').style.paddingTop='0px';\
                             document.querySelector('.ListHeader').style.top='0px';\
                         }, false);",
                        None,
                        |_| {}
                    );
                }
            }
            webkit2gtk::LoadEvent::Started => {
                if view.get_uri().unwrap().starts_with("https://login.live.com/gls.srf") {
                    view.load_uri(HOME);
                }
            }
            _ => {}
        }}));

        let new_window = Self {
            window, view, header_bar, view_action_group
        };

        new_window.setup_gactions();

        new_window
    }

    fn add_gaction<F>(&self, group: &SimpleActionGroup, name: &str, action: F) -> gio::SimpleAction
        where
                for<'r, 's> F: Fn(&'r gio::SimpleAction, &'s Option<glib::Variant>) + 'static,
    {
        let simple_action = gio::SimpleAction::new(name, None);
        simple_action.connect_activate(action);
        group.add_action(&simple_action);
        simple_action
    }

    fn setup_gactions(&self) {

        let view = &self.view;
        let view_action_group = &self.view_action_group;
        let zoom_label = self.header_bar.get_zoom_label();

        // return home
        self.add_gaction(&view_action_group, "home", clone!(view => move |_, _|{
            view.load_uri(HOME);
        }));


        // Zoom in
        self.add_gaction(&view_action_group, "zoom-in", clone!(zoom_label, view_action_group, view => move |sa, _|{
            let mut wanted_zoom = view.get_zoom_level() + 0.25;
            if wanted_zoom >= 2.5 {
                wanted_zoom = 2.5;
                sa.set_enabled(false);
            }
            let zout = view_action_group.lookup_action("zoom-out").unwrap().downcast::<gio::SimpleAction>().unwrap();
            let zdef = view_action_group.lookup_action("zoom-standard").unwrap().downcast::<gio::SimpleAction>().unwrap();
            zout.set_enabled(true);
            zdef.set_enabled(wanted_zoom != 1.0);
            view.set_zoom_level(wanted_zoom);
            zoom_label.set_text(&format!("{}%", (wanted_zoom * 100.0) as u32));
        }));

        // Zoom out
        self.add_gaction(&view_action_group, "zoom-out", clone!(zoom_label, view_action_group, view => move |sa, _|{
            let mut wanted_zoom = view.get_zoom_level() - 0.25;
            if wanted_zoom <= 0.5 {
                wanted_zoom = 0.5;
                sa.set_enabled(false);
            }
            let zin = view_action_group.lookup_action("zoom-in").unwrap().downcast::<gio::SimpleAction>().unwrap();
            let zdef = view_action_group.lookup_action("zoom-standard").unwrap().downcast::<gio::SimpleAction>().unwrap();
            zin.set_enabled(true);
            zdef.set_enabled(wanted_zoom != 1.0);
            view.set_zoom_level(wanted_zoom);
            zoom_label.set_text(&format!("{}%", (wanted_zoom * 100.0) as u32));
        }));

        // Zoom default
        self.add_gaction(&view_action_group, "zoom-standard", clone!(view => move |_, _|{
            view.set_zoom_level(1.0);
        })).set_enabled(false);

        // Print
        self.add_gaction(&view_action_group, "print", clone!(view => move |_, _|
            view.run_javascript(
                "window.print();",
                None,
                |_| {}
            )
        ));

        // Reload
        self.add_gaction(&view_action_group, "reload", clone!(view => move |_, _|{
            view.reload()
        }));


        // Logout
        self.add_gaction(&view_action_group, "logout", clone!(view => move |_, _|{
            view.load_uri("https://login.live.com/logout.srf");
        }));

    }



}