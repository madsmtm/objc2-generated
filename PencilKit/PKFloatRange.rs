//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/pencilkit/pkfloatrange?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PKFloatRange;
);

extern_conformance!(
    unsafe impl NSCopying for PKFloatRange {}
);

unsafe impl CopyingHelper for PKFloatRange {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for PKFloatRange {}
);

impl PKFloatRange {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(lowerBound))]
        #[unsafe(method_family = none)]
        pub unsafe fn lowerBound(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(upperBound))]
        #[unsafe(method_family = none)]
        pub unsafe fn upperBound(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithLowerBound:upperBound:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLowerBound_upperBound(
            this: Allocated<Self>,
            lower_bound: CGFloat,
            upper_bound: CGFloat,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl PKFloatRange {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
