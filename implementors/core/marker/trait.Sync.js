(function() {var implementors = {};
implementors["block"] = [{"text":"impl&lt;A, R&gt; !Sync for Block&lt;A, R&gt;","synthetic":true,"types":[]},{"text":"impl&lt;A, R&gt; !Sync for RcBlock&lt;A, R&gt;","synthetic":true,"types":[]},{"text":"impl&lt;A, R, F&gt; !Sync for ConcreteBlock&lt;A, R, F&gt;","synthetic":true,"types":[]}];
implementors["cocoa"] = [{"text":"impl Sync for NSApplicationPresentationOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSWindowStyleMask","synthetic":true,"types":[]},{"text":"impl Sync for NSWindowOrderingMode","synthetic":true,"types":[]},{"text":"impl Sync for NSAlignmentOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSWindowCollectionBehavior","synthetic":true,"types":[]},{"text":"impl Sync for NSWindowOcclusionState","synthetic":true,"types":[]},{"text":"impl Sync for NSEventSwipeTrackingOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSEventPhase","synthetic":true,"types":[]},{"text":"impl Sync for NSTouchPhase","synthetic":true,"types":[]},{"text":"impl Sync for NSEventMask","synthetic":true,"types":[]},{"text":"impl Sync for NSEventModifierFlags","synthetic":true,"types":[]},{"text":"impl Sync for NSApplicationActivationPolicy","synthetic":true,"types":[]},{"text":"impl Sync for NSApplicationActivationOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSApplicationTerminateReply","synthetic":true,"types":[]},{"text":"impl Sync for NSWindowTitleVisibility","synthetic":true,"types":[]},{"text":"impl Sync for NSBackingStoreType","synthetic":true,"types":[]},{"text":"impl Sync for NSOpenGLPixelFormatAttribute","synthetic":true,"types":[]},{"text":"impl Sync for NSOpenGLPFAOpenGLProfiles","synthetic":true,"types":[]},{"text":"impl Sync for NSOpenGLContextParameter","synthetic":true,"types":[]},{"text":"impl Sync for NSWindowButton","synthetic":true,"types":[]},{"text":"impl Sync for NSBezelStyle","synthetic":true,"types":[]},{"text":"impl Sync for NSRequestUserAttentionType","synthetic":true,"types":[]},{"text":"impl Sync for NSPasteboardReadingOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSPasteboardWritingOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSEventGestureAxis","synthetic":true,"types":[]},{"text":"impl Sync for NSEventType","synthetic":true,"types":[]},{"text":"impl Sync for NSPointingDeviceType","synthetic":true,"types":[]},{"text":"impl Sync for NSEventButtonMask","synthetic":true,"types":[]},{"text":"impl Sync for NSEventSubtype","synthetic":true,"types":[]},{"text":"impl Sync for NSCompositingOperation","synthetic":true,"types":[]},{"text":"impl Sync for NSImageCacheMode","synthetic":true,"types":[]},{"text":"impl Sync for NSTIFFCompression","synthetic":true,"types":[]},{"text":"impl Sync for NSImageLoadStatus","synthetic":true,"types":[]},{"text":"impl Sync for NSTabViewType","synthetic":true,"types":[]},{"text":"impl Sync for NSTabState","synthetic":true,"types":[]},{"text":"impl Sync for NSPoint","synthetic":true,"types":[]},{"text":"impl Sync for NSSize","synthetic":true,"types":[]},{"text":"impl Sync for NSRect","synthetic":true,"types":[]},{"text":"impl Sync for NSRange","synthetic":true,"types":[]},{"text":"impl Sync for NSEnumerationOptions","synthetic":true,"types":[]},{"text":"impl !Sync for NSFastIterator","synthetic":true,"types":[]},{"text":"impl Sync for NSDataReadingOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSDataBase64EncodingOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSDataBase64DecodingOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSDataWritingOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSDataSearchOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSRectEdge","synthetic":true,"types":[]},{"text":"impl Sync for NSComparisonResult","synthetic":true,"types":[]},{"text":"impl Sync for EdgeAntialiasingMask","synthetic":true,"types":[]},{"text":"impl Sync for CornerMask","synthetic":true,"types":[]},{"text":"impl Sync for AutoresizingMask","synthetic":true,"types":[]},{"text":"impl Sync for CATransform3D","synthetic":true,"types":[]},{"text":"impl Sync for CVTimeStamp","synthetic":true,"types":[]},{"text":"impl Sync for CVSMPTETime","synthetic":true,"types":[]},{"text":"impl !Sync for ContentsGravity","synthetic":true,"types":[]},{"text":"impl !Sync for ContentsFormat","synthetic":true,"types":[]},{"text":"impl !Sync for Filter","synthetic":true,"types":[]},{"text":"impl Sync for CALayer","synthetic":false,"types":[]},{"text":"impl Sync for CARenderer","synthetic":false,"types":[]}];
implementors["core_foundation"] = [{"text":"impl&lt;T&nbsp;=&nbsp;*const c_void&gt; !Sync for CFArray&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !Sync for CFArrayIterator&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl !Sync for CFAttributedString","synthetic":true,"types":[]},{"text":"impl !Sync for CFMutableAttributedString","synthetic":true,"types":[]},{"text":"impl !Sync for CFType","synthetic":true,"types":[]},{"text":"impl !Sync for CFAllocator","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Sync for ItemRef&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Sync for ItemMutRef&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl !Sync for CFBoolean","synthetic":true,"types":[]},{"text":"impl !Sync for CFData","synthetic":true,"types":[]},{"text":"impl !Sync for CFDate","synthetic":true,"types":[]},{"text":"impl&lt;K&nbsp;=&nbsp;*const c_void, V&nbsp;=&nbsp;*const c_void&gt; !Sync for CFDictionary&lt;K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;K&nbsp;=&nbsp;*const c_void, V&nbsp;=&nbsp;*const c_void&gt; !Sync for CFMutableDictionary&lt;K, V&gt;","synthetic":true,"types":[]},{"text":"impl !Sync for CFError","synthetic":true,"types":[]},{"text":"impl !Sync for CFFileDescriptor","synthetic":true,"types":[]},{"text":"impl !Sync for CFNumber","synthetic":true,"types":[]},{"text":"impl&lt;T&nbsp;=&nbsp;*const c_void&gt; !Sync for CFSet&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl !Sync for CFString","synthetic":true,"types":[]},{"text":"impl !Sync for CFURL","synthetic":true,"types":[]},{"text":"impl !Sync for CFBundle","synthetic":true,"types":[]},{"text":"impl !Sync for CFPropertyList","synthetic":true,"types":[]},{"text":"impl !Sync for CFRunLoop","synthetic":true,"types":[]},{"text":"impl !Sync for CFRunLoopTimer","synthetic":true,"types":[]},{"text":"impl !Sync for CFRunLoopSource","synthetic":true,"types":[]},{"text":"impl !Sync for CFRunLoopObserver","synthetic":true,"types":[]},{"text":"impl !Sync for CFTimeZone","synthetic":true,"types":[]},{"text":"impl !Sync for CFUUID","synthetic":true,"types":[]}];
implementors["core_foundation_sys"] = [{"text":"impl Sync for CFArrayCallBacks","synthetic":true,"types":[]},{"text":"impl Sync for __CFArray","synthetic":true,"types":[]},{"text":"impl Sync for __CFAttributedString","synthetic":true,"types":[]},{"text":"impl Sync for CFRange","synthetic":true,"types":[]},{"text":"impl !Sync for CFAllocatorContext","synthetic":true,"types":[]},{"text":"impl Sync for CFComparisonResult","synthetic":true,"types":[]},{"text":"impl Sync for __CFBundle","synthetic":true,"types":[]},{"text":"impl Sync for __CFData","synthetic":true,"types":[]},{"text":"impl Sync for __CFDate","synthetic":true,"types":[]},{"text":"impl Sync for CFDictionaryKeyCallBacks","synthetic":true,"types":[]},{"text":"impl Sync for CFDictionaryValueCallBacks","synthetic":true,"types":[]},{"text":"impl Sync for __CFDictionary","synthetic":true,"types":[]},{"text":"impl Sync for __CFError","synthetic":true,"types":[]},{"text":"impl Sync for __CFFileDescriptor","synthetic":true,"types":[]},{"text":"impl !Sync for CFFileDescriptorContext","synthetic":true,"types":[]},{"text":"impl !Sync for CFMessagePortContext","synthetic":true,"types":[]},{"text":"impl Sync for __CFMessagePort","synthetic":true,"types":[]},{"text":"impl Sync for __CFBoolean","synthetic":true,"types":[]},{"text":"impl Sync for __CFNumber","synthetic":true,"types":[]},{"text":"impl Sync for __CFRunLoop","synthetic":true,"types":[]},{"text":"impl Sync for __CFRunLoopSource","synthetic":true,"types":[]},{"text":"impl Sync for __CFRunLoopObserver","synthetic":true,"types":[]},{"text":"impl !Sync for CFRunLoopSourceContext","synthetic":true,"types":[]},{"text":"impl !Sync for CFRunLoopSourceContext1","synthetic":true,"types":[]},{"text":"impl !Sync for CFRunLoopObserverContext","synthetic":true,"types":[]},{"text":"impl !Sync for CFRunLoopTimerContext","synthetic":true,"types":[]},{"text":"impl Sync for __CFRunLoopTimer","synthetic":true,"types":[]},{"text":"impl !Sync for CFSetCallBacks","synthetic":true,"types":[]},{"text":"impl Sync for __CFSet","synthetic":true,"types":[]},{"text":"impl Sync for __CFString","synthetic":true,"types":[]},{"text":"impl Sync for __CFTimeZone","synthetic":true,"types":[]},{"text":"impl Sync for __CFURL","synthetic":true,"types":[]},{"text":"impl Sync for __CFUUID","synthetic":true,"types":[]},{"text":"impl Sync for CFUUIDBytes","synthetic":true,"types":[]}];
implementors["core_graphics"] = [{"text":"impl !Sync for CGColor","synthetic":true,"types":[]},{"text":"impl !Sync for CGColorSpace","synthetic":true,"types":[]},{"text":"impl !Sync for CGColorSpaceRef","synthetic":true,"types":[]},{"text":"impl !Sync for CGContext","synthetic":true,"types":[]},{"text":"impl !Sync for CGContextRef","synthetic":true,"types":[]},{"text":"impl Sync for CGBlendMode","synthetic":true,"types":[]},{"text":"impl Sync for CGTextDrawingMode","synthetic":true,"types":[]},{"text":"impl !Sync for CGDataProvider","synthetic":true,"types":[]},{"text":"impl !Sync for CGDataProviderRef","synthetic":true,"types":[]},{"text":"impl Sync for CGDisplay","synthetic":true,"types":[]},{"text":"impl !Sync for CGDisplayMode","synthetic":true,"types":[]},{"text":"impl !Sync for CGDisplayModeRef","synthetic":true,"types":[]},{"text":"impl Sync for CGConfigureOption","synthetic":true,"types":[]},{"text":"impl Sync for CGEventFlags","synthetic":true,"types":[]},{"text":"impl Sync for KeyCode","synthetic":true,"types":[]},{"text":"impl Sync for ScrollEventUnit","synthetic":true,"types":[]},{"text":"impl Sync for EventField","synthetic":true,"types":[]},{"text":"impl !Sync for CGEvent","synthetic":true,"types":[]},{"text":"impl !Sync for CGEventRef","synthetic":true,"types":[]},{"text":"impl Sync for CGEventType","synthetic":true,"types":[]},{"text":"impl Sync for CGMouseButton","synthetic":true,"types":[]},{"text":"impl Sync for CGEventTapLocation","synthetic":true,"types":[]},{"text":"impl !Sync for CGEventSource","synthetic":true,"types":[]},{"text":"impl !Sync for CGEventSourceRef","synthetic":true,"types":[]},{"text":"impl Sync for CGEventSourceStateID","synthetic":true,"types":[]},{"text":"impl !Sync for CGFontRef","synthetic":true,"types":[]},{"text":"impl Sync for CGSize","synthetic":true,"types":[]},{"text":"impl Sync for CGPoint","synthetic":true,"types":[]},{"text":"impl Sync for CGRect","synthetic":true,"types":[]},{"text":"impl Sync for CGAffineTransform","synthetic":true,"types":[]},{"text":"impl !Sync for CGSRegion","synthetic":true,"types":[]},{"text":"impl Sync for CGSSurface","synthetic":true,"types":[]},{"text":"impl !Sync for CGImage","synthetic":true,"types":[]},{"text":"impl !Sync for CGImageRef","synthetic":true,"types":[]},{"text":"impl Sync for CGImageAlphaInfo","synthetic":true,"types":[]},{"text":"impl Sync for CGImageByteOrderInfo","synthetic":true,"types":[]},{"text":"impl !Sync for CGPath","synthetic":true,"types":[]},{"text":"impl !Sync for CGPathRef","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !Sync for CGPathElementRef&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl !Sync for CGPathElement","synthetic":true,"types":[]},{"text":"impl Sync for CGPathElementType","synthetic":true,"types":[]},{"text":"impl Sync for CGFont","synthetic":false,"types":[]}];
implementors["darwin_webkit"] = [{"text":"impl Sync for NSURLBookmarkCreationOptions","synthetic":true,"types":[]},{"text":"impl Sync for NSURLBookmarkResolutionOptions","synthetic":true,"types":[]},{"text":"impl Sync for DarwinWKApp","synthetic":false,"types":[]},{"text":"impl Sync for DarwinWKWebView","synthetic":false,"types":[]}];
implementors["dispatch"] = [{"text":"impl Sync for GroupGuard","synthetic":true,"types":[]},{"text":"impl Sync for SuspendGuard","synthetic":true,"types":[]},{"text":"impl Sync for SemaphoreGuard","synthetic":true,"types":[]},{"text":"impl Sync for WaitTimeout","synthetic":true,"types":[]},{"text":"impl Sync for QueueAttribute","synthetic":true,"types":[]},{"text":"impl Sync for QueuePriority","synthetic":true,"types":[]},{"text":"impl Sync for dispatch_object_s","synthetic":true,"types":[]},{"text":"impl Sync for Group","synthetic":false,"types":[]},{"text":"impl Sync for Queue","synthetic":false,"types":[]},{"text":"impl Sync for Once","synthetic":false,"types":[]},{"text":"impl Sync for Semaphore","synthetic":false,"types":[]}];
implementors["foreign_types_shared"] = [{"text":"impl !Sync for Opaque","synthetic":true,"types":[]}];
implementors["libc"] = [{"text":"impl !Sync for group","synthetic":true,"types":[]},{"text":"impl Sync for utimbuf","synthetic":true,"types":[]},{"text":"impl Sync for timeval","synthetic":true,"types":[]},{"text":"impl Sync for timespec","synthetic":true,"types":[]},{"text":"impl Sync for rlimit","synthetic":true,"types":[]},{"text":"impl Sync for rusage","synthetic":true,"types":[]},{"text":"impl Sync for ipv6_mreq","synthetic":true,"types":[]},{"text":"impl !Sync for hostent","synthetic":true,"types":[]},{"text":"impl !Sync for iovec","synthetic":true,"types":[]},{"text":"impl Sync for pollfd","synthetic":true,"types":[]},{"text":"impl Sync for winsize","synthetic":true,"types":[]},{"text":"impl Sync for linger","synthetic":true,"types":[]},{"text":"impl !Sync for sigval","synthetic":true,"types":[]},{"text":"impl Sync for itimerval","synthetic":true,"types":[]},{"text":"impl Sync for tms","synthetic":true,"types":[]},{"text":"impl !Sync for servent","synthetic":true,"types":[]},{"text":"impl !Sync for protoent","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_in6","synthetic":true,"types":[]},{"text":"impl !Sync for passwd","synthetic":true,"types":[]},{"text":"impl !Sync for ifaddrs","synthetic":true,"types":[]},{"text":"impl Sync for fd_set","synthetic":true,"types":[]},{"text":"impl !Sync for tm","synthetic":true,"types":[]},{"text":"impl !Sync for msghdr","synthetic":true,"types":[]},{"text":"impl Sync for cmsghdr","synthetic":true,"types":[]},{"text":"impl Sync for fsid_t","synthetic":true,"types":[]},{"text":"impl !Sync for if_nameindex","synthetic":true,"types":[]},{"text":"impl !Sync for regex_t","synthetic":true,"types":[]},{"text":"impl Sync for regmatch_t","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_un","synthetic":true,"types":[]},{"text":"impl Sync for utsname","synthetic":true,"types":[]},{"text":"impl Sync for ip_mreq","synthetic":true,"types":[]},{"text":"impl !Sync for aiocb","synthetic":true,"types":[]},{"text":"impl !Sync for glob_t","synthetic":true,"types":[]},{"text":"impl !Sync for addrinfo","synthetic":true,"types":[]},{"text":"impl Sync for mach_timebase_info","synthetic":true,"types":[]},{"text":"impl Sync for stat","synthetic":true,"types":[]},{"text":"impl Sync for pthread_mutexattr_t","synthetic":true,"types":[]},{"text":"impl Sync for pthread_condattr_t","synthetic":true,"types":[]},{"text":"impl Sync for pthread_rwlockattr_t","synthetic":true,"types":[]},{"text":"impl !Sync for siginfo_t","synthetic":true,"types":[]},{"text":"impl Sync for sigaction","synthetic":true,"types":[]},{"text":"impl !Sync for stack_t","synthetic":true,"types":[]},{"text":"impl Sync for fstore_t","synthetic":true,"types":[]},{"text":"impl Sync for radvisory","synthetic":true,"types":[]},{"text":"impl Sync for statvfs","synthetic":true,"types":[]},{"text":"impl !Sync for Dl_info","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_in","synthetic":true,"types":[]},{"text":"impl Sync for kevent64_s","synthetic":true,"types":[]},{"text":"impl Sync for dqblk","synthetic":true,"types":[]},{"text":"impl Sync for if_msghdr","synthetic":true,"types":[]},{"text":"impl Sync for termios","synthetic":true,"types":[]},{"text":"impl Sync for flock","synthetic":true,"types":[]},{"text":"impl !Sync for sf_hdtr","synthetic":true,"types":[]},{"text":"impl !Sync for lconv","synthetic":true,"types":[]},{"text":"impl Sync for proc_taskinfo","synthetic":true,"types":[]},{"text":"impl Sync for proc_bsdinfo","synthetic":true,"types":[]},{"text":"impl Sync for proc_taskallinfo","synthetic":true,"types":[]},{"text":"impl Sync for xsw_usage","synthetic":true,"types":[]},{"text":"impl Sync for xucred","synthetic":true,"types":[]},{"text":"impl Sync for mach_header","synthetic":true,"types":[]},{"text":"impl Sync for mach_header_64","synthetic":true,"types":[]},{"text":"impl Sync for segment_command","synthetic":true,"types":[]},{"text":"impl Sync for segment_command_64","synthetic":true,"types":[]},{"text":"impl Sync for load_command","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_dl","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_inarp","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_ctl","synthetic":true,"types":[]},{"text":"impl Sync for in_pktinfo","synthetic":true,"types":[]},{"text":"impl Sync for in6_pktinfo","synthetic":true,"types":[]},{"text":"impl Sync for ipc_perm","synthetic":true,"types":[]},{"text":"impl Sync for sembuf","synthetic":true,"types":[]},{"text":"impl Sync for arphdr","synthetic":true,"types":[]},{"text":"impl Sync for in_addr","synthetic":true,"types":[]},{"text":"impl !Sync for sa_endpoints_t","synthetic":true,"types":[]},{"text":"impl Sync for timex","synthetic":true,"types":[]},{"text":"impl Sync for ntptimeval","synthetic":true,"types":[]},{"text":"impl !Sync for kevent","synthetic":true,"types":[]},{"text":"impl Sync for semid_ds","synthetic":true,"types":[]},{"text":"impl !Sync for shmid_ds","synthetic":true,"types":[]},{"text":"impl Sync for proc_threadinfo","synthetic":true,"types":[]},{"text":"impl Sync for statfs","synthetic":true,"types":[]},{"text":"impl Sync for dirent","synthetic":true,"types":[]},{"text":"impl Sync for pthread_rwlock_t","synthetic":true,"types":[]},{"text":"impl Sync for pthread_mutex_t","synthetic":true,"types":[]},{"text":"impl Sync for pthread_cond_t","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_storage","synthetic":true,"types":[]},{"text":"impl Sync for utmpx","synthetic":true,"types":[]},{"text":"impl !Sync for sigevent","synthetic":true,"types":[]},{"text":"impl Sync for timeval32","synthetic":true,"types":[]},{"text":"impl Sync for if_data","synthetic":true,"types":[]},{"text":"impl Sync for bpf_hdr","synthetic":true,"types":[]},{"text":"impl !Sync for ucontext_t","synthetic":true,"types":[]},{"text":"impl Sync for __darwin_mcontext64","synthetic":true,"types":[]},{"text":"impl Sync for __darwin_x86_exception_state64","synthetic":true,"types":[]},{"text":"impl Sync for __darwin_x86_thread_state64","synthetic":true,"types":[]},{"text":"impl Sync for __darwin_x86_float_state64","synthetic":true,"types":[]},{"text":"impl Sync for __darwin_mmst_reg","synthetic":true,"types":[]},{"text":"impl Sync for __darwin_xmm_reg","synthetic":true,"types":[]},{"text":"impl Sync for pthread_attr_t","synthetic":true,"types":[]},{"text":"impl Sync for max_align_t","synthetic":true,"types":[]},{"text":"impl Sync for in6_addr","synthetic":true,"types":[]},{"text":"impl !Sync for semun","synthetic":true,"types":[]},{"text":"impl Sync for DIR","synthetic":true,"types":[]},{"text":"impl Sync for FILE","synthetic":true,"types":[]},{"text":"impl Sync for fpos_t","synthetic":true,"types":[]},{"text":"impl Sync for timezone","synthetic":true,"types":[]}];
implementors["malloc_buf"] = [{"text":"impl&lt;T&gt; !Sync for MallocBuffer&lt;T&gt;","synthetic":true,"types":[]}];
implementors["objc"] = [{"text":"impl !Sync for Encoding","synthetic":true,"types":[]},{"text":"impl Sync for MessageError","synthetic":true,"types":[]},{"text":"impl Sync for Ivar","synthetic":true,"types":[]},{"text":"impl Sync for Method","synthetic":true,"types":[]},{"text":"impl Sync for Class","synthetic":true,"types":[]},{"text":"impl Sync for Protocol","synthetic":true,"types":[]},{"text":"impl Sync for Object","synthetic":true,"types":[]},{"text":"impl !Sync for ClassDecl","synthetic":true,"types":[]},{"text":"impl !Sync for ProtocolDecl","synthetic":true,"types":[]},{"text":"impl !Sync for StrongPtr","synthetic":true,"types":[]},{"text":"impl !Sync for WeakPtr","synthetic":true,"types":[]},{"text":"impl Sync for Sel","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()