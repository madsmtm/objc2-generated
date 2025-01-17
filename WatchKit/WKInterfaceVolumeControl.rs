//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfacevolumecontrolorigin?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKInterfaceVolumeControlOrigin(pub NSInteger);
impl WKInterfaceVolumeControlOrigin {
    #[doc(alias = "WKInterfaceVolumeControlOriginLocal")]
    pub const Local: Self = Self(0);
    #[doc(alias = "WKInterfaceVolumeControlOriginCompanion")]
    pub const Companion: Self = Self(1);
}

unsafe impl Encode for WKInterfaceVolumeControlOrigin {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKInterfaceVolumeControlOrigin {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfacevolumecontrol?language=objc)
    #[unsafe(super(WKInterfaceObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WKInterfaceObject")]
    pub struct WKInterfaceVolumeControl;
);

#[cfg(feature = "WKInterfaceObject")]
unsafe impl NSObjectProtocol for WKInterfaceVolumeControl {}

extern_methods!(
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceVolumeControl {
        #[unsafe(method_family(init))]
        #[method_id(initWithOrigin:)]
        pub unsafe fn initWithOrigin(
            this: Allocated<Self>,
            origin: WKInterfaceVolumeControlOrigin,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-ui-kit")]
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[method(focus)]
        pub unsafe fn focus(&self);

        #[method(resignFocus)]
        pub unsafe fn resignFocus(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `WKInterfaceObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceVolumeControl {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceVolumeControl {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
