//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CATransformLayer;

    #[cfg(feature = "CALayer")]
    unsafe impl ClassType for CATransformLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
    }
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
unsafe impl CAMediaTiming for CATransformLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSCoding for CATransformLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSObjectProtocol for CATransformLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSSecureCoding for CATransformLayer {}

extern_methods!(
    #[cfg(feature = "CALayer")]
    unsafe impl CATransformLayer {}
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CALayer")]
    unsafe impl CATransformLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CALayer")]
    unsafe impl CATransformLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
