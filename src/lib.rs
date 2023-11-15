//! ![](https://github.com/yamadapc/rust-darwin-webkit/workflows/Rust/badge.svg)
//! ![](https://github.com/yamadapc/rust-darwin-webkit/workflows/Documentation/badge.svg)
//! ![Crates.io](https://img.shields.io/crates/v/darwin-webkit)
//!
//! * [**Documentation**](https://yamadapc.github.io/rust-darwin-webkit/darwin_webkit/)
//! * [**Crate on crates.io**](https://crates.io/crates/darwin-webkit)
//! **darwin_webkit** exposes bindings to some of the WebKit's API on MacOS for
//! Rust.
//!
//! Modules follow the `cocoa` crate convention for naming bindings.
//!
//! It uses the `objc` and `cocoa` crates to bind with Objective-C.
//!
//! **This has not been tested properly yet.**
//!
//! Can be embedded onto audio plug-ins by getting the native webview handle and
//! adding it to a plug-ins native `NSWindowView` handle.
//!
//! **Closure captures are unsafe as we'll just pass pointers around.**
//!
//! The `darwin_webkit::foundation` module exposes some dependencies to using
//! the WKWebView APIs, like `NSURLRequest`.
//!
//! The `darwin_webkit::webkit` module exposes bindings to the `WKWebView` API.
//!
//! In `darwin_webkit::helpers` there's a very small higher level wrapper that
//! may turn into a higher level API.
//!
//! Callbacks from JavaScript to rust may be registered with:
//!
//! * `darwin_webkit::webkit::wk_script_message_handler::make_new_handler`
//! * or `darwin_webkit::helpers::DarwinWKWebView`
//!
//! Rust may evaluate JavaScript and HTML with:
//!
//! * `darwin_webkit::helpers::DarwinWKWebView::evaluate_javascript`
//! * `darwin_webkit::helpers::DarwinWKWebView::load_url`
//! * `darwin_webkit::helpers::DarwinWKWebView::load_html_string`
//!
//! ## High-level wrapper
//!
//! Raw bindings are exposed, but under the `darwin_webkit::helpers` module, some high-level
//! rust wrappers are provided.
//!
//! ### High-level Example
//!
//! ```rust
//! use darwin_webkit::helpers::dwk_app::DarwinWKApp;
//!
//! unsafe {
//!     let app = DarwinWKApp::new("Simple WebView");
//!     let webview = app.create_webview();
//!     webview.load_url("https://www.google.com.br");
//!     app.set_webview(&webview);
//!     // Then run with:
//!     // app.run();
//! }
//! ```
//!
//! ### High-level Communication example
//! ```rust
//! use cocoa::base::id;
//! use darwin_webkit::helpers::dwk_app::DarwinWKApp;
//! use std::rc::Rc;
//!
//! unsafe {
//!     let app = DarwinWKApp::new("Host an app");
//!     let webview = Rc::new(app.create_webview());
//!
//!     let callback = Box::into_raw(Box::new(Box::new(|_: id, _: id| {
//!         println!("JavaScript called rust!");
//!         webview.evaluate_javascript("document.body.innerHTML += ' -> response from rust';");
//!     })));
//!     webview.add_message_handler("hello", callback);
//!     webview.load_html_string(
//!         "
//!         <script>
//!             document.body.innerHTML += 'start';
//!             window.webkit.messageHandlers.hello.postMessage('hello');
//!             document.body.innerHTML += ' -> success';
//!         </script>
//!         ",
//!         "",
//!     );
//!
//!     app.set_webview(&webview);
//!     // Then run with:
//!     // app.run();
//! }
//! ```
//!
//! ## Missing
//!
//! - ~~Callbacks from JavaScript to Rust.~~
//! - TODOs in raw bindings
//! * `serverTrust` in `WKWebView`
//! * In `NSURLRequest`
//! * Working with a Cache Policy
//! * Accessing Request Components
//! * Getting Header Fields
//! * Controlling Request Behavior
//! * Accessing the Service Type
//! * Supporting Secure Coding
//! - ...
//!
//! ## Linting
//! ```bash
//! cargo clippy
//! ```

#![allow(non_snake_case)]

#[macro_use]
extern crate objc;

pub mod foundation;
pub mod helpers;
pub mod webkit;
