//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowscenepresentationstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIWindowScenePresentationStyle(pub NSUInteger);
impl UIWindowScenePresentationStyle {
    #[doc(alias = "UIWindowScenePresentationStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIWindowScenePresentationStyleStandard")]
    pub const Standard: Self = Self(1);
    #[doc(alias = "UIWindowScenePresentationStyleProminent")]
    pub const Prominent: Self = Self(2);
}

unsafe impl Encode for UIWindowScenePresentationStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIWindowScenePresentationStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowsceneactivationrequestoptions?language=objc)
    #[unsafe(super(UISceneActivationRequestOptions, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UISceneOptions")]
    pub struct UIWindowSceneActivationRequestOptions;
);

#[cfg(feature = "UISceneOptions")]
extern_conformance!(
    unsafe impl NSObjectProtocol for UIWindowSceneActivationRequestOptions {}
);

#[cfg(feature = "UISceneOptions")]
impl UIWindowSceneActivationRequestOptions {
    extern_methods!(
        #[deprecated = "Place use .placement with an appropriate UIWindowScenePlacement."]
        #[unsafe(method(preferredPresentationStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn preferredPresentationStyle(&self) -> UIWindowScenePresentationStyle;

        /// Setter for [`preferredPresentationStyle`][Self::preferredPresentationStyle].
        #[deprecated = "Place use .placement with an appropriate UIWindowScenePlacement."]
        #[unsafe(method(setPreferredPresentationStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPreferredPresentationStyle(
            &self,
            preferred_presentation_style: UIWindowScenePresentationStyle,
        );

        #[cfg(feature = "UIWindowScenePlacement")]
        /// The preferred placement of the window scene to be activated. Scene placements influence how the
        /// system positions the activated scene. A `nil` value indicates that the system should determine
        /// the most appropriate placement to use.
        #[unsafe(method(placement))]
        #[unsafe(method_family = none)]
        pub unsafe fn placement(&self) -> Option<Retained<UIWindowScenePlacement>>;

        #[cfg(feature = "UIWindowScenePlacement")]
        /// Setter for [`placement`][Self::placement].
        #[unsafe(method(setPlacement:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPlacement(&self, placement: Option<&UIWindowScenePlacement>);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "UISceneOptions")]
impl UIWindowSceneActivationRequestOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
