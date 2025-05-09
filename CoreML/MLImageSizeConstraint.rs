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

extern_conformance!(
    unsafe impl NSCoding for MLImageSizeConstraint {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MLImageSizeConstraint {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MLImageSizeConstraint {}
);

impl MLImageSizeConstraint {
    extern_methods!(
        #[cfg(feature = "MLImageSizeConstraintType")]
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        pub unsafe fn r#type(&self) -> MLImageSizeConstraintType;

        #[unsafe(method(pixelsWideRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn pixelsWideRange(&self) -> NSRange;

        #[unsafe(method(pixelsHighRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn pixelsHighRange(&self) -> NSRange;

        #[cfg(feature = "MLImageSize")]
        #[unsafe(method(enumeratedImageSizes))]
        #[unsafe(method_family = none)]
        pub unsafe fn enumeratedImageSizes(&self) -> Retained<NSArray<MLImageSize>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MLImageSizeConstraint {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
