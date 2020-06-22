extern crate cocoa;
extern crate darwin_webkit;

use darwin_webkit::helpers::dwk_app::DarwinWKApp;

fn main() {
    unsafe {
        let app = DarwinWKApp::new("Host an app");
        let webview = app.create_webview();

        webview.load_html_string("<h1>Hello there</h1>", "");

        app.set_webview(&webview);
        app.run();
    }
}
