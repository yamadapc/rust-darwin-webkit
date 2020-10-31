extern crate cocoa;
extern crate criterion;
extern crate darwin_webkit;

use cocoa::base::id;
use darwin_webkit::helpers::dwk_app::DarwinWKApp;
use darwin_webkit::helpers::dwk_webview::{string_from_nsstring, DarwinWKWebView};
use darwin_webkit::webkit::wk_script_message_handler::WKScriptMessage;
use std::ops::{Add, Deref};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, RecvError};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

unsafe fn count_with_message_handlers(
    webview: Arc<DarwinWKWebView>,
    n: u64,
) -> Arc<DarwinWKWebView> {
    println!("Starting webview");
    let (sender, receiver) = channel();

    let mut i = 0;
    let message_sender = sender.clone();
    let cb_webview = webview.clone();
    let mut callback = Box::new(Box::new(move |_: id, _message: id| {
        i += 1;
        let value = i;

        if value > n {
            message_sender.send(());
        } else {
            let main_cb_webview = cb_webview.clone();
            dispatch::Queue::main().exec_async(move || {
                main_cb_webview.evaluate_javascript(format!("onMessage('{}')", value).as_str());
            });
        }
    }));
    let callback = Box::into_raw(callback);

    let start_webview = webview.clone();
    start_webview.add_message_handler("general", callback);
    let start = Instant::now();
    dispatch::Queue::main().exec_async(move || {
        println!("Sending message 1");
        start_webview.evaluate_javascript("onMessage('1')");
    });

    receiver.recv();
    let duration = start.elapsed();
    println!("Finished in {:?} - Sent {:?} messages", duration, n);
    let average_duration: f64 = (duration.as_millis() as f64) / (n as f64);
    println!("Average: {:?}ms", average_duration);

    println!("Thread waiting");
    webview
}

fn main() {
    unsafe {
        let app = Arc::new(DarwinWKApp::new("Host an app"));
        let webview = Arc::new(app.create_webview());
        webview.load_html_string(
            r"
                <script>
                window.onerror = (msg, url, line, column, error) => {
                  const message = {
                    message: msg,
                    url: url,
                    line: line,
                    column: column,
                    error: JSON.stringify(error)
                  }

                  window.webkit.messageHandlers.general.postMessage(JSON.stringify(message));
                };

                window.onMessage = function onMessage(n) {
                    // n = +n;
                    window.webkit.messageHandlers.general.postMessage(null);
                };
                </script>
                ",
            "",
        );
        app.set_webview(&webview);

        let main_thread_app = app.clone();
        let main_thread = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            count_with_message_handlers(webview.clone(), 50000);
            main_thread_app.stop();
        });

        app.run();
        main_thread.join().unwrap();
    }
}
