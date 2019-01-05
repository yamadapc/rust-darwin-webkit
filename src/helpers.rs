//! Higher-level wrapper.
//!
//! Exposes two structs:
//!
//! `DarwinWKApp`, which is not to be used seriously but
//! nice for testing. This configures the `NSApplication` and opens a
//! `NSWindow`.
//!
//! `DarwinWKWebView` wraps `WKWebView`, but unlike generic wrappers, allows
//! for it to be used in the context of an existing window (for example, in vst
//! plugins).
use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSBackingStoreBuffered, NSMenu, NSMenuItem,
    NSRunningApplication, NSView, NSViewHeightSizable, NSViewWidthSizable, NSWindow,
    NSWindowStyleMask,
};
use cocoa::base::{id, nil, selector, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};

use foundation::*;
use webkit::*;

pub struct DarwinWKApp {
    pub nsapp: id,
    pub main_window: id,
}

impl DarwinWKApp {
    pub unsafe fn new(windowTitle: &str) -> DarwinWKApp {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();

        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title =
            quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        // create Window
        let styleMask = NSWindowStyleMask::NSTitledWindowMask
            | NSWindowStyleMask::NSClosableWindowMask
            | NSWindowStyleMask::NSResizableWindowMask;

        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(800., 800.)),
                styleMask,
                NSBackingStoreBuffered,
                NO,
            ).autorelease();
        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.center();

        let title = NSString::alloc(nil).init_str(windowTitle);
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);

        return DarwinWKApp {
            nsapp: app,
            main_window: window,
        };
    }

    pub unsafe fn run(&self) {
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        self.nsapp.run();
    }

    pub unsafe fn create_webview(&self) -> DarwinWKWebView {
        let frame = NSWindow::frame(self.main_window);
        DarwinWKWebView::new(frame)
    }

    pub unsafe fn set_webview(&self, webview: &DarwinWKWebView) {
        self.main_window.setContentView_(webview.webview);
    }
}

pub struct DarwinWKWebView {
    webview: id,
}

impl DarwinWKWebView {
    pub unsafe fn new(frame: NSRect) -> DarwinWKWebView {
        let configuration = WKWebViewConfiguration::init(
            WKWebViewConfiguration::alloc(nil)
        );
        let webview = WKWebView::alloc(nil)
            .initWithFrame_configuration_(frame, configuration);

        NSView::setAutoresizingMask_(webview, NSViewWidthSizable | NSViewHeightSizable);

        DarwinWKWebView {
            webview,
        }
    }

    pub unsafe fn load_url(&self, url: &str) {
        let url = NSString::alloc(nil).init_str(url);
        let url = NSURL::alloc(nil).initWithString_(url);
        let req = NSURLRequest::alloc(nil).initWithURL_(url);
        self.webview.loadRequest_(req);
    }
}
