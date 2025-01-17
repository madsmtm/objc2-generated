//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uivisualeffect?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIVisualEffect;
);

unsafe impl NSCoding for UIVisualEffect {}

unsafe impl NSCopying for UIVisualEffect {}

unsafe impl CopyingHelper for UIVisualEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIVisualEffect {}

unsafe impl NSSecureCoding for UIVisualEffect {}

extern_methods!(
    unsafe impl UIVisualEffect {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIVisualEffect {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
