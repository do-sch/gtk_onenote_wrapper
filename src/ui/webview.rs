use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use gtk::WidgetExt;
use webkit2gtk::{WebViewExt, WebContextExt, CookieManagerExt, CookiePersistentStorage};

use crate::window::HOME;
use crate::res::NAME;

pub fn webview_new() -> webkit2gtk::WebView {
    // Creates the WebView
    let web_view = webkit2gtk::WebView::new();

    // Set minimum size of widget
    web_view.set_size_request(640, 480);

    // Create .cache path
    let mut cache_path = glib::get_user_cache_dir().unwrap();
    cache_path.push(NAME);
    create_dir_all(&cache_path).expect("error creating path");

    // Create cookie file
    let mut cookie_path = cache_path.clone();
    cookie_path.push("cookies.sqlite");
    OpenOptions::new().create(true).write(true).open(&cookie_path).expect("error creating file");
    let cookie_path_str = cookie_path.as_path().to_str().unwrap();

    // Set cookie file
    web_view.get_context().unwrap().get_cookie_manager().unwrap().set_persistent_storage(
        cookie_path_str,
        CookiePersistentStorage::Sqlite
    );

    // Create webkit cache path
    cache_path.push("WebKit cache");
    create_dir_all(&cache_path).expect("error creating path");
    let cache_path_str = cache_path.as_path().to_str().unwrap();
    web_view.get_context().unwrap().set_disk_cache_directory(cache_path_str);

    // Load URL
    web_view.load_uri(HOME);

    web_view
}

//const ZOOM_Steps: &[];

//#[derive(Debug,Clone)]
//pub struct WebView {
//    pub web_view: webkit2gtk::WebView,
//
//    zoom: i32
//}

//impl WebView {
//
//    pub fn new(header_bar: & HeaderBar) -> Self {
//
//        // Creates the WebView
//        let web_view = webkit2gtk::WebView::new();
//        let zoom = 1;
//
//        // Set minimum size of widget
//        web_view.set_size_request(640, 480);
//
//        // Load URL
//        web_view.load_uri(HOME);
//
//        // Set HeaderBar title
//        web_view.connect_load_changed(clone!(header_bar => move | view, event | {
//        if event == webkit2gtk::LoadEvent::Finished {
//            let path = view.get_uri().unwrap();
//            let title = view.get_title().unwrap();
//
//            println ! ("{}", title);
//            println ! ("{}\n", path);
//
//            if HIDE_BUTTON.iter().any( | p | path.starts_with(p)) {
//                header_bar.hide_button();
//            } else if path.starts_with("https://onedrive.live.com/edit.aspx") {
//
//            view.run_javascript(
//                "var elst = document.getElementById('sdx_ow_div').style;\
//                 elst.height = 'auto';\
//                 elst.top = '-50px';\
//                 elst.bottom = '0px';",
//                None,
//                | _| {}
//            );
//
//                header_bar.show_button();
//
//            } else {
//
//                header_bar.suggest_button();
//
//            }
//
//            header_bar.set_title(title.as_str());
//            header_bar.set_subtitle(path.as_str())
//        }
//        }));
//
//        Self { web_view, zoom }
//    }
//
//    pub fn home(&self) {
//        &self.web_view.load_uri(HOME);
//    }
//
//    pub fn inc_zoom(&self) {
//
//    }
//
//    pub fn dec_zoom(&self) {
//
//    }
//
//    pub fn get_zoom_perc() -> i32 {
//
//    }
//}