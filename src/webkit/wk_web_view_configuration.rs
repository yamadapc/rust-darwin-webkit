use cocoa::base::id;

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKUserContentController: id;
}

pub trait WKUserContentController: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(WKUserContentController), alloc]
    }
    unsafe fn init(self) -> id;

    unsafe fn addScriptMessageHandler(self, message_handler: id, name: id) -> id;
}

impl WKUserContentController for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn addScriptMessageHandler(self, message_handler: id, name: id) -> id {
        msg_send![self, addScriptMessageHandler: message_handler name: name]
    }
}

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKWebViewConfiguration: id;
}

pub trait WKWebViewConfiguration: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(WKWebViewConfiguration), alloc]
    }
    unsafe fn init(self) -> id;

    unsafe fn setUserContentController(self, controller: id);
}

impl WKWebViewConfiguration for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn setUserContentController(self, controller: id) {
        msg_send![self, setUserContentController: controller]
    }
}
