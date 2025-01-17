//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// Use this class to express macOS-specific geometry preferences when calling `-[UIWindowScene requestGeometryUpdateWithPreferences:errorHandler:]`
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowscenegeometrypreferencesmac?language=objc)
    #[unsafe(super(UIWindowSceneGeometryPreferences, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIWindowSceneGeometryPreferences")]
    pub struct UIWindowSceneGeometryPreferencesMac;
);

#[cfg(feature = "UIWindowSceneGeometryPreferences")]
unsafe impl NSObjectProtocol for UIWindowSceneGeometryPreferencesMac {}

extern_methods!(
    #[cfg(feature = "UIWindowSceneGeometryPreferences")]
    unsafe impl UIWindowSceneGeometryPreferencesMac {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(init))]
        #[method_id(initWithSystemFrame:)]
        pub unsafe fn initWithSystemFrame(
            this: Allocated<Self>,
            system_frame: CGRect,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// The preferred system frame in the system scale where an origin of (0, 0) corresponds to the top-left
        /// corner of the main display. Defaults to CGRectNull to indicate no preference.
        #[method(systemFrame)]
        pub unsafe fn systemFrame(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`systemFrame`][Self::systemFrame].
        #[method(setSystemFrame:)]
        pub unsafe fn setSystemFrame(&self, system_frame: CGRect);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIWindowSceneGeometryPreferences`
    #[cfg(feature = "UIWindowSceneGeometryPreferences")]
    unsafe impl UIWindowSceneGeometryPreferencesMac {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
