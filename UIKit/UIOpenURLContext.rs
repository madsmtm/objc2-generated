//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiopenurlcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIOpenURLContext;
);

unsafe impl NSObjectProtocol for UIOpenURLContext {}

extern_methods!(
    unsafe impl UIOpenURLContext {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        #[cfg(feature = "UISceneOptions")]
        #[unsafe(method_family(none))]
        #[method_id(options)]
        pub unsafe fn options(&self) -> Retained<UISceneOpenURLOptions>;
    }
);
