(function() {var implementors = {};
implementors["core_foundation"] = [{"text":"impl From&lt;bool&gt; for CFBoolean","synthetic":false,"types":[]},{"text":"impl From&lt;CFBoolean&gt; for bool","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; From&lt;&amp;'a CFDictionary&lt;K, V&gt;&gt; for CFMutableDictionary&lt;K, V&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;i32&gt; for CFNumber","synthetic":false,"types":[]},{"text":"impl From&lt;i64&gt; for CFNumber","synthetic":false,"types":[]},{"text":"impl From&lt;f32&gt; for CFNumber","synthetic":false,"types":[]},{"text":"impl From&lt;f64&gt; for CFNumber","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;&amp;'a str&gt; for CFString","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;&amp;'a CFString&gt; for Cow&lt;'a, str&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_channel"] = [{"text":"impl&lt;T&gt; From&lt;SendError&lt;T&gt;&gt; for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;SendError&lt;T&gt;&gt; for SendTimeoutError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;RecvError&gt; for TryRecvError","synthetic":false,"types":[]},{"text":"impl From&lt;RecvError&gt; for RecvTimeoutError","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T:&nbsp;?Sized + Pointable&gt; From&lt;Owned&lt;T&gt;&gt; for Atomic&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;Box&lt;T&gt;&gt; for Atomic&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for Atomic&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'g, T:&nbsp;?Sized + Pointable&gt; From&lt;Shared&lt;'g, T&gt;&gt; for Atomic&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;*const T&gt; for Atomic&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for Owned&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;Box&lt;T&gt;&gt; for Owned&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, '_&gt; From&lt;*const T&gt; for Shared&lt;'_, T&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_utils"] = [{"text":"impl&lt;T&gt; From&lt;T&gt; for AtomicCell&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for CachePadded&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for ShardedLock&lt;T&gt;","synthetic":false,"types":[]}];
implementors["deflate"] = [{"text":"impl From&lt;Compression&gt; for CompressionOptions","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; From&lt;Result&lt;R, L&gt;&gt; for Either&lt;L, R&gt;","synthetic":false,"types":[]}];
implementors["gif"] = [{"text":"impl From&lt;Extension&gt; for AnyExtension","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for DecodingError","synthetic":false,"types":[]},{"text":"impl From&lt;DecodingFormatError&gt; for DecodingError","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for EncodingError","synthetic":false,"types":[]}];
implementors["glutin"] = [{"text":"impl From&lt;CreationError&gt; for CreationError","synthetic":false,"types":[]}];
implementors["image"] = [{"text":"impl From&lt;Error&gt; for ImageError","synthetic":false,"types":[]},{"text":"impl From&lt;ImageFormat&gt; for ImageFormatHint","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; From&lt;&amp;'_ Path&gt; for ImageFormatHint","synthetic":false,"types":[]},{"text":"impl From&lt;ImageFormatHint&gt; for UnsupportedError","synthetic":false,"types":[]},{"text":"impl From&lt;NeuQuant&gt; for NeuQuant","synthetic":false,"types":[]},{"text":"impl From&lt;NeuQuant&gt; for NeuQuant","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for ImageError","synthetic":false,"types":[]},{"text":"impl From&lt;BitmapHeader&gt; for PnmHeader","synthetic":false,"types":[]},{"text":"impl From&lt;GraymapHeader&gt; for PnmHeader","synthetic":false,"types":[]},{"text":"impl From&lt;PixmapHeader&gt; for PnmHeader","synthetic":false,"types":[]},{"text":"impl From&lt;ArbitraryHeader&gt; for PnmHeader","synthetic":false,"types":[]},{"text":"impl From&lt;Delay&gt; for Duration","synthetic":false,"types":[]},{"text":"impl From&lt;ColorType&gt; for ExtendedColorType","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Primitive + 'static&gt; From&lt;[T; 3]&gt; for Rgb&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Primitive + 'static&gt; From&lt;[T; 3]&gt; for Bgr&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Primitive + 'static&gt; From&lt;[T; 1]&gt; for Luma&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Primitive + 'static&gt; From&lt;[T; 4]&gt; for Rgba&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Primitive + 'static&gt; From&lt;[T; 4]&gt; for Bgra&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Primitive + 'static&gt; From&lt;[T; 2]&gt; for LumaA&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;ImageFormat&gt; for ImageOutputFormat","synthetic":false,"types":[]}];
implementors["input"] = [{"text":"impl From&lt;u32&gt; for Key","synthetic":false,"types":[]},{"text":"impl From&lt;Key&gt; for u32","synthetic":false,"types":[]},{"text":"impl From&lt;u32&gt; for MouseButton","synthetic":false,"types":[]},{"text":"impl From&lt;MouseButton&gt; for u32","synthetic":false,"types":[]},{"text":"impl From&lt;Key&gt; for Button","synthetic":false,"types":[]},{"text":"impl From&lt;MouseButton&gt; for Button","synthetic":false,"types":[]},{"text":"impl From&lt;ControllerButton&gt; for Button","synthetic":false,"types":[]},{"text":"impl From&lt;ButtonArgs&gt; for Input","synthetic":false,"types":[]},{"text":"impl From&lt;ControllerAxisArgs&gt; for Motion","synthetic":false,"types":[]},{"text":"impl From&lt;ControllerAxisArgs&gt; for Input","synthetic":false,"types":[]},{"text":"impl From&lt;TouchArgs&gt; for Motion","synthetic":false,"types":[]},{"text":"impl From&lt;TouchArgs&gt; for Input","synthetic":false,"types":[]},{"text":"impl From&lt;Motion&gt; for Input","synthetic":false,"types":[]},{"text":"impl From&lt;RenderArgs&gt; for Loop","synthetic":false,"types":[]},{"text":"impl From&lt;RenderArgs&gt; for Event","synthetic":false,"types":[]},{"text":"impl From&lt;AfterRenderArgs&gt; for Loop","synthetic":false,"types":[]},{"text":"impl From&lt;AfterRenderArgs&gt; for Event","synthetic":false,"types":[]},{"text":"impl From&lt;UpdateArgs&gt; for Loop","synthetic":false,"types":[]},{"text":"impl From&lt;UpdateArgs&gt; for Event","synthetic":false,"types":[]},{"text":"impl From&lt;IdleArgs&gt; for Loop","synthetic":false,"types":[]},{"text":"impl From&lt;IdleArgs&gt; for Event","synthetic":false,"types":[]},{"text":"impl From&lt;CloseArgs&gt; for Input","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for Event <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Input: From&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;(T, Option&lt;u32&gt;)&gt; for Event <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Input: From&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl From&lt;Loop&gt; for Event","synthetic":false,"types":[]}];
implementors["jpeg_decoder"] = [{"text":"impl From&lt;Error&gt; for Error","synthetic":false,"types":[]}];
implementors["miniz_oxide"] = [{"text":"impl From&lt;MZFlush&gt; for TDEFLFlush","synthetic":false,"types":[]},{"text":"impl From&lt;StreamResult&gt; for MZResult","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; From&lt;&amp;'_ StreamResult&gt; for MZResult","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T&gt; From&lt;T&gt; for Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;(T, T)&gt; for Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["opengl_graphics"] = [{"text":"impl From&lt;Error&gt; for Error","synthetic":false,"types":[]}];
implementors["ordered_float"] = [{"text":"impl&lt;T:&nbsp;Float&gt; From&lt;T&gt; for OrderedFloat&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;NotNan&lt;f32&gt;&gt; for f32","synthetic":false,"types":[]},{"text":"impl From&lt;NotNan&lt;f64&gt;&gt; for f64","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Float&gt; From&lt;T&gt; for NotNan&lt;T&gt;","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl From&lt;Compression&gt; for Compression","synthetic":false,"types":[]},{"text":"impl From&lt;Compression&gt; for CompressionOptions","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for DecodingError","synthetic":false,"types":[]},{"text":"impl From&lt;String&gt; for DecodingError","synthetic":false,"types":[]},{"text":"impl From&lt;DecodingError&gt; for Error","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for EncodingError","synthetic":false,"types":[]},{"text":"impl From&lt;EncodingError&gt; for Error","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl From&lt;Span&gt; for Span","synthetic":false,"types":[]},{"text":"impl From&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl From&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl From&lt;TokenTree&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl From&lt;Group&gt; for TokenTree","synthetic":false,"types":[]},{"text":"impl From&lt;Ident&gt; for TokenTree","synthetic":false,"types":[]},{"text":"impl From&lt;Punct&gt; for TokenTree","synthetic":false,"types":[]},{"text":"impl From&lt;Literal&gt; for TokenTree","synthetic":false,"types":[]}];
implementors["rusttype"] = [{"text":"impl&lt;'a&gt; From&lt;&amp;'a [u8]&gt; for SharedBytes&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;Arc&lt;[u8]&gt;&gt; for SharedBytes&lt;'static&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;Box&lt;[u8]&gt;&gt; for SharedBytes&lt;'static&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;Vec&lt;u8&gt;&gt; for SharedBytes&lt;'static&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;AsRef&lt;[u8]&gt;&gt; From&lt;&amp;'a T&gt; for SharedBytes&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;VMetrics&gt; for VMetrics","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for Error","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl From&lt;SelfValue&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;SelfType&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Super&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Crate&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Extern&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Underscore&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Path&gt; for Meta","synthetic":false,"types":[]},{"text":"impl From&lt;MetaList&gt; for Meta","synthetic":false,"types":[]},{"text":"impl From&lt;MetaNameValue&gt; for Meta","synthetic":false,"types":[]},{"text":"impl From&lt;Meta&gt; for NestedMeta","synthetic":false,"types":[]},{"text":"impl From&lt;Lit&gt; for NestedMeta","synthetic":false,"types":[]},{"text":"impl From&lt;FieldsNamed&gt; for Fields","synthetic":false,"types":[]},{"text":"impl From&lt;FieldsUnnamed&gt; for Fields","synthetic":false,"types":[]},{"text":"impl From&lt;VisPublic&gt; for Visibility","synthetic":false,"types":[]},{"text":"impl From&lt;VisCrate&gt; for Visibility","synthetic":false,"types":[]},{"text":"impl From&lt;VisRestricted&gt; for Visibility","synthetic":false,"types":[]},{"text":"impl From&lt;ExprArray&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAssign&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAssignOp&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAsync&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAwait&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBinary&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBlock&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBox&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBreak&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprCall&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprCast&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprClosure&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprContinue&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprField&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprForLoop&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprGroup&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprIf&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprIndex&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprLet&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprLit&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprLoop&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprMacro&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprMatch&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprMethodCall&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprParen&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprPath&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprRange&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprReference&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprRepeat&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprReturn&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprStruct&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprTry&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprTryBlock&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprTuple&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprType&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprUnary&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprUnsafe&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprWhile&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprYield&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;usize&gt; for Index","synthetic":false,"types":[]},{"text":"impl From&lt;TypeParam&gt; for GenericParam","synthetic":false,"types":[]},{"text":"impl From&lt;LifetimeDef&gt; for GenericParam","synthetic":false,"types":[]},{"text":"impl From&lt;ConstParam&gt; for GenericParam","synthetic":false,"types":[]},{"text":"impl From&lt;Ident&gt; for TypeParam","synthetic":false,"types":[]},{"text":"impl From&lt;TraitBound&gt; for TypeParamBound","synthetic":false,"types":[]},{"text":"impl From&lt;Lifetime&gt; for TypeParamBound","synthetic":false,"types":[]},{"text":"impl From&lt;PredicateType&gt; for WherePredicate","synthetic":false,"types":[]},{"text":"impl From&lt;PredicateLifetime&gt; for WherePredicate","synthetic":false,"types":[]},{"text":"impl From&lt;PredicateEq&gt; for WherePredicate","synthetic":false,"types":[]},{"text":"impl From&lt;LitStr&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitByteStr&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitByte&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitChar&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitInt&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitFloat&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitBool&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;Literal&gt; for LitInt","synthetic":false,"types":[]},{"text":"impl From&lt;Literal&gt; for LitFloat","synthetic":false,"types":[]},{"text":"impl From&lt;DataStruct&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;DataEnum&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;DataUnion&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;TypeArray&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeBareFn&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeGroup&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeImplTrait&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeInfer&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeMacro&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeNever&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeParen&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypePath&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypePtr&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeReference&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeSlice&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeTraitObject&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeTuple&gt; for Type","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for Path <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;PathSegment&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for PathSegment <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;Ident&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl From&lt;LexError&gt; for Error","synthetic":false,"types":[]}];
implementors["tiff"] = [{"text":"impl From&lt;Error&gt; for TiffError","synthetic":false,"types":[]},{"text":"impl From&lt;FromUtf8Error&gt; for TiffError","synthetic":false,"types":[]},{"text":"impl From&lt;TiffFormatError&gt; for TiffError","synthetic":false,"types":[]},{"text":"impl From&lt;TiffUnsupportedError&gt; for TiffError","synthetic":false,"types":[]},{"text":"impl From&lt;TryFromIntError&gt; for TiffError","synthetic":false,"types":[]}];
implementors["window"] = [{"text":"impl From&lt;[u32; 2]&gt; for Size","synthetic":false,"types":[]},{"text":"impl From&lt;[f64; 2]&gt; for Size","synthetic":false,"types":[]},{"text":"impl From&lt;(u32, u32)&gt; for Size","synthetic":false,"types":[]},{"text":"impl From&lt;(f64, f64)&gt; for Size","synthetic":false,"types":[]},{"text":"impl From&lt;Size&gt; for [u32; 2]","synthetic":false,"types":[]},{"text":"impl From&lt;Size&gt; for [f64; 2]","synthetic":false,"types":[]},{"text":"impl From&lt;Size&gt; for (u32, u32)","synthetic":false,"types":[]},{"text":"impl From&lt;Size&gt; for (f64, f64)","synthetic":false,"types":[]},{"text":"impl From&lt;[i32; 2]&gt; for Position","synthetic":false,"types":[]},{"text":"impl From&lt;(i32, i32)&gt; for Position","synthetic":false,"types":[]},{"text":"impl From&lt;Position&gt; for [i32; 2]","synthetic":false,"types":[]},{"text":"impl From&lt;Position&gt; for (i32, i32)","synthetic":false,"types":[]}];
implementors["winit"] = [{"text":"impl From&lt;(f64, f64)&gt; for LogicalPosition","synthetic":false,"types":[]},{"text":"impl From&lt;(i32, i32)&gt; for LogicalPosition","synthetic":false,"types":[]},{"text":"impl From&lt;(f64, f64)&gt; for PhysicalPosition","synthetic":false,"types":[]},{"text":"impl From&lt;(i32, i32)&gt; for PhysicalPosition","synthetic":false,"types":[]},{"text":"impl From&lt;(f64, f64)&gt; for LogicalSize","synthetic":false,"types":[]},{"text":"impl From&lt;(u32, u32)&gt; for LogicalSize","synthetic":false,"types":[]},{"text":"impl From&lt;(f64, f64)&gt; for PhysicalSize","synthetic":false,"types":[]},{"text":"impl From&lt;(u32, u32)&gt; for PhysicalSize","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()