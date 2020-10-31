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
use std::time::Duration;

unsafe fn count_with_message_handlers(
    webview: Arc<DarwinWKWebView>,
    n: u64,
) -> Arc<DarwinWKWebView> {
    println!("Starting webview");
    let (sender, receiver) = channel();
    let (message_sender, message_receiver) = channel();

    let mut i = 0;
    let cb_webview = webview.clone();
    let message_sender = message_sender.clone();
    let mut callback = Box::new(move |_: id, message: id| {
        println!("Parsing message");
        let body = message.body();
        let str = string_from_nsstring(body);
        println!(" > Message: {}", str.as_ref().unwrap().as_str());

        i += 1;
        let value = i;

        println!(" > Value: {}", value);
        if value > n {
            println!(" > End");
            message_sender.send("".to_string()).unwrap();
            sender.send(());
        } else {
            // dispatch::Queue::main().exec_sync(move || {
            println!(" > Sending message N");
            message_sender.send(String::from(format!("onMessage('{}')", value).as_str()));
            // });
        }
    });
    let callback = Box::into_raw(callback);

    let start_webview = webview.clone();
    dispatch::Queue::main().exec_async(move || {
        start_webview.add_message_handler("general", callback);
        println!("Sending message 1");
        start_webview.evaluate_javascript("onMessage('1')");
    });

    let mut running = true;
    while running {
        match message_receiver.recv() {
            Ok(msg) => {
                if msg == "" {
                    println!("Done");
                    running = false;
                } else {
                    println!("tick");
                    cb_webview.evaluate_javascript(msg.as_str());
                }
            }
            Err(err) => println!("{:?}", err),
        }
    }

    println!("Thread waiting");
    receiver.recv();
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
                    n = +n;
                    window.webkit.messageHandlers.general.postMessage('' + (n + 1));
                };
                </script>
                ",
            "",
        );
        app.set_webview(&webview);

        let main_thread = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            let ws = count_with_message_handlers(webview.clone(), 100);
            ws.clone();
        });

        app.run();
        main_thread.join().unwrap();
    }
}
