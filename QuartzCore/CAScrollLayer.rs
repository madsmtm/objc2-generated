//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cascrolllayerscrollmode?language=objc)
// NS_TYPED_ENUM
pub type CAScrollLayerScrollMode = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cascrolllayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CAScrollLayer;
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
extern_conformance!(
    unsafe impl CAMediaTiming for CAScrollLayer {}
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl NSCoding for CAScrollLayer {}
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CAScrollLayer {}
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl NSSecureCoding for CAScrollLayer {}
);

#[cfg(feature = "CALayer")]
impl CAScrollLayer {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(scrollToPoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scrollToPoint(&self, p: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(scrollToRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scrollToRect(&self, r: CGRect);

        #[unsafe(method(scrollMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn scrollMode(&self) -> Retained<CAScrollLayerScrollMode>;

        /// Setter for [`scrollMode`][Self::scrollMode].
        #[unsafe(method(setScrollMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setScrollMode(&self, scroll_mode: &CAScrollLayerScrollMode);
    );
}

/// Methods declared on superclass `CALayer`.
#[cfg(feature = "CALayer")]
impl CAScrollLayer {
    extern_methods!(
        /// Layer creation and initialization. *
        #[unsafe(method(layer))]
        #[unsafe(method_family = none)]
        pub unsafe fn layer() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithLayer:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "CALayer")]
impl CAScrollLayer {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// CALayerScrolling.
#[cfg(feature = "CALayer")]
impl CALayer {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(scrollPoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scrollPoint(&self, p: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(scrollRectToVisible:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scrollRectToVisible(&self, r: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(visibleRect))]
        #[unsafe(method_family = none)]
        pub unsafe fn visibleRect(&self) -> CGRect;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcascrollnone?language=objc)
    pub static kCAScrollNone: &'static CAScrollLayerScrollMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcascrollvertically?language=objc)
    pub static kCAScrollVertically: &'static CAScrollLayerScrollMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcascrollhorizontally?language=objc)
    pub static kCAScrollHorizontally: &'static CAScrollLayerScrollMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcascrollboth?language=objc)
    pub static kCAScrollBoth: &'static CAScrollLayerScrollMode;
}
