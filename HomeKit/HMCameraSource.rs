//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Abstract class for source of data from a camera.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcamerasource?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HMCameraSource;
);

unsafe impl Send for HMCameraSource {}

unsafe impl Sync for HMCameraSource {}

unsafe impl NSObjectProtocol for HMCameraSource {}

extern_methods!(
    unsafe impl HMCameraSource {
        /// Represents the aspect ratio of the camera source, defined as width over height.
        #[method(aspectRatio)]
        pub unsafe fn aspectRatio(&self) -> c_double;

        #[deprecated = "HMCameraSource is a base class for other types. Directly creating them is not supported."]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HMCameraSource {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);