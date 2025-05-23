//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerInteraction;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIPointerInteraction {}
);

#[cfg(feature = "UIInteraction")]
extern_conformance!(
    unsafe impl UIInteraction for UIPointerInteraction {}
);

impl UIPointerInteraction {
    extern_methods!(
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPointerInteractionDelegate>>>;

        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[unsafe(method(initWithDelegate:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: Option<&ProtocolObject<dyn UIPointerInteractionDelegate>>,
        ) -> Retained<Self>;

        /// Call this method to cause the interaction to update the pointer in response to some event.
        #[unsafe(method(invalidate))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidate(&self);
    );
}

/// Methods declared on superclass `NSObject`.
impl UIPointerInteraction {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerinteractiondelegate?language=objc)
    pub unsafe trait UIPointerInteractionDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "UIPointerRegion")]
        /// Called as the pointer moves within the interaction's view.
        ///
        ///
        /// Parameter `interaction`: This UIPointerInteraction.
        ///
        /// Parameter `request`: Request object describing the pointer's location in the interaction's view.
        ///
        /// Parameter `defaultRegion`: Region representing the entire surface of the interaction's view.
        ///
        ///
        /// Returns: A UIPointerRegion in which to apply a pointer style. Return nil to indicate that this interaction should not customize the pointer for the current location.
        #[optional]
        #[unsafe(method(pointerInteraction:regionForRequest:defaultRegion:))]
        #[unsafe(method_family = none)]
        unsafe fn pointerInteraction_regionForRequest_defaultRegion(
            &self,
            interaction: &UIPointerInteraction,
            request: &UIPointerRegionRequest,
            default_region: &UIPointerRegion,
        ) -> Option<Retained<UIPointerRegion>>;

        #[cfg(all(
            feature = "UIHoverStyle",
            feature = "UIPointerRegion",
            feature = "UIPointerStyle"
        ))]
        /// Called after the interaction receives a new UIPointerRegion from pointerInteraction:regionForRequest:defaultRegion:.
        ///
        ///
        /// Parameter `interaction`: This UIPointerInteraction.
        ///
        /// Parameter `region`: The UIPointerRegion for which a style is being requested.
        ///
        ///
        /// Returns: A UIPointerStyle describing the desired hover effect or pointer appearance for the given UIPointerRegion.
        #[optional]
        #[unsafe(method(pointerInteraction:styleForRegion:))]
        #[unsafe(method_family = none)]
        unsafe fn pointerInteraction_styleForRegion(
            &self,
            interaction: &UIPointerInteraction,
            region: &UIPointerRegion,
        ) -> Option<Retained<UIPointerStyle>>;

        #[cfg(feature = "UIPointerRegion")]
        /// Called when the pointer enters a given region.
        ///
        ///
        /// Parameter `interaction`: This UIPointerInteraction.
        ///
        /// Parameter `region`: The UIPointerRegion the pointer is about to enter.
        ///
        /// Parameter `animator`: Region entrance animator. Add animations to run them alongside the pointer's entrance animation.
        #[optional]
        #[unsafe(method(pointerInteraction:willEnterRegion:animator:))]
        #[unsafe(method_family = none)]
        unsafe fn pointerInteraction_willEnterRegion_animator(
            &self,
            interaction: &UIPointerInteraction,
            region: &UIPointerRegion,
            animator: &ProtocolObject<dyn UIPointerInteractionAnimating>,
        );

        #[cfg(feature = "UIPointerRegion")]
        /// Called when the pointer exists a given region.
        ///
        ///
        /// Parameter `interaction`: This UIPointerInteraction.
        ///
        /// Parameter `region`: The UIPointerRegion the pointer is about to exit.
        ///
        /// Parameter `animator`: Region exit animator. Add animations to run them alongside the pointer's exit animation.
        #[optional]
        #[unsafe(method(pointerInteraction:willExitRegion:animator:))]
        #[unsafe(method_family = none)]
        unsafe fn pointerInteraction_willExitRegion_animator(
            &self,
            interaction: &UIPointerInteraction,
            region: &UIPointerRegion,
            animator: &ProtocolObject<dyn UIPointerInteractionAnimating>,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerregionrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerRegionRequest;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIPointerRegionRequest {}
);

impl UIPointerRegionRequest {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        /// The location of the pointer in the interaction's view's coordinate space.
        #[unsafe(method(location))]
        #[unsafe(method_family = none)]
        pub unsafe fn location(&self) -> CGPoint;

        #[cfg(feature = "UICommand")]
        /// Key modifier flags representing keyboard keys pressed by the user at the time of this request.
        #[unsafe(method(modifiers))]
        #[unsafe(method_family = none)]
        pub unsafe fn modifiers(&self) -> UIKeyModifierFlags;
    );
}

/// Methods declared on superclass `NSObject`.
impl UIPointerRegionRequest {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerinteractionanimating?language=objc)
    pub unsafe trait UIPointerInteractionAnimating:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "block2")]
        #[unsafe(method(addAnimations:))]
        #[unsafe(method_family = none)]
        unsafe fn addAnimations(&self, animations: &block2::DynBlock<dyn Fn()>);

        #[cfg(feature = "block2")]
        #[unsafe(method(addCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn addCompletion(&self, completion: &block2::DynBlock<dyn Fn(Bool)>);
    }
);
