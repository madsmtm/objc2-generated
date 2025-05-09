//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextdropaction?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextDropAction(pub NSUInteger);
impl UITextDropAction {
    #[doc(alias = "UITextDropActionInsert")]
    pub const Insert: Self = Self(0);
    #[doc(alias = "UITextDropActionReplaceSelection")]
    pub const ReplaceSelection: Self = Self(1);
    #[doc(alias = "UITextDropActionReplaceAll")]
    pub const ReplaceAll: Self = Self(2);
}

unsafe impl Encode for UITextDropAction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UITextDropAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextdropprogressmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextDropProgressMode(pub NSUInteger);
impl UITextDropProgressMode {
    #[doc(alias = "UITextDropProgressModeSystem")]
    pub const System: Self = Self(0);
    #[doc(alias = "UITextDropProgressModeCustom")]
    pub const Custom: Self = Self(1);
}

unsafe impl Encode for UITextDropProgressMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UITextDropProgressMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextdropperformer?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextDropPerformer(pub NSUInteger);
impl UITextDropPerformer {
    #[doc(alias = "UITextDropPerformerView")]
    pub const View: Self = Self(0);
    #[doc(alias = "UITextDropPerformerDelegate")]
    pub const Delegate: Self = Self(1);
}

unsafe impl Encode for UITextDropPerformer {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UITextDropPerformer {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextdropproposal?language=objc)
    #[unsafe(super(UIDropProposal, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDropInteraction")]
    pub struct UITextDropProposal;
);

#[cfg(feature = "UIDropInteraction")]
extern_conformance!(
    unsafe impl NSCopying for UITextDropProposal {}
);

#[cfg(feature = "UIDropInteraction")]
unsafe impl CopyingHelper for UITextDropProposal {
    type Result = Self;
}

#[cfg(feature = "UIDropInteraction")]
extern_conformance!(
    unsafe impl NSObjectProtocol for UITextDropProposal {}
);

#[cfg(feature = "UIDropInteraction")]
impl UITextDropProposal {
    extern_methods!(
        #[unsafe(method(dropAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn dropAction(&self) -> UITextDropAction;

        /// Setter for [`dropAction`][Self::dropAction].
        #[unsafe(method(setDropAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDropAction(&self, drop_action: UITextDropAction);

        #[unsafe(method(dropProgressMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn dropProgressMode(&self) -> UITextDropProgressMode;

        /// Setter for [`dropProgressMode`][Self::dropProgressMode].
        #[unsafe(method(setDropProgressMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDropProgressMode(&self, drop_progress_mode: UITextDropProgressMode);

        #[unsafe(method(useFastSameViewOperations))]
        #[unsafe(method_family = none)]
        pub unsafe fn useFastSameViewOperations(&self) -> bool;

        /// Setter for [`useFastSameViewOperations`][Self::useFastSameViewOperations].
        #[unsafe(method(setUseFastSameViewOperations:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUseFastSameViewOperations(&self, use_fast_same_view_operations: bool);

        #[unsafe(method(dropPerformer))]
        #[unsafe(method_family = none)]
        pub unsafe fn dropPerformer(&self) -> UITextDropPerformer;

        /// Setter for [`dropPerformer`][Self::dropPerformer].
        #[unsafe(method(setDropPerformer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDropPerformer(&self, drop_performer: UITextDropPerformer);
    );
}

/// Methods declared on superclass `UIDropProposal`.
#[cfg(feature = "UIDropInteraction")]
impl UITextDropProposal {
    extern_methods!(
        #[unsafe(method(initWithDropOperation:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDropOperation(
            this: Allocated<Self>,
            operation: UIDropOperation,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
