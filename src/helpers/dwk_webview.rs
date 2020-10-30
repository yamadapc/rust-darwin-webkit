use cocoa::appkit::{NSView, NSViewHeightSizable, NSViewWidthSizable};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSString};

use block::ConcreteBlock;
use foundation::*;
use webkit::wk_script_message_handler::make_new_handler;
use webkit::*;

pub struct DarwinWKWebView {
    webview: id,
    configuration: id,
    content_controller: id,
}

impl DarwinWKWebView {
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn new(frame: NSRect) -> DarwinWKWebView {
        let configuration = WKWebViewConfiguration::init(WKWebViewConfiguration::alloc(nil));
        let content_controller = WKUserContentController::init(WKUserContentController::alloc(nil));
        configuration.setUserContentController(content_controller);
        let webview = WKWebView::alloc(nil).initWithFrame_configuration_(frame, configuration);

        NSView::setAutoresizingMask_(webview, NSViewWidthSizable | NSViewHeightSizable);

        DarwinWKWebView {
            webview,
            configuration,
            content_controller,
        }
    }

    pub fn get_native_handle(&self) -> id {
        self.webview
    }

    pub fn get_user_content_controller_handle(&self) -> id {
        self.content_controller
    }

    pub fn get_configuration_handle(&self) -> id {
        self.configuration
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn load_url(&self, url: &str) {
        let url = NSString::alloc(nil).init_str(url);
        let url = NSURL::alloc(nil).initWithString_(url);
        let req = NSURLRequest::alloc(nil).initWithURL_(url);
        self.webview.loadRequest_(req);
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn load_html_string(&self, html: &str, base_url: &str) {
        let html = NSString::alloc(nil).init_str(html);
        let base_url = NSString::alloc(nil).init_str(base_url);
        let base_url = NSURL::alloc(nil).initWithString_(base_url);
        self.webview.loadHTMLString_baseURL_(html, base_url);
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn evaluate_javascript(&self, javascript: &str) {
        let javascript = NSString::alloc(nil).init_str(javascript);
        let b = |_: id, _: id| {};
        let b = ConcreteBlock::new(b);
        let b = b.copy();
        self.webview.evaluateJavaScript_(javascript, &b);
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn add_message_handler<'a, Func>(&'a self, name: &str, callback: &'a mut Func)
    where
        Func: FnMut(id, id),
    {
        let handler = make_new_handler(format!("DWKHandler_{}", name).as_str(), callback);
        let name = NSString::alloc(nil).init_str(name);
        self.content_controller
            .addScriptMessageHandler(handler, name);
    }
}

pub extern "C" fn javascript_callback(_: id, _: id) {}
