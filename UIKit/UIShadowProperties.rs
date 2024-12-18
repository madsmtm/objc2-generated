//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uishadowproperties?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIShadowProperties;
);

unsafe impl NSCoding for UIShadowProperties {}

unsafe impl NSCopying for UIShadowProperties {}

unsafe impl CopyingHelper for UIShadowProperties {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIShadowProperties {}

unsafe impl NSSecureCoding for UIShadowProperties {}

extern_methods!(
    unsafe impl UIShadowProperties {
        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Retained<UIColor>;

        #[cfg(feature = "UIColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &UIColor);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(opacity)]
        pub unsafe fn opacity(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setOpacity:)]
        pub unsafe fn setOpacity(&self, opacity: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(radius)]
        pub unsafe fn radius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setRadius:)]
        pub unsafe fn setRadius(&self, radius: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(offset)]
        pub unsafe fn offset(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setOffset:)]
        pub unsafe fn setOffset(&self, offset: CGSize);

        #[cfg(feature = "UIBezierPath")]
        #[method_id(@__retain_semantics Other path)]
        pub unsafe fn path(&self) -> Option<Retained<UIBezierPath>>;

        #[cfg(feature = "UIBezierPath")]
        #[method(setPath:)]
        pub unsafe fn setPath(&self, path: Option<&UIBezierPath>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIShadowProperties {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
