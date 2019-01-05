use cocoa::base::id;

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKNavigation: id;
}

pub trait WKNavigation: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(WKNavigation), alloc]
    }
}
