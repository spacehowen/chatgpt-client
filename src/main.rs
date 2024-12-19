use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use webkit2gtk::{CookieManagerExt, WebContext, WebContextExt, WebView, WebViewExt};

fn main() {

    let app = Application::new(Some("com.spacehowen.chatgptclient"), Default::default());

    app.connect_activate(|app| {

        let window = ApplicationWindow::new(app);
        window.set_default_size(800, 600);

        window.set_title("ChatGPT Client");

        let context = WebContext::default().expect("No se pudo crear un WebContext");

        if let Some(cookie_manager) = context.cookie_manager() {
            cookie_manager.set_persistent_storage(
                "./cookies",
                webkit2gtk::CookiePersistentStorage::Text,
            );
        }

        let webview = WebView::with_context(&context);

        webview.load_uri("https://chat.openai.com/");

        window.set_child(Some(&webview));

        window.show_all();
    });

    app.run();
}