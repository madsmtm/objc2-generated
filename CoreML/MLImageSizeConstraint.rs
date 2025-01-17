//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlimagesizeconstraint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLImageSizeConstraint;
);

unsafe impl NSCoding for MLImageSizeConstraint {}

unsafe impl NSObjectProtocol for MLImageSizeConstraint {}

unsafe impl NSSecureCoding for MLImageSizeConstraint {}

extern_methods!(
    unsafe impl MLImageSizeConstraint {
        #[cfg(feature = "MLImageSizeConstraintType")]
        #[method(type)]
        pub unsafe fn r#type(&self) -> MLImageSizeConstraintType;

        #[method(pixelsWideRange)]
        pub unsafe fn pixelsWideRange(&self) -> NSRange;

        #[method(pixelsHighRange)]
        pub unsafe fn pixelsHighRange(&self) -> NSRange;

        #[cfg(feature = "MLImageSize")]
        #[unsafe(method_family(none))]
        #[method_id(enumeratedImageSizes)]
        pub unsafe fn enumeratedImageSizes(&self) -> Retained<NSArray<MLImageSize>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLImageSizeConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
