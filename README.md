# rust-darwin-webkit
**darwin_webkit** exposes bindings to some of the WebKit's API on MacOS for
Rust. It uses the `objc` and `cocoa` crates to bind with Objective-C.

## Missing

- ~~Callbacks from JavaScript to Rust.~~
- TODOs in raw bindings
  * `serverTrust` in `WKWebView`
  * In `NSURLRequest`
    * Working with a Cache Policy
    * Accessing Request Components
    * Getting Header Fields
    * Controlling Request Behavior
    * Accessing the Service Type
    * Supporting Secure Coding
- ...
