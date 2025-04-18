//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlimagesize?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLImageSize;
);

extern_conformance!(
    unsafe impl NSCoding for MLImageSize {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MLImageSize {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MLImageSize {}
);

impl MLImageSize {
    extern_methods!(
        #[unsafe(method(pixelsWide))]
        #[unsafe(method_family = none)]
        pub unsafe fn pixelsWide(&self) -> NSInteger;

        #[unsafe(method(pixelsHigh))]
        #[unsafe(method_family = none)]
        pub unsafe fn pixelsHigh(&self) -> NSInteger;
    );
}

/// Methods declared on superclass `NSObject`.
impl MLImageSize {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
