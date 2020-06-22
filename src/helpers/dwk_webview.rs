use cocoa::appkit::{NSView, NSViewHeightSizable, NSViewWidthSizable};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSString};

use foundation::*;
use webkit::*;

pub struct DarwinWKWebView {
    webview: id,
}

impl DarwinWKWebView {
    pub unsafe fn new(frame: NSRect) -> DarwinWKWebView {
        let configuration = WKWebViewConfiguration::init(WKWebViewConfiguration::alloc(nil));
        let webview = WKWebView::alloc(nil).initWithFrame_configuration_(frame, configuration);

        NSView::setAutoresizingMask_(webview, NSViewWidthSizable | NSViewHeightSizable);

        DarwinWKWebView { webview }
    }

    pub fn get_native_handle(&self) -> id {
        return self.webview;
    }

    pub unsafe fn load_url(&self, url: &str) {
        let url = NSString::alloc(nil).init_str(url);
        let url = NSURL::alloc(nil).initWithString_(url);
        let req = NSURLRequest::alloc(nil).initWithURL_(url);
        self.webview.loadRequest_(req);
    }
}
