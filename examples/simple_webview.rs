extern crate cocoa;
extern crate darwin_webkit;

use cocoa::appkit::*;
use cocoa::base::*;
use cocoa::foundation::*;

use darwin_webkit::foundation::*;
use darwin_webkit::helpers::*;
use darwin_webkit::webkit::*;

fn main() {
    unsafe {
        let app = DarwinWKApp::new("Simple WebView");
        let webview = app.create_webview();

        webview.load_url("https://www.google.com.br");

        app.set_webview(&webview);
        app.run();
    }
}
