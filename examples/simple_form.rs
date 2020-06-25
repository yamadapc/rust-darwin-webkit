extern crate cocoa;
extern crate darwin_webkit;

use cocoa::base::id;
use darwin_webkit::helpers::dwk_app::DarwinWKApp;
use std::rc::Rc;

fn main() {
    unsafe {
        let app = DarwinWKApp::new("Host an app");
        let webview = Rc::new(app.create_webview());

        let mut callback = |_: id, _: id| {
            println!("JavaScript called rust!");
            webview.evaluate_javascript("document.body.innerHTML += ' -> response from rust';");
        };
        webview.add_message_handler("hello", &mut callback);
        webview.load_html_string(
            "
            <h1>Hello there</h1>

            <script>
                document.body.innerHTML += 'start';
                window.webkit.messageHandlers.hello.postMessage('hello');
                document.body.innerHTML += ' -> success';
            </script>
            ",
            "",
        );

        app.set_webview(&webview);
        app.run();
    }
}
