//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfacegroup?language=objc)
    #[unsafe(super(WKInterfaceObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WKInterfaceObject")]
    pub struct WKInterfaceGroup;
);

#[cfg(feature = "WKInterfaceObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for WKInterfaceGroup {}
);

#[cfg(all(feature = "WKInterfaceImage", feature = "WKInterfaceObject"))]
extern_conformance!(
    unsafe impl WKImageAnimatable for WKInterfaceGroup {}
);

#[cfg(feature = "WKInterfaceObject")]
impl WKInterfaceGroup {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(setCornerRadius:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[cfg(feature = "objc2-ui-kit")]
        #[unsafe(method(setContentInset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentInset(&self, content_inset: UIEdgeInsets);

        #[cfg(feature = "objc2-ui-kit")]
        #[unsafe(method(setBackgroundColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBackgroundColor(&self, color: Option<&UIColor>);

        #[cfg(feature = "objc2-ui-kit")]
        #[unsafe(method(setBackgroundImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBackgroundImage(&self, image: Option<&UIImage>);

        #[unsafe(method(setBackgroundImageData:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBackgroundImageData(&self, image_data: Option<&NSData>);

        #[unsafe(method(setBackgroundImageNamed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBackgroundImageNamed(&self, image_name: Option<&NSString>);
    );
}

/// Methods declared on superclass `WKInterfaceObject`.
#[cfg(feature = "WKInterfaceObject")]
impl WKInterfaceGroup {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "WKInterfaceObject")]
impl WKInterfaceGroup {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
