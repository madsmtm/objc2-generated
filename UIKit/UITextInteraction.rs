//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextinteractionmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextInteractionMode(pub NSInteger);
impl UITextInteractionMode {
    #[doc(alias = "UITextInteractionModeEditable")]
    pub const Editable: Self = Self(0);
    #[doc(alias = "UITextInteractionModeNonEditable")]
    pub const NonEditable: Self = Self(1);
}

unsafe impl Encode for UITextInteractionMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextInteractionMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextinteractiondelegate?language=objc)
    pub unsafe trait UITextInteractionDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "objc2-core-foundation")]
        #[optional]
        #[method(interactionShouldBegin:atPoint:)]
        unsafe fn interactionShouldBegin_atPoint(
            &self,
            interaction: &UITextInteraction,
            point: CGPoint,
        ) -> bool;

        #[optional]
        #[method(interactionWillBegin:)]
        unsafe fn interactionWillBegin(&self, interaction: &UITextInteraction);

        #[optional]
        #[method(interactionDidEnd:)]
        unsafe fn interactionDidEnd(&self, interaction: &UITextInteraction);
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextInteraction;
);

unsafe impl NSObjectProtocol for UITextInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UITextInteraction {}

extern_methods!(
    unsafe impl UITextInteraction {
        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITextInteractionDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UITextInteractionDelegate>>,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits"
        ))]
        #[unsafe(method_family(none))]
        #[method_id(textInput)]
        pub unsafe fn textInput(&self) -> Option<Retained<UIResponder>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits"
        ))]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`textInput`][Self::textInput].
        #[method(setTextInput:)]
        pub unsafe fn setTextInput(&self, text_input: Option<&UIResponder>);

        #[method(textInteractionMode)]
        pub unsafe fn textInteractionMode(&self) -> UITextInteractionMode;

        #[cfg(feature = "UIGestureRecognizer")]
        #[unsafe(method_family(none))]
        #[method_id(gesturesForFailureRequirements)]
        pub unsafe fn gesturesForFailureRequirements(
            &self,
        ) -> Retained<NSArray<UIGestureRecognizer>>;

        #[unsafe(method_family(none))]
        #[method_id(textInteractionForMode:)]
        pub unsafe fn textInteractionForMode(
            mode: UITextInteractionMode,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextInteraction {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
