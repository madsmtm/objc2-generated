//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklookaroundscene?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLookAroundScene;
);

extern_conformance!(
    unsafe impl NSCopying for MKLookAroundScene {}
);

unsafe impl CopyingHelper for MKLookAroundScene {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MKLookAroundScene {}
);

impl MKLookAroundScene {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
