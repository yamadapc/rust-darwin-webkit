use cocoa::base::{id, BOOL};
use cocoa::foundation::NSRect;
use core_graphics::base::CGFloat;
use core_graphics::geometry::CGPoint;
use libc::c_double;

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKWebView: id;
}

pub trait WKWebView: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(WKWebView), alloc]
    }

    // Determining Whether WebKit can Load Content
    unsafe fn handlesURLScheme_(_: Self, urlScheme: id) -> BOOL;

    // Initializing a Web View
    unsafe fn configuration(self) -> id;
    unsafe fn initWithFrame_configuration_(self, frameRect: NSRect, configuration: id) -> id;
    unsafe fn initWithCoder_(self, coder: id) -> id;

    // Inspecting the View Information
    unsafe fn scrollView(self) -> id;
    unsafe fn title(self) -> id;
    unsafe fn URL(self) -> id;
    unsafe fn customUserAgent(self) -> id;
    unsafe fn setCustomUserAgent_(self, customUserAgent: id);
    // Maybe there's setCustomerUserAgent too ???

    // TODO
    // unsafe fn serverTrust(self) -> SecTrustRef;

    // Setting Delegates
    unsafe fn navigationDelegate(self) -> id;
    unsafe fn setNavigationDelegate_(self, navigationDelegate: id);
    unsafe fn UIDelegate(self) -> id;
    unsafe fn setUIDelegate_(self, navigationDelegate: id);

    // Loading Content
    unsafe fn estimatedProgress(self) -> c_double;
    unsafe fn hasOnlySecureContent(self) -> BOOL;
    unsafe fn loadHTMLString_baseURL_(self, string: id, baseURL: id) -> id;
    unsafe fn loading(self) -> BOOL;
    unsafe fn reload_(self) -> id;
    unsafe fn reload_sender_(self, sender: id) -> id;
    unsafe fn reloadFromOrigin_(self) -> id;
    unsafe fn reloadFromOrigin_sender_(self, sender: id) -> id;
    unsafe fn stopLoading_(self);
    unsafe fn stopLoading_sender_(self, sender: id) -> id;
    unsafe fn loadData_MIMEType_characterEncodingName_baseURL_(
        self,
        data: id,
        MIMEType: id,
        characterEncodingName: id,
        baseURL: id,
    ) -> id;
    unsafe fn loadFileURL_allowingReadAccessToURL_(
        self,
        URL: id,
        allowingReadAccessToURL: id,
    ) -> id;

    // Scaling Content
    unsafe fn allowsMagnification(self) -> BOOL;
    unsafe fn setAllowsMagnification_(self, allowsMagnification: BOOL);
    unsafe fn magnification(self) -> CGFloat;
    unsafe fn setMagnification_(self, magnification: CGFloat);
    unsafe fn setMagnification_centeredAtPoint_(
        self,
        magnification: CGFloat,
        centeredAtPoint: CGPoint,
    );

    // Navigating
    unsafe fn allowsBackForwardNavigationGestures(self) -> BOOL;
    unsafe fn setAllowsBackForwardNavigationGestures_(
        self,
        allowsBackForwardNavigationGestures: BOOL,
    );
    unsafe fn backForwardList(self) -> id;
    unsafe fn canGoBack(self) -> BOOL;
    unsafe fn canGoForward(self) -> BOOL;
    unsafe fn allowsLinkPreview(self) -> BOOL;
    unsafe fn setAllowsLinkPreview_(self, allowsLinkPreview: BOOL);
    unsafe fn goBack_(self) -> id;
    unsafe fn goBack_sender_(self, sender: id) -> id;
    unsafe fn goForward_(self) -> id;
    unsafe fn goForward_sender_(self, sender: id) -> id;
    unsafe fn goToBackForwardListItem_(self, item: id) -> id;
    unsafe fn loadRequest_(self, request: id);

    // Executing JavaScript
    unsafe fn evaluateJavaScript_(
        self,
        javascriptString: id,
        completionHandler: extern "C" fn(id, id),
    );

    // Taking Snapshots
    unsafe fn takeSnapshotWithConfiguration_(
        self,
        snapshotConfiguration: id,
        completionHandler: extern "C" fn(id, id),
    );
}

impl WKWebView for id {
    // Determining Whether WebKit can Load Content
    unsafe fn handlesURLScheme_(_: Self, urlScheme: id) -> BOOL {
        msg_send![class!(WKWebView), handlesURLScheme: urlScheme]
    }

    // Initializing a Web View
    unsafe fn configuration(self) -> id {
        msg_send![self, configuration]
    }

    unsafe fn initWithFrame_configuration_(self, frameRect: NSRect, configuration: id) -> id {
        msg_send![self, initWithFrame:frameRect configuration:configuration]
    }

    unsafe fn initWithCoder_(self, coder: id) -> id {
        msg_send![self, initWithCoder: coder]
    }

    // Inspecting the View Information
    unsafe fn scrollView(self) -> id {
        msg_send![self, scrollView]
    }

    unsafe fn title(self) -> id {
        msg_send![self, title]
    }

    unsafe fn URL(self) -> id {
        msg_send![self, URL]
    }

    unsafe fn customUserAgent(self) -> id {
        msg_send![self, customUserAgent]
    }

    unsafe fn setCustomUserAgent_(self, customUserAgent: id) {
        msg_send![self, setCustomUserAgent: customUserAgent]
    }

    // Setting Delegates
    unsafe fn navigationDelegate(self) -> id {
        msg_send![self, navigationDelegate]
    }

    unsafe fn setNavigationDelegate_(self, navigationDelegate: id) {
        msg_send![self, setNavigationDelegate: navigationDelegate]
    }

    unsafe fn UIDelegate(self) -> id {
        msg_send![self, UIDelegate]
    }

    unsafe fn setUIDelegate_(self, navigationDelegate: id) {
        msg_send![self, setUIDelegate: navigationDelegate]
    }

    // Loading Content
    unsafe fn estimatedProgress(self) -> c_double {
        msg_send![self, estimatedProgress]
    }

    unsafe fn hasOnlySecureContent(self) -> BOOL {
        msg_send![self, hasOnlySecureContent]
    }

    unsafe fn loadHTMLString_baseURL_(self, string: id, baseURL: id) -> id {
        msg_send![self, loadHTMLString:string baseURL:baseURL]
    }

    unsafe fn loading(self) -> BOOL {
        msg_send![self, loading]
    }
    unsafe fn reload_(self) -> id {
        msg_send![self, reload]
    }

    unsafe fn reload_sender_(self, sender: id) -> id {
        msg_send![self, reload: sender]
    }

    unsafe fn reloadFromOrigin_(self) -> id {
        msg_send![self, reloadFromOrigin]
    }

    unsafe fn reloadFromOrigin_sender_(self, sender: id) -> id {
        msg_send![self, reloadFromOrigin: sender]
    }

    unsafe fn stopLoading_(self) {
        msg_send![self, stopLoading]
    }

    unsafe fn stopLoading_sender_(self, sender: id) -> id {
        msg_send![self, stopLoading: sender]
    }

    unsafe fn loadData_MIMEType_characterEncodingName_baseURL_(
        self,
        data: id,
        MIMEType: id,
        characterEncodingName: id,
        baseURL: id,
    ) -> id {
        msg_send![
            self,
            loadData:data
                MIMEType:MIMEType
                characterEncodingName:characterEncodingName
                baseURL:baseURL
        ]
    }

    unsafe fn loadFileURL_allowingReadAccessToURL_(
        self,
        URL: id,
        allowingReadAccessToURL: id,
    ) -> id {
        msg_send![
            self,
            loadFileURL:URL
                allowingReadAccessToURL:allowingReadAccessToURL
        ]
    }

    // Scaling Content
    unsafe fn allowsMagnification(self) -> BOOL {
        msg_send![self, allowsMagnification]
    }

    unsafe fn setAllowsMagnification_(self, allowsMagnification: BOOL) {
        msg_send![self, setAllowsMagnification: allowsMagnification]
    }

    unsafe fn magnification(self) -> CGFloat {
        msg_send![self, magnification]
    }

    unsafe fn setMagnification_(self, magnification: CGFloat) {
        msg_send![self, setMagnification: magnification]
    }

    unsafe fn setMagnification_centeredAtPoint_(
        self,
        magnification: CGFloat,
        centeredAtPoint: CGPoint,
    ) {
        msg_send![
            self,
            setMagnification:magnification
                centeredAtPoint:centeredAtPoint
        ]
    }

    // Navigating
    unsafe fn allowsBackForwardNavigationGestures(self) -> BOOL {
        msg_send![self, allowsBackForwardNavigationGestures]
    }

    unsafe fn setAllowsBackForwardNavigationGestures_(
        self,
        allowsBackForwardNavigationGestures: BOOL,
    ) {
        msg_send![
            self,
            setAllowsBackForwardNavigationGestures: allowsBackForwardNavigationGestures
        ]
    }

    unsafe fn backForwardList(self) -> id {
        msg_send![self, backForwardList]
    }

    unsafe fn canGoBack(self) -> BOOL {
        msg_send![self, canGoBack]
    }

    unsafe fn canGoForward(self) -> BOOL {
        msg_send![self, canGoForward]
    }

    unsafe fn allowsLinkPreview(self) -> BOOL {
        msg_send![self, allowsLinkPreview]
    }

    unsafe fn setAllowsLinkPreview_(self, allowsLinkPreview: BOOL) {
        msg_send![self, setAllowsLinkPreview: allowsLinkPreview]
    }

    unsafe fn goBack_(self) -> id {
        msg_send![self, goBack]
    }

    unsafe fn goBack_sender_(self, sender: id) -> id {
        msg_send![self, goBack: sender]
    }

    unsafe fn goForward_(self) -> id {
        msg_send![self, goForward]
    }

    unsafe fn goForward_sender_(self, sender: id) -> id {
        msg_send![self, goForward: sender]
    }

    unsafe fn goToBackForwardListItem_(self, item: id) -> id {
        msg_send![self, goToBackForwardListItem: item]
    }

    unsafe fn loadRequest_(self, request: id) {
        msg_send![self, loadRequest: request]
    }

    // Executing JavaScript
    unsafe fn evaluateJavaScript_(
        self,
        javascriptString: id,
        completionHandler: extern "C" fn(id, id),
    ) {
        msg_send![
            self,
            evaluateJavaScript:javascriptString
                completionHandler:completionHandler
        ]
    }

    // Taking Snapshots
    unsafe fn takeSnapshotWithConfiguration_(
        self,
        snapshotConfiguration: id,
        completionHandler: extern "C" fn(id, id),
    ) {
        msg_send![
            self,
            takeSnapshotWithConfiguration:snapshotConfiguration
                completionHandler:completionHandler
        ]
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use cocoa::base::nil;
//     use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSSize};
//     use webkit::wk_web_view_configuration::WKWebViewConfiguration;
//     #[test]
//     fn test_webview_configuration() {
//         unsafe {
//             let pool = NSAutoreleasePool::new(nil);
//             println!("Init frame");
//             let frame = NSRect::new(NSPoint::new(0., 0.), NSSize::new(800., 800.));
//             println!("Init configuration");
//             let configuration = WKWebViewConfiguration::init(WKWebViewConfiguration::alloc(nil));
//             println!("Init webview");
//             let webview = WKWebView::alloc(nil).initWithFrame_configuration_(frame, configuration);
//             println!("Assert");
//             assert!(webview.configuration() == configuration);
//             pool.autorelease();
//         }
//     }
// }
