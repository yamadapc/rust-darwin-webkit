(function() {var implementors = {};
implementors["block"] = [{"text":"impl&lt;A, R&gt; !Send for Block&lt;A, R&gt;","synthetic":true,"types":[]},{"text":"impl&lt;A, R&gt; !Send for RcBlock&lt;A, R&gt;","synthetic":true,"types":[]},{"text":"impl&lt;A, R, F&gt; !Send for ConcreteBlock&lt;A, R, F&gt;","synthetic":true,"types":[]}];
implementors["cocoa"] = [{"text":"impl Send for NSApplicationPresentationOptions","synthetic":true,"types":[]},{"text":"impl Send for NSWindowStyleMask","synthetic":true,"types":[]},{"text":"impl Send for NSWindowOrderingMode","synthetic":true,"types":[]},{"text":"impl Send for NSAlignmentOptions","synthetic":true,"types":[]},{"text":"impl Send for NSWindowCollectionBehavior","synthetic":true,"types":[]},{"text":"impl Send for NSWindowOcclusionState","synthetic":true,"types":[]},{"text":"impl Send for NSEventSwipeTrackingOptions","synthetic":true,"types":[]},{"text":"impl Send for NSEventPhase","synthetic":true,"types":[]},{"text":"impl Send for NSTouchPhase","synthetic":true,"types":[]},{"text":"impl Send for NSEventMask","synthetic":true,"types":[]},{"text":"impl Send for NSEventModifierFlags","synthetic":true,"types":[]},{"text":"impl Send for NSApplicationActivationPolicy","synthetic":true,"types":[]},{"text":"impl Send for NSApplicationActivationOptions","synthetic":true,"types":[]},{"text":"impl Send for NSApplicationTerminateReply","synthetic":true,"types":[]},{"text":"impl Send for NSWindowTitleVisibility","synthetic":true,"types":[]},{"text":"impl Send for NSBackingStoreType","synthetic":true,"types":[]},{"text":"impl Send for NSOpenGLPixelFormatAttribute","synthetic":true,"types":[]},{"text":"impl Send for NSOpenGLPFAOpenGLProfiles","synthetic":true,"types":[]},{"text":"impl Send for NSOpenGLContextParameter","synthetic":true,"types":[]},{"text":"impl Send for NSWindowButton","synthetic":true,"types":[]},{"text":"impl Send for NSBezelStyle","synthetic":true,"types":[]},{"text":"impl Send for NSRequestUserAttentionType","synthetic":true,"types":[]},{"text":"impl Send for NSPasteboardReadingOptions","synthetic":true,"types":[]},{"text":"impl Send for NSPasteboardWritingOptions","synthetic":true,"types":[]},{"text":"impl Send for NSEventGestureAxis","synthetic":true,"types":[]},{"text":"impl Send for NSEventType","synthetic":true,"types":[]},{"text":"impl Send for NSPointingDeviceType","synthetic":true,"types":[]},{"text":"impl Send for NSEventButtonMask","synthetic":true,"types":[]},{"text":"impl Send for NSEventSubtype","synthetic":true,"types":[]},{"text":"impl Send for NSCompositingOperation","synthetic":true,"types":[]},{"text":"impl Send for NSImageCacheMode","synthetic":true,"types":[]},{"text":"impl Send for NSTIFFCompression","synthetic":true,"types":[]},{"text":"impl Send for NSImageLoadStatus","synthetic":true,"types":[]},{"text":"impl Send for NSTabViewType","synthetic":true,"types":[]},{"text":"impl Send for NSTabState","synthetic":true,"types":[]},{"text":"impl Send for NSPoint","synthetic":true,"types":[]},{"text":"impl Send for NSSize","synthetic":true,"types":[]},{"text":"impl Send for NSRect","synthetic":true,"types":[]},{"text":"impl Send for NSRange","synthetic":true,"types":[]},{"text":"impl Send for NSEnumerationOptions","synthetic":true,"types":[]},{"text":"impl !Send for NSFastIterator","synthetic":true,"types":[]},{"text":"impl Send for NSDataReadingOptions","synthetic":true,"types":[]},{"text":"impl Send for NSDataBase64EncodingOptions","synthetic":true,"types":[]},{"text":"impl Send for NSDataBase64DecodingOptions","synthetic":true,"types":[]},{"text":"impl Send for NSDataWritingOptions","synthetic":true,"types":[]},{"text":"impl Send for NSDataSearchOptions","synthetic":true,"types":[]},{"text":"impl Send for NSRectEdge","synthetic":true,"types":[]},{"text":"impl Send for NSComparisonResult","synthetic":true,"types":[]},{"text":"impl Send for EdgeAntialiasingMask","synthetic":true,"types":[]},{"text":"impl Send for CornerMask","synthetic":true,"types":[]},{"text":"impl Send for AutoresizingMask","synthetic":true,"types":[]},{"text":"impl Send for CATransform3D","synthetic":true,"types":[]},{"text":"impl Send for CVTimeStamp","synthetic":true,"types":[]},{"text":"impl Send for CVSMPTETime","synthetic":true,"types":[]},{"text":"impl !Send for ContentsGravity","synthetic":true,"types":[]},{"text":"impl !Send for ContentsFormat","synthetic":true,"types":[]},{"text":"impl !Send for Filter","synthetic":true,"types":[]},{"text":"impl Send for CALayer","synthetic":false,"types":[]},{"text":"impl Send for CARenderer","synthetic":false,"types":[]}];
implementors["core_foundation"] = [{"text":"impl&lt;T&nbsp;=&nbsp;*const c_void&gt; !Send for CFArray&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !Send for CFArrayIterator&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl !Send for CFAttributedString","synthetic":true,"types":[]},{"text":"impl !Send for CFMutableAttributedString","synthetic":true,"types":[]},{"text":"impl !Send for CFType","synthetic":true,"types":[]},{"text":"impl !Send for CFAllocator","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Send for ItemRef&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Send for ItemMutRef&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl !Send for CFBoolean","synthetic":true,"types":[]},{"text":"impl !Send for CFData","synthetic":true,"types":[]},{"text":"impl !Send for CFDate","synthetic":true,"types":[]},{"text":"impl&lt;K&nbsp;=&nbsp;*const c_void, V&nbsp;=&nbsp;*const c_void&gt; !Send for CFDictionary&lt;K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;K&nbsp;=&nbsp;*const c_void, V&nbsp;=&nbsp;*const c_void&gt; !Send for CFMutableDictionary&lt;K, V&gt;","synthetic":true,"types":[]},{"text":"impl !Send for CFError","synthetic":true,"types":[]},{"text":"impl !Send for CFFileDescriptor","synthetic":true,"types":[]},{"text":"impl !Send for CFNumber","synthetic":true,"types":[]},{"text":"impl&lt;T&nbsp;=&nbsp;*const c_void&gt; !Send for CFSet&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl !Send for CFString","synthetic":true,"types":[]},{"text":"impl !Send for CFURL","synthetic":true,"types":[]},{"text":"impl !Send for CFBundle","synthetic":true,"types":[]},{"text":"impl !Send for CFPropertyList","synthetic":true,"types":[]},{"text":"impl !Send for CFRunLoop","synthetic":true,"types":[]},{"text":"impl !Send for CFRunLoopTimer","synthetic":true,"types":[]},{"text":"impl !Send for CFRunLoopSource","synthetic":true,"types":[]},{"text":"impl !Send for CFRunLoopObserver","synthetic":true,"types":[]},{"text":"impl !Send for CFTimeZone","synthetic":true,"types":[]},{"text":"impl !Send for CFUUID","synthetic":true,"types":[]}];
implementors["core_foundation_sys"] = [{"text":"impl Send for CFArrayCallBacks","synthetic":true,"types":[]},{"text":"impl Send for __CFArray","synthetic":true,"types":[]},{"text":"impl Send for __CFAttributedString","synthetic":true,"types":[]},{"text":"impl Send for CFRange","synthetic":true,"types":[]},{"text":"impl !Send for CFAllocatorContext","synthetic":true,"types":[]},{"text":"impl Send for CFComparisonResult","synthetic":true,"types":[]},{"text":"impl Send for __CFBundle","synthetic":true,"types":[]},{"text":"impl Send for __CFData","synthetic":true,"types":[]},{"text":"impl Send for __CFDate","synthetic":true,"types":[]},{"text":"impl Send for CFDictionaryKeyCallBacks","synthetic":true,"types":[]},{"text":"impl Send for CFDictionaryValueCallBacks","synthetic":true,"types":[]},{"text":"impl Send for __CFDictionary","synthetic":true,"types":[]},{"text":"impl Send for __CFError","synthetic":true,"types":[]},{"text":"impl Send for __CFFileDescriptor","synthetic":true,"types":[]},{"text":"impl !Send for CFFileDescriptorContext","synthetic":true,"types":[]},{"text":"impl !Send for CFMessagePortContext","synthetic":true,"types":[]},{"text":"impl Send for __CFMessagePort","synthetic":true,"types":[]},{"text":"impl Send for __CFBoolean","synthetic":true,"types":[]},{"text":"impl Send for __CFNumber","synthetic":true,"types":[]},{"text":"impl Send for __CFRunLoop","synthetic":true,"types":[]},{"text":"impl Send for __CFRunLoopSource","synthetic":true,"types":[]},{"text":"impl Send for __CFRunLoopObserver","synthetic":true,"types":[]},{"text":"impl !Send for CFRunLoopSourceContext","synthetic":true,"types":[]},{"text":"impl !Send for CFRunLoopSourceContext1","synthetic":true,"types":[]},{"text":"impl !Send for CFRunLoopObserverContext","synthetic":true,"types":[]},{"text":"impl !Send for CFRunLoopTimerContext","synthetic":true,"types":[]},{"text":"impl Send for __CFRunLoopTimer","synthetic":true,"types":[]},{"text":"impl !Send for CFSetCallBacks","synthetic":true,"types":[]},{"text":"impl Send for __CFSet","synthetic":true,"types":[]},{"text":"impl Send for __CFString","synthetic":true,"types":[]},{"text":"impl Send for __CFTimeZone","synthetic":true,"types":[]},{"text":"impl Send for __CFURL","synthetic":true,"types":[]},{"text":"impl Send for __CFUUID","synthetic":true,"types":[]},{"text":"impl Send for CFUUIDBytes","synthetic":true,"types":[]}];
implementors["core_graphics"] = [{"text":"impl !Send for CGColor","synthetic":true,"types":[]},{"text":"impl !Send for CGColorSpace","synthetic":true,"types":[]},{"text":"impl Send for CGColorSpaceRef","synthetic":true,"types":[]},{"text":"impl !Send for CGContext","synthetic":true,"types":[]},{"text":"impl Send for CGContextRef","synthetic":true,"types":[]},{"text":"impl Send for CGBlendMode","synthetic":true,"types":[]},{"text":"impl Send for CGTextDrawingMode","synthetic":true,"types":[]},{"text":"impl !Send for CGDataProvider","synthetic":true,"types":[]},{"text":"impl Send for CGDataProviderRef","synthetic":true,"types":[]},{"text":"impl Send for CGDisplay","synthetic":true,"types":[]},{"text":"impl !Send for CGDisplayMode","synthetic":true,"types":[]},{"text":"impl Send for CGDisplayModeRef","synthetic":true,"types":[]},{"text":"impl Send for CGConfigureOption","synthetic":true,"types":[]},{"text":"impl Send for CGEventFlags","synthetic":true,"types":[]},{"text":"impl Send for KeyCode","synthetic":true,"types":[]},{"text":"impl Send for ScrollEventUnit","synthetic":true,"types":[]},{"text":"impl Send for EventField","synthetic":true,"types":[]},{"text":"impl !Send for CGEvent","synthetic":true,"types":[]},{"text":"impl Send for CGEventRef","synthetic":true,"types":[]},{"text":"impl Send for CGEventType","synthetic":true,"types":[]},{"text":"impl Send for CGMouseButton","synthetic":true,"types":[]},{"text":"impl Send for CGEventTapLocation","synthetic":true,"types":[]},{"text":"impl !Send for CGEventSource","synthetic":true,"types":[]},{"text":"impl Send for CGEventSourceRef","synthetic":true,"types":[]},{"text":"impl Send for CGEventSourceStateID","synthetic":true,"types":[]},{"text":"impl Send for CGFontRef","synthetic":true,"types":[]},{"text":"impl Send for CGSize","synthetic":true,"types":[]},{"text":"impl Send for CGPoint","synthetic":true,"types":[]},{"text":"impl Send for CGRect","synthetic":true,"types":[]},{"text":"impl Send for CGAffineTransform","synthetic":true,"types":[]},{"text":"impl !Send for CGSRegion","synthetic":true,"types":[]},{"text":"impl Send for CGSSurface","synthetic":true,"types":[]},{"text":"impl !Send for CGImage","synthetic":true,"types":[]},{"text":"impl Send for CGImageRef","synthetic":true,"types":[]},{"text":"impl Send for CGImageAlphaInfo","synthetic":true,"types":[]},{"text":"impl Send for CGImageByteOrderInfo","synthetic":true,"types":[]},{"text":"impl !Send for CGPath","synthetic":true,"types":[]},{"text":"impl Send for CGPathRef","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !Send for CGPathElementRef&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl !Send for CGPathElement","synthetic":true,"types":[]},{"text":"impl Send for CGPathElementType","synthetic":true,"types":[]},{"text":"impl Send for CGFont","synthetic":false,"types":[]}];
implementors["darwin_webkit"] = [{"text":"impl Send for NSURLBookmarkCreationOptions","synthetic":true,"types":[]},{"text":"impl Send for NSURLBookmarkResolutionOptions","synthetic":true,"types":[]},{"text":"impl Send for DarwinWKApp","synthetic":false,"types":[]},{"text":"impl Send for DarwinWKWebView","synthetic":false,"types":[]}];
implementors["dispatch"] = [{"text":"impl Send for GroupGuard","synthetic":true,"types":[]},{"text":"impl Send for Once","synthetic":true,"types":[]},{"text":"impl Send for SuspendGuard","synthetic":true,"types":[]},{"text":"impl Send for SemaphoreGuard","synthetic":true,"types":[]},{"text":"impl Send for WaitTimeout","synthetic":true,"types":[]},{"text":"impl Send for QueueAttribute","synthetic":true,"types":[]},{"text":"impl Send for QueuePriority","synthetic":true,"types":[]},{"text":"impl Send for dispatch_object_s","synthetic":true,"types":[]},{"text":"impl Send for Group","synthetic":false,"types":[]},{"text":"impl Send for Queue","synthetic":false,"types":[]},{"text":"impl Send for Semaphore","synthetic":false,"types":[]}];
implementors["foreign_types_shared"] = [{"text":"impl Send for Opaque","synthetic":true,"types":[]}];
implementors["libc"] = [{"text":"impl !Send for group","synthetic":true,"types":[]},{"text":"impl Send for utimbuf","synthetic":true,"types":[]},{"text":"impl Send for timeval","synthetic":true,"types":[]},{"text":"impl Send for timespec","synthetic":true,"types":[]},{"text":"impl Send for rlimit","synthetic":true,"types":[]},{"text":"impl Send for rusage","synthetic":true,"types":[]},{"text":"impl Send for ipv6_mreq","synthetic":true,"types":[]},{"text":"impl !Send for hostent","synthetic":true,"types":[]},{"text":"impl !Send for iovec","synthetic":true,"types":[]},{"text":"impl Send for pollfd","synthetic":true,"types":[]},{"text":"impl Send for winsize","synthetic":true,"types":[]},{"text":"impl Send for linger","synthetic":true,"types":[]},{"text":"impl !Send for sigval","synthetic":true,"types":[]},{"text":"impl Send for itimerval","synthetic":true,"types":[]},{"text":"impl Send for tms","synthetic":true,"types":[]},{"text":"impl !Send for servent","synthetic":true,"types":[]},{"text":"impl !Send for protoent","synthetic":true,"types":[]},{"text":"impl Send for sockaddr","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_in6","synthetic":true,"types":[]},{"text":"impl !Send for passwd","synthetic":true,"types":[]},{"text":"impl !Send for ifaddrs","synthetic":true,"types":[]},{"text":"impl Send for fd_set","synthetic":true,"types":[]},{"text":"impl !Send for tm","synthetic":true,"types":[]},{"text":"impl !Send for msghdr","synthetic":true,"types":[]},{"text":"impl Send for cmsghdr","synthetic":true,"types":[]},{"text":"impl Send for fsid_t","synthetic":true,"types":[]},{"text":"impl !Send for if_nameindex","synthetic":true,"types":[]},{"text":"impl !Send for regex_t","synthetic":true,"types":[]},{"text":"impl Send for regmatch_t","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_un","synthetic":true,"types":[]},{"text":"impl Send for utsname","synthetic":true,"types":[]},{"text":"impl Send for ip_mreq","synthetic":true,"types":[]},{"text":"impl !Send for aiocb","synthetic":true,"types":[]},{"text":"impl !Send for glob_t","synthetic":true,"types":[]},{"text":"impl !Send for addrinfo","synthetic":true,"types":[]},{"text":"impl Send for mach_timebase_info","synthetic":true,"types":[]},{"text":"impl Send for stat","synthetic":true,"types":[]},{"text":"impl Send for pthread_mutexattr_t","synthetic":true,"types":[]},{"text":"impl Send for pthread_condattr_t","synthetic":true,"types":[]},{"text":"impl Send for pthread_rwlockattr_t","synthetic":true,"types":[]},{"text":"impl !Send for siginfo_t","synthetic":true,"types":[]},{"text":"impl Send for sigaction","synthetic":true,"types":[]},{"text":"impl !Send for stack_t","synthetic":true,"types":[]},{"text":"impl Send for fstore_t","synthetic":true,"types":[]},{"text":"impl Send for radvisory","synthetic":true,"types":[]},{"text":"impl Send for statvfs","synthetic":true,"types":[]},{"text":"impl !Send for Dl_info","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_in","synthetic":true,"types":[]},{"text":"impl Send for kevent64_s","synthetic":true,"types":[]},{"text":"impl Send for dqblk","synthetic":true,"types":[]},{"text":"impl Send for if_msghdr","synthetic":true,"types":[]},{"text":"impl Send for termios","synthetic":true,"types":[]},{"text":"impl Send for flock","synthetic":true,"types":[]},{"text":"impl !Send for sf_hdtr","synthetic":true,"types":[]},{"text":"impl !Send for lconv","synthetic":true,"types":[]},{"text":"impl Send for proc_taskinfo","synthetic":true,"types":[]},{"text":"impl Send for proc_bsdinfo","synthetic":true,"types":[]},{"text":"impl Send for proc_taskallinfo","synthetic":true,"types":[]},{"text":"impl Send for xsw_usage","synthetic":true,"types":[]},{"text":"impl Send for xucred","synthetic":true,"types":[]},{"text":"impl Send for mach_header","synthetic":true,"types":[]},{"text":"impl Send for mach_header_64","synthetic":true,"types":[]},{"text":"impl Send for segment_command","synthetic":true,"types":[]},{"text":"impl Send for segment_command_64","synthetic":true,"types":[]},{"text":"impl Send for load_command","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_dl","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_inarp","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_ctl","synthetic":true,"types":[]},{"text":"impl Send for in_pktinfo","synthetic":true,"types":[]},{"text":"impl Send for in6_pktinfo","synthetic":true,"types":[]},{"text":"impl Send for ipc_perm","synthetic":true,"types":[]},{"text":"impl Send for sembuf","synthetic":true,"types":[]},{"text":"impl Send for arphdr","synthetic":true,"types":[]},{"text":"impl Send for in_addr","synthetic":true,"types":[]},{"text":"impl !Send for sa_endpoints_t","synthetic":true,"types":[]},{"text":"impl Send for timex","synthetic":true,"types":[]},{"text":"impl Send for ntptimeval","synthetic":true,"types":[]},{"text":"impl !Send for kevent","synthetic":true,"types":[]},{"text":"impl Send for semid_ds","synthetic":true,"types":[]},{"text":"impl !Send for shmid_ds","synthetic":true,"types":[]},{"text":"impl Send for proc_threadinfo","synthetic":true,"types":[]},{"text":"impl Send for statfs","synthetic":true,"types":[]},{"text":"impl Send for dirent","synthetic":true,"types":[]},{"text":"impl Send for pthread_rwlock_t","synthetic":true,"types":[]},{"text":"impl Send for pthread_mutex_t","synthetic":true,"types":[]},{"text":"impl Send for pthread_cond_t","synthetic":true,"types":[]},{"text":"impl Send for sockaddr_storage","synthetic":true,"types":[]},{"text":"impl Send for utmpx","synthetic":true,"types":[]},{"text":"impl !Send for sigevent","synthetic":true,"types":[]},{"text":"impl Send for timeval32","synthetic":true,"types":[]},{"text":"impl Send for if_data","synthetic":true,"types":[]},{"text":"impl Send for bpf_hdr","synthetic":true,"types":[]},{"text":"impl !Send for ucontext_t","synthetic":true,"types":[]},{"text":"impl Send for __darwin_mcontext64","synthetic":true,"types":[]},{"text":"impl Send for __darwin_x86_exception_state64","synthetic":true,"types":[]},{"text":"impl Send for __darwin_x86_thread_state64","synthetic":true,"types":[]},{"text":"impl Send for __darwin_x86_float_state64","synthetic":true,"types":[]},{"text":"impl Send for __darwin_mmst_reg","synthetic":true,"types":[]},{"text":"impl Send for __darwin_xmm_reg","synthetic":true,"types":[]},{"text":"impl Send for pthread_attr_t","synthetic":true,"types":[]},{"text":"impl Send for max_align_t","synthetic":true,"types":[]},{"text":"impl Send for in6_addr","synthetic":true,"types":[]},{"text":"impl !Send for semun","synthetic":true,"types":[]},{"text":"impl Send for DIR","synthetic":true,"types":[]},{"text":"impl Send for FILE","synthetic":true,"types":[]},{"text":"impl Send for fpos_t","synthetic":true,"types":[]},{"text":"impl Send for timezone","synthetic":true,"types":[]}];
implementors["malloc_buf"] = [{"text":"impl&lt;T&gt; !Send for MallocBuffer&lt;T&gt;","synthetic":true,"types":[]}];
implementors["objc"] = [{"text":"impl !Send for Encoding","synthetic":true,"types":[]},{"text":"impl Send for MessageError","synthetic":true,"types":[]},{"text":"impl Send for Ivar","synthetic":true,"types":[]},{"text":"impl Send for Method","synthetic":true,"types":[]},{"text":"impl Send for Class","synthetic":true,"types":[]},{"text":"impl Send for Protocol","synthetic":true,"types":[]},{"text":"impl Send for Object","synthetic":true,"types":[]},{"text":"impl !Send for ClassDecl","synthetic":true,"types":[]},{"text":"impl !Send for ProtocolDecl","synthetic":true,"types":[]},{"text":"impl !Send for StrongPtr","synthetic":true,"types":[]},{"text":"impl !Send for WeakPtr","synthetic":true,"types":[]},{"text":"impl Send for Sel","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()