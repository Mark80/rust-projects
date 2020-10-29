(function() {var implementors = {};
implementors["block"] = [{"text":"impl&lt;A, R&gt; Drop for RcBlock&lt;A, R&gt;","synthetic":false,"types":[]}];
implementors["cocoa"] = [{"text":"impl Drop for CALayer","synthetic":false,"types":[]},{"text":"impl Drop for CARenderer","synthetic":false,"types":[]}];
implementors["core_foundation"] = [{"text":"impl&lt;T&gt; Drop for CFArray&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Drop for CFAttributedString","synthetic":false,"types":[]},{"text":"impl Drop for CFMutableAttributedString","synthetic":false,"types":[]},{"text":"impl Drop for CFType","synthetic":false,"types":[]},{"text":"impl Drop for CFAllocator","synthetic":false,"types":[]},{"text":"impl Drop for CFBoolean","synthetic":false,"types":[]},{"text":"impl Drop for CFData","synthetic":false,"types":[]},{"text":"impl Drop for CFDate","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; Drop for CFDictionary&lt;K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; Drop for CFMutableDictionary&lt;K, V&gt;","synthetic":false,"types":[]},{"text":"impl Drop for CFError","synthetic":false,"types":[]},{"text":"impl Drop for CFFileDescriptor","synthetic":false,"types":[]},{"text":"impl Drop for CFNumber","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Drop for CFSet&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Drop for CFString","synthetic":false,"types":[]},{"text":"impl Drop for CFURL","synthetic":false,"types":[]},{"text":"impl Drop for CFBundle","synthetic":false,"types":[]},{"text":"impl Drop for CFPropertyList","synthetic":false,"types":[]},{"text":"impl Drop for CFRunLoop","synthetic":false,"types":[]},{"text":"impl Drop for CFRunLoopTimer","synthetic":false,"types":[]},{"text":"impl Drop for CFRunLoopSource","synthetic":false,"types":[]},{"text":"impl Drop for CFRunLoopObserver","synthetic":false,"types":[]},{"text":"impl Drop for CFTimeZone","synthetic":false,"types":[]},{"text":"impl Drop for CFUUID","synthetic":false,"types":[]}];
implementors["core_graphics"] = [{"text":"impl Drop for CGColor","synthetic":false,"types":[]},{"text":"impl Drop for CGColorSpace","synthetic":false,"types":[]},{"text":"impl Drop for CGContext","synthetic":false,"types":[]},{"text":"impl Drop for CGDataProvider","synthetic":false,"types":[]},{"text":"impl Drop for CGDisplayMode","synthetic":false,"types":[]},{"text":"impl Drop for CGEvent","synthetic":false,"types":[]},{"text":"impl Drop for CGEventSource","synthetic":false,"types":[]},{"text":"impl Drop for CGFont","synthetic":false,"types":[]},{"text":"impl Drop for CGSRegion","synthetic":false,"types":[]},{"text":"impl Drop for CGImage","synthetic":false,"types":[]},{"text":"impl Drop for CGPath","synthetic":false,"types":[]}];
implementors["crossbeam_channel"] = [{"text":"impl&lt;T&gt; Drop for Sender&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Drop for Receiver&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; Drop for SelectedOperation&lt;'_&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_deque"] = [{"text":"impl&lt;T&gt; Drop for Injector&lt;T&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T:&nbsp;?Sized + Pointable&gt; Drop for Owned&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Drop for LocalHandle","synthetic":false,"types":[]},{"text":"impl Drop for Guard","synthetic":false,"types":[]}];
implementors["crossbeam_utils"] = [{"text":"impl&lt;T:&nbsp;?Sized, '_&gt; Drop for ShardedLockWriteGuard&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl Drop for WaitGroup","synthetic":false,"types":[]}];
implementors["deflate"] = [{"text":"impl&lt;W:&nbsp;Write&gt; Drop for DeflateEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Drop for ZlibEncoder&lt;W&gt;","synthetic":false,"types":[]}];
implementors["gif"] = [{"text":"impl&lt;W:&nbsp;Write&gt; Drop for Encoder&lt;W&gt;","synthetic":false,"types":[]}];
implementors["objc"] = [{"text":"impl Drop for ClassDecl","synthetic":false,"types":[]},{"text":"impl Drop for StrongPtr","synthetic":false,"types":[]},{"text":"impl Drop for WeakPtr","synthetic":false,"types":[]}];
implementors["opengl_graphics"] = [{"text":"impl Drop for DynamicAttribute","synthetic":false,"types":[]},{"text":"impl Drop for Colored","synthetic":false,"types":[]},{"text":"impl Drop for Textured","synthetic":false,"types":[]},{"text":"impl Drop for Texture","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl&lt;W:&nbsp;Write&gt; Drop for Writer&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, W:&nbsp;Write&gt; Drop for StreamWriter&lt;'a, W&gt;","synthetic":false,"types":[]}];
implementors["rayon"] = [{"text":"impl&lt;'a, T:&nbsp;Ord + Send&gt; Drop for Drain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Send&gt; Drop for Drain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Drop for Drain&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, T:&nbsp;Send&gt; Drop for Drain&lt;'data, T&gt;","synthetic":false,"types":[]}];
implementors["rayon_core"] = [{"text":"impl Drop for ThreadPool","synthetic":false,"types":[]}];
implementors["scoped_threadpool"] = [{"text":"impl Drop for Pool","synthetic":false,"types":[]},{"text":"impl&lt;'pool, 'scope&gt; Drop for Scope&lt;'pool, 'scope&gt;","synthetic":false,"types":[]}];
implementors["scopeguard"] = [{"text":"impl&lt;T, F, S&gt; Drop for ScopeGuard&lt;T, F, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnOnce(T),<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Strategy,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;'a&gt; Drop for ParseBuffer&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["tiff"] = [{"text":"impl&lt;'a, W:&nbsp;Write + Seek&gt; Drop for DirectoryEncoder&lt;'a, W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, W:&nbsp;Write + Seek, C:&nbsp;ColorType&gt; Drop for ImageEncoder&lt;'a, W, C&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()