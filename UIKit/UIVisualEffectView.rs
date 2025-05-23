//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uivisualeffectview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIVisualEffectView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
extern_conformance!(
    unsafe impl CALayerDelegate for UIVisualEffectView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSCoding for UIVisualEffectView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for UIVisualEffectView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSSecureCoding for UIVisualEffectView {}
);

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIAppearance for UIVisualEffectView {}
);

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIAppearanceContainer for UIVisualEffectView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UICoordinateSpace for UIVisualEffectView {}
);

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIDynamicItem for UIVisualEffectView {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusEnvironment for UIVisualEffectView {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusItem for UIVisualEffectView {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusItemContainer for UIVisualEffectView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIResponderStandardEditActions for UIVisualEffectView {}
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITraitEnvironment for UIVisualEffectView {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UIVisualEffectView {
    extern_methods!(
        #[unsafe(method(contentView))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentView(&self) -> Retained<UIView>;

        #[cfg(feature = "UIVisualEffect")]
        #[unsafe(method(effect))]
        #[unsafe(method_family = none)]
        pub unsafe fn effect(&self) -> Option<Retained<UIVisualEffect>>;

        #[cfg(feature = "UIVisualEffect")]
        /// Setter for [`effect`][Self::effect].
        #[unsafe(method(setEffect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEffect(&self, effect: Option<&UIVisualEffect>);

        #[cfg(feature = "UIVisualEffect")]
        #[unsafe(method(initWithEffect:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithEffect(
            this: Allocated<Self>,
            effect: Option<&UIVisualEffect>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `UIView`.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UIVisualEffectView {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UIVisualEffectView {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
