//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiproposedscenesizenopreference?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static UIProposedSceneSizeNoPreference: CGFloat;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowscenegeometrypreferencesvision?language=objc)
    #[unsafe(super(UIWindowSceneGeometryPreferences, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIWindowSceneGeometryPreferences")]
    pub struct UIWindowSceneGeometryPreferencesVision;
);

#[cfg(feature = "UIWindowSceneGeometryPreferences")]
unsafe impl NSObjectProtocol for UIWindowSceneGeometryPreferencesVision {}

extern_methods!(
    #[cfg(feature = "UIWindowSceneGeometryPreferences")]
    unsafe impl UIWindowSceneGeometryPreferencesVision {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Allocated<Self>, size: CGSize) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(size)]
        pub unsafe fn size(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumSize)]
        pub unsafe fn minimumSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMinimumSize:)]
        pub unsafe fn setMinimumSize(&self, minimum_size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(maximumSize)]
        pub unsafe fn maximumSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMaximumSize:)]
        pub unsafe fn setMaximumSize(&self, maximum_size: CGSize);

        #[cfg(feature = "UIWindowSceneGeometry")]
        #[method(resizingRestrictions)]
        pub unsafe fn resizingRestrictions(&self) -> UIWindowSceneResizingRestrictions;

        #[cfg(feature = "UIWindowSceneGeometry")]
        #[method(setResizingRestrictions:)]
        pub unsafe fn setResizingRestrictions(
            &self,
            resizing_restrictions: UIWindowSceneResizingRestrictions,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIWindowSceneGeometryPreferences`
    #[cfg(feature = "UIWindowSceneGeometryPreferences")]
    unsafe impl UIWindowSceneGeometryPreferencesVision {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
