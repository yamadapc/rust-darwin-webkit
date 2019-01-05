//! Bindings for `NSURL`
//!
//! https://developer.apple.com/documentation/foundation/nsurl?language=objc
use cocoa::base::{id, BOOL};

use foundation::ns_url_bookmark_creation_options::{
    NSURLBookmarkCreationOptions, NSURLBookmarkFileCreationOptions,
};
use foundation::ns_url_bookmark_resolution_options::NSURLBookmarkResolutionOptions;

pub trait NSURL: Sized {
    unsafe fn alloc(_: Self) -> id;

    unsafe fn URLWithString_(_: Self, string: id) -> id;
    unsafe fn initWithString_(self, string: id) -> id;
    unsafe fn URLWithString_relativeToURL_(_: Self, string: id, url: id) -> id;
    unsafe fn initWithString_relativeToURL_(self, string: id, url: id) -> id;
    unsafe fn fileURLWithPath_isDirectory_(_: Self, path: id, is_dir: BOOL) -> id;
    unsafe fn initFileURLWithPath_isDirectory_(self, path: id, is_dir: BOOL) -> id;
    unsafe fn fileURLWithPath_relativeToURL_(_: Self, path: id, url: id) -> id;
    unsafe fn initFileURLWithPath_relativeToURL_(self, path: id, url: id) -> id;
    unsafe fn fileURLWithPath_isDirectory_relativeToURL_(
        _: Self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id;
    unsafe fn initFileURLWithPath_isDirectory_relativeToURL_(
        self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id;
    unsafe fn fileURLWithPath_(_: Self, path: id) -> id;
    unsafe fn initFileURLWithPath_(self, path: id) -> id;
    unsafe fn fileURLWithPathComponents_(
        _: Self,
        path_components: id, /* (NSArray<NSString*>*) */
    ) -> id;
    unsafe fn URLByResolvingAliasFileAtURL_options_error_(
        _: Self,
        url: id,
        options: NSURLBookmarkResolutionOptions,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id;
    unsafe fn URLByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        _: Self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id;
    unsafe fn initByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id;
    // unsafe fn fileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    // unsafe fn getFileSystemRepresentation_maxLength_
    // unsafe fn initFileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    unsafe fn absoluteURLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    unsafe fn initAbsoluteURLWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    unsafe fn URLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    unsafe fn initWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    unsafe fn dataRepresentation(self) -> id /* (NSData) */;

    unsafe fn isEqual_(self, id: id) -> BOOL;

    unsafe fn checkResourceIsReachableAndReturnError_(
        self,
        error: id, /* (NSError _Nullable) */
    ) -> BOOL;
    unsafe fn isFileReferenceURL(self) -> BOOL;
    unsafe fn isFileURL(self) -> BOOL;

    unsafe fn absoluteString(self) -> id /* (NSString) */;
    unsafe fn absoluteURL(self) -> id /* (NSURL) */;
    unsafe fn baseURL(self) -> id /* (NSURL) */;
    // unsafe fn fileSystemRepresentation
    unsafe fn fragment(self) -> id /* (NSString) */;
    unsafe fn host(self) -> id /* (NSString) */;
    unsafe fn lastPathComponent(self) -> id /* (NSString) */;
    unsafe fn parameterString(self) -> id /* (NSString) */;
    unsafe fn password(self) -> id /* (NSString) */;
    unsafe fn path(self) -> id /* (NSString) */;
    unsafe fn pathComponents(self) -> id /* (NSArray<NSString*>) */;
    unsafe fn pathExtension(self) -> id /* (NSString) */;
    unsafe fn port(self) -> id /* (NSNumber) */;
    unsafe fn query(self) -> id /* (NSString) */;
    unsafe fn relativePath(self) -> id /* (NSString) */;
    unsafe fn relativeString(self) -> id /* (NSString) */;
    unsafe fn resourceSpecifier(self) -> id /* (NSString) */;
    unsafe fn scheme(self) -> id /* (NSString) */;
    unsafe fn standardizedURL(self) -> id /* (NSURL) */;
    unsafe fn user(self) -> id /* (NSString) */;

    // unsafe fn resourceValuesForKeys_error_
    // unsafe fn getResourceValue_forKey_error_
    // unsafe fn setResourceValue_forKey_error_
    // unsafe fn setResourceValues_error_
    // unsafe fn removeAllCachedResourceValues
    // unsafe fn removeCachedResourceValueForKey_
    // unsafe fn setTemporaryResourceValue_forKey_
    unsafe fn NSURLResourceKey(self) -> id /* (NSString) */;

    unsafe fn filePathURL(self) -> id;
    unsafe fn fileReferenceURL(self) -> id;
    unsafe fn URLByAppendingPathComponent_(self, path_component: id /* (NSString) */) -> id;
    unsafe fn URLByAppendingPathComponent_isDirectory_(
        self,
        path_component: id, /* (NSString) */
        is_dir: BOOL,
    ) -> id;
    unsafe fn URLByAppendingPathExtension_(self, extension: id /* (NSString) */) -> id;
    unsafe fn URLByDeletingLastPathComponent(self) -> id;
    unsafe fn URLByDeletingPathExtension(self) -> id;
    unsafe fn URLByResolvingSymlinksInPath(self) -> id;
    unsafe fn URLByStandardizingPath(self) -> id;
    unsafe fn hasDirectoryPath(self) -> BOOL;

    unsafe fn bookmarkDataWithContentsOfURL_error_(
        _: Self,
        url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */;
    unsafe fn bookmarkDataWithOptions_includingResourceValuesForKeys_relativeToURL_error_(
        self,
        options: NSURLBookmarkCreationOptions,
        resource_value_for_keys: id, /* (NSArray<NSURLResourceKey>) */
        relative_to_url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */;
    // unsafe fn resourceValuesForKeys_fromBookmarkData_
    unsafe fn writeBookmarkData_toURL_options_error_(
        _: Self,
        data: id, /* (NSData) */
        to_url: id,
        options: NSURLBookmarkFileCreationOptions,
        error: id, /* (NSError _Nullable) */
    ) -> id;
    unsafe fn startAccessingSecurityScopedResource(self) -> BOOL;
    unsafe fn stopAccessingSecurityScopedResource(self);
    unsafe fn NSURLBookmarkFileCreationOptions(self) -> NSURLBookmarkFileCreationOptions;
    unsafe fn NSURLBookmarkCreationOptions(self) -> NSURLBookmarkCreationOptions;
    unsafe fn NSURLBookmarkResolutionOptions(self) -> NSURLBookmarkResolutionOptions;

    // unsafe fn checkPromisedItemIsReachableAndReturnError_
    // unsafe fn getPromisedItemResourceValue_forKey_error_
    // unsafe fn promisedItemResourceValuesForKeys_error_

    // unsafe fn URLFromPasteboard_
    // unsafe fn writeToPasteboard_
}

impl NSURL for id {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSURL), alloc]
    }

    unsafe fn URLWithString_(_: Self, string: id) -> id {
        msg_send![class!(NSURL), URLWithString: string]
    }
    unsafe fn initWithString_(self, string: id) -> id {
        msg_send![self, initWithString: string]
    }
    unsafe fn URLWithString_relativeToURL_(_: Self, string: id, url: id) -> id {
        msg_send![class!(NSURL), URLWithString: string relativeToURL:url]
    }
    unsafe fn initWithString_relativeToURL_(self, string: id, url: id) -> id {
        msg_send![self, initWithString:string relativeToURL:url]
    }
    unsafe fn fileURLWithPath_isDirectory_(_: Self, path: id, is_dir: BOOL) -> id {
        msg_send![class!(NSURL), fileURLWithPath:path isDirectory:is_dir]
    }
    unsafe fn initFileURLWithPath_isDirectory_(self, path: id, is_dir: BOOL) -> id {
        msg_send![self, initFileURLWithPath:path isDirectory:is_dir]
    }
    unsafe fn fileURLWithPath_relativeToURL_(_: Self, path: id, url: id) -> id {
        msg_send![class!(NSURL), fileURLWithPath:path relativeToURL:url]
    }
    unsafe fn initFileURLWithPath_relativeToURL_(self, path: id, url: id) -> id {
        msg_send![self, initFileURLWithPath:path relativeToURL:url]
    }
    unsafe fn fileURLWithPath_isDirectory_relativeToURL_(
        _: Self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id {
        msg_send![class!(NSURL), fileURLWithPath:path isDirectory:is_dir relativeToURL:url]
    }
    unsafe fn initFileURLWithPath_isDirectory_relativeToURL_(
        self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id {
        msg_send![self, initFileURLWithPath:path isDirectory:is_dir relativeToURL:url]
    }
    unsafe fn fileURLWithPath_(_: Self, path: id) -> id {
        msg_send![class!(NSURL), fileURLWithPath: path]
    }
    unsafe fn initFileURLWithPath_(self, path: id) -> id {
        msg_send![self, initFileURLWithPath: path]
    }
    unsafe fn fileURLWithPathComponents_(
        _: Self,
        path_components: id, /* (NSArray<NSString*>*) */
    ) -> id {
        msg_send![class!(NSURL), fileURLWithPathComponents: path_components]
    }
    unsafe fn URLByResolvingAliasFileAtURL_options_error_(
        _: Self,
        url: id,
        options: NSURLBookmarkResolutionOptions,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![class!(NSURL), URLByResolvingAliasFileAtURL:url options:options error:error]
    }
    unsafe fn URLByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        _: Self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![class!(NSURL), URLByResolvingBookmarkData:data options:options relativeToURL:relative_to_url bookmarkDataIsStale:is_stale error:error]
    }
    unsafe fn initByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![self, initByResolvingBookmarkData:data options:options relativeToURL:relative_to_url bookmarkDataIsStale:is_stale error:error]
    }
    // unsafe fn fileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    // unsafe fn getFileSystemRepresentation_maxLength_
    // unsafe fn initFileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    unsafe fn absoluteURLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![class!(NSURL), absoluteURLWithDataRepresentation:data relativeToURL:url]
    }
    unsafe fn initAbsoluteURLWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![self, initAbsoluteURLWithDataRepresentation:data relativeToURL:url]
    }
    unsafe fn URLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![class!(NSURL), URLWithDataRepresentation:data relativeToURL:url]
    }
    unsafe fn initWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![self, initWithDataRepresentation:data relativeToURL:url]
    }
    unsafe fn dataRepresentation(self) -> id /* (NSData) */ {
        msg_send![self, dataRepresentation]
    }

    unsafe fn isEqual_(self, id: id) -> BOOL {
        msg_send![self, isEqual: id]
    }

    unsafe fn checkResourceIsReachableAndReturnError_(
        self,
        error: id, /* (NSError _Nullable) */
    ) -> BOOL {
        msg_send![self, checkResourceIsReachableAndReturnError: error]
    }
    unsafe fn isFileReferenceURL(self) -> BOOL {
        msg_send![self, isFileReferenceURL]
    }
    unsafe fn isFileURL(self) -> BOOL {
        msg_send![self, isFileURL]
    }

    unsafe fn absoluteString(self) -> id /* (NSString) */ {
        msg_send![self, absoluteString]
    }
    unsafe fn absoluteURL(self) -> id /* (NSURL) */ {
        msg_send![self, absoluteURL]
    }
    unsafe fn baseURL(self) -> id /* (NSURL) */ {
        msg_send![self, baseURL]
    }
    // unsafe fn fileSystemRepresentation
    unsafe fn fragment(self) -> id /* (NSString) */ {
        msg_send![self, fragment]
    }
    unsafe fn host(self) -> id /* (NSString) */ {
        msg_send![self, host]
    }
    unsafe fn lastPathComponent(self) -> id /* (NSString) */ {
        msg_send![self, lastPathComponent]
    }
    unsafe fn parameterString(self) -> id /* (NSString) */ {
        msg_send![self, parameterString]
    }
    unsafe fn password(self) -> id /* (NSString) */ {
        msg_send![self, password]
    }
    unsafe fn path(self) -> id /* (NSString) */ {
        msg_send![self, path]
    }
    unsafe fn pathComponents(self) -> id /* (NSArray<NSString*>) */ {
        msg_send![self, pathComponents]
    }
    unsafe fn pathExtension(self) -> id /* (NSString) */ {
        msg_send![self, pathExtension]
    }
    unsafe fn port(self) -> id /* (NSNumber) */ {
        msg_send![self, port]
    }
    unsafe fn query(self) -> id /* (NSString) */ {
        msg_send![self, query]
    }
    unsafe fn relativePath(self) -> id /* (NSString) */ {
        msg_send![self, relativePath]
    }
    unsafe fn relativeString(self) -> id /* (NSString) */ {
        msg_send![self, relativeString]
    }
    unsafe fn resourceSpecifier(self) -> id /* (NSString) */ {
        msg_send![self, resourceSpecifier]
    }
    unsafe fn scheme(self) -> id /* (NSString) */ {
        msg_send![self, scheme]
    }
    unsafe fn standardizedURL(self) -> id /* (NSURL) */ {
        msg_send![self, standardizedURL]
    }
    unsafe fn user(self) -> id /* (NSString) */ {
        msg_send![self, user]
    }

    // unsafe fn resourceValuesForKeys_error_
    // unsafe fn getResourceValue_forKey_error_
    // unsafe fn setResourceValue_forKey_error_
    // unsafe fn setResourceValues_error_
    // unsafe fn removeAllCachedResourceValues
    // unsafe fn removeCachedResourceValueForKey_
    // unsafe fn setTemporaryResourceValue_forKey_
    unsafe fn NSURLResourceKey(self) -> id /* (NSString) */ {
        msg_send![self, NSURLResourceKey]
    }

    unsafe fn filePathURL(self) -> id {
        msg_send![self, filePathURL]
    }
    unsafe fn fileReferenceURL(self) -> id {
        msg_send![self, fileReferenceURL]
    }
    unsafe fn URLByAppendingPathComponent_(self, path_component: id /* (NSString) */) -> id {
        msg_send![self, URLByAppendingPathComponent: path_component]
    }
    unsafe fn URLByAppendingPathComponent_isDirectory_(
        self,
        path_component: id, /* (NSString) */
        is_dir: BOOL,
    ) -> id {
        msg_send![self, URLByAppendingPathComponent:path_component isDirectory:is_dir]
    }
    unsafe fn URLByAppendingPathExtension_(self, extension: id /* (NSString) */) -> id {
        msg_send![self, URLByAppendingPathExtension: extension]
    }
    unsafe fn URLByDeletingLastPathComponent(self) -> id {
        msg_send![self, URLByDeletingLastPathComponent]
    }
    unsafe fn URLByDeletingPathExtension(self) -> id {
        msg_send![self, URLByDeletingPathExtension]
    }
    unsafe fn URLByResolvingSymlinksInPath(self) -> id {
        msg_send![self, URLByResolvingSymlinksInPath]
    }
    unsafe fn URLByStandardizingPath(self) -> id {
        msg_send![self, URLByStandardizingPath]
    }
    unsafe fn hasDirectoryPath(self) -> BOOL {
        msg_send![self, hasDirectoryPath]
    }

    unsafe fn bookmarkDataWithContentsOfURL_error_(
        _: Self,
        url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */ {
        msg_send![class!(NSURL), bookmarkDataWithContentsOfURL:url error:error]
    }
    unsafe fn bookmarkDataWithOptions_includingResourceValuesForKeys_relativeToURL_error_(
        self,
        options: NSURLBookmarkCreationOptions,
        resource_value_for_keys: id, /* (NSArray<NSURLResourceKey>) */
        relative_to_url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */ {
        msg_send![self, bookmarkDataWithOptions:options includingResourceValuesForKeys:resource_value_for_keys relativeToURL:relative_to_url error:error]
    }
    // unsafe fn resourceValuesForKeys_fromBookmarkData_
    unsafe fn writeBookmarkData_toURL_options_error_(
        _: Self,
        data: id, /* (NSData) */
        to_url: id,
        options: NSURLBookmarkFileCreationOptions,
        error: id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![class!(NSURL), writeBookmarkData:data toURL:to_url options:options error:error]
    }
    unsafe fn startAccessingSecurityScopedResource(self) -> BOOL {
        msg_send![self, startAccessingSecurityScopedResource]
    }
    unsafe fn stopAccessingSecurityScopedResource(self) {
        msg_send![self, stopAccessingSecurityScopedResource]
    }
    unsafe fn NSURLBookmarkFileCreationOptions(self) -> NSURLBookmarkFileCreationOptions {
        msg_send![self, NSURLBookmarkFileCreationOptions]
    }
    unsafe fn NSURLBookmarkCreationOptions(self) -> NSURLBookmarkCreationOptions {
        msg_send![self, NSURLBookmarkCreationOptions]
    }
    unsafe fn NSURLBookmarkResolutionOptions(self) -> NSURLBookmarkResolutionOptions {
        msg_send![self, NSURLBookmarkResolutionOptions]
    }

    // unsafe fn checkPromisedItemIsReachableAndReturnError_
    // unsafe fn getPromisedItemResourceValue_forKey_error_
    // unsafe fn promisedItemResourceValuesForKeys_error_

    // unsafe fn URLFromPasteboard_
    // unsafe fn writeToPasteboard_
}
