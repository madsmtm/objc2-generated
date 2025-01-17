//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkimageanimatable?language=objc)
    pub unsafe trait WKImageAnimatable: NSObjectProtocol {
        #[method(startAnimating)]
        unsafe fn startAnimating(&self);

        #[method(startAnimatingWithImagesInRange:duration:repeatCount:)]
        unsafe fn startAnimatingWithImagesInRange_duration_repeatCount(
            &self,
            image_range: NSRange,
            duration: NSTimeInterval,
            repeat_count: NSInteger,
        );

        #[method(stopAnimating)]
        unsafe fn stopAnimating(&self);
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfaceimage?language=objc)
    #[unsafe(super(WKInterfaceObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WKInterfaceObject")]
    pub struct WKInterfaceImage;
);

#[cfg(feature = "WKInterfaceObject")]
unsafe impl NSObjectProtocol for WKInterfaceImage {}

#[cfg(feature = "WKInterfaceObject")]
unsafe impl WKImageAnimatable for WKInterfaceImage {}

extern_methods!(
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceImage {
        #[cfg(feature = "objc2-ui-kit")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[method(setImageData:)]
        pub unsafe fn setImageData(&self, image_data: Option<&NSData>);

        #[method(setImageNamed:)]
        pub unsafe fn setImageNamed(&self, image_name: Option<&NSString>);

        #[cfg(feature = "objc2-ui-kit")]
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);
    }
);

extern_methods!(
    /// Methods declared on superclass `WKInterfaceObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceImage {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceImage {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
