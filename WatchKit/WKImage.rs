//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkimage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKImage;
);

extern_conformance!(
    unsafe impl NSCoding for WKImage {}
);

extern_conformance!(
    unsafe impl NSCopying for WKImage {}
);

unsafe impl CopyingHelper for WKImage {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for WKImage {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for WKImage {}
);

impl WKImage {
    extern_methods!(
        #[cfg(feature = "objc2-ui-kit")]
        #[unsafe(method(imageWithImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageWithImage(image: &UIImage) -> Retained<Self>;

        #[unsafe(method(imageWithImageData:))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageWithImageData(image_data: &NSData) -> Retained<Self>;

        #[unsafe(method(imageWithImageName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageWithImageName(image_name: &NSString) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-ui-kit")]
        #[unsafe(method(image))]
        #[unsafe(method_family = none)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[unsafe(method(imageData))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageData(&self) -> Option<Retained<NSData>>;

        #[unsafe(method(imageName))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageName(&self) -> Option<Retained<NSString>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl WKImage {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
