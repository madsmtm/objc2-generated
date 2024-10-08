//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CISampler;

    unsafe impl ClassType for CISampler {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for CISampler {}

unsafe impl CopyingHelper for CISampler {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CISampler {}

extern_methods!(
    unsafe impl CISampler {
        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other samplerWithImage:)]
        pub unsafe fn samplerWithImage(im: &CIImage) -> Retained<Self>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other samplerWithImage:options:)]
        pub unsafe fn samplerWithImage_options(
            im: &CIImage,
            dict: Option<&NSDictionary>,
        ) -> Retained<Self>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Init initWithImage:)]
        pub unsafe fn initWithImage(this: Allocated<Self>, im: &CIImage) -> Retained<Self>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Init initWithImage:options:)]
        pub unsafe fn initWithImage_options(
            this: Allocated<Self>,
            im: &CIImage,
            dict: Option<&NSDictionary>,
        ) -> Retained<Self>;

        #[cfg(feature = "CIFilterShape")]
        #[method_id(@__retain_semantics Other definition)]
        pub unsafe fn definition(&self) -> Retained<CIFilterShape>;

        #[method(extent)]
        pub unsafe fn extent(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CISampler {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static kCISamplerAffineMatrix: &'static NSString;
}

extern "C" {
    pub static kCISamplerWrapMode: &'static NSString;
}

extern "C" {
    pub static kCISamplerFilterMode: &'static NSString;
}

extern "C" {
    pub static kCISamplerWrapBlack: &'static NSString;
}

extern "C" {
    pub static kCISamplerWrapClamp: &'static NSString;
}

extern "C" {
    pub static kCISamplerFilterNearest: &'static NSString;
}

extern "C" {
    pub static kCISamplerFilterLinear: &'static NSString;
}

extern "C" {
    pub static kCISamplerColorSpace: &'static NSString;
}
