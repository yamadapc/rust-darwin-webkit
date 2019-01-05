use cocoa::base::id;

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKWebViewConfiguration: id;
}

pub trait WKWebViewConfiguration: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(WKWebViewConfiguration), alloc]
    }

    unsafe fn init(self) -> id;
}

impl WKWebViewConfiguration for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }
}
