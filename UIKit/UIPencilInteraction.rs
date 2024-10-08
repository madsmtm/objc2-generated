//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPencilPreferredAction(pub NSInteger);
impl UIPencilPreferredAction {
    #[doc(alias = "UIPencilPreferredActionIgnore")]
    pub const Ignore: Self = Self(0);
    #[doc(alias = "UIPencilPreferredActionSwitchEraser")]
    pub const SwitchEraser: Self = Self(1);
    #[doc(alias = "UIPencilPreferredActionSwitchPrevious")]
    pub const SwitchPrevious: Self = Self(2);
    #[doc(alias = "UIPencilPreferredActionShowColorPalette")]
    pub const ShowColorPalette: Self = Self(3);
    #[doc(alias = "UIPencilPreferredActionShowInkAttributes")]
    pub const ShowInkAttributes: Self = Self(4);
    #[doc(alias = "UIPencilPreferredActionShowContextualPalette")]
    pub const ShowContextualPalette: Self = Self(5);
    #[doc(alias = "UIPencilPreferredActionRunSystemShortcut")]
    pub const RunSystemShortcut: Self = Self(6);
}

unsafe impl Encode for UIPencilPreferredAction {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPencilPreferredAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPencilInteractionPhase(pub NSUInteger);
impl UIPencilInteractionPhase {
    #[doc(alias = "UIPencilInteractionPhaseBegan")]
    pub const Began: Self = Self(0);
    #[doc(alias = "UIPencilInteractionPhaseChanged")]
    pub const Changed: Self = Self(1);
    #[doc(alias = "UIPencilInteractionPhaseEnded")]
    pub const Ended: Self = Self(2);
    #[doc(alias = "UIPencilInteractionPhaseCancelled")]
    pub const Cancelled: Self = Self(3);
}

unsafe impl Encode for UIPencilInteractionPhase {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIPencilInteractionPhase {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPencilInteraction;

    unsafe impl ClassType for UIPencilInteraction {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIPencilInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIPencilInteraction {}

extern_methods!(
    unsafe impl UIPencilInteraction {
        #[method(preferredTapAction)]
        pub unsafe fn preferredTapAction(mtm: MainThreadMarker) -> UIPencilPreferredAction;

        #[method(preferredSqueezeAction)]
        pub unsafe fn preferredSqueezeAction(mtm: MainThreadMarker) -> UIPencilPreferredAction;

        #[method(prefersPencilOnlyDrawing)]
        pub unsafe fn prefersPencilOnlyDrawing(mtm: MainThreadMarker) -> bool;

        #[method(prefersHoverToolPreview)]
        pub unsafe fn prefersHoverToolPreview(mtm: MainThreadMarker) -> bool;

        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: &ProtocolObject<dyn UIPencilInteractionDelegate>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPencilInteractionDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIPencilInteractionDelegate>>,
        );

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPencilInteraction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPencilHoverPose;

    unsafe impl ClassType for UIPencilHoverPose {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIPencilHoverPose {}

extern_methods!(
    unsafe impl UIPencilHoverPose {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(location)]
        pub unsafe fn location(&self) -> CGPoint;

        #[method(zOffset)]
        pub unsafe fn zOffset(&self) -> CGFloat;

        #[method(azimuthAngle)]
        pub unsafe fn azimuthAngle(&self) -> CGFloat;

        #[method(azimuthUnitVector)]
        pub unsafe fn azimuthUnitVector(&self) -> CGVector;

        #[method(altitudeAngle)]
        pub unsafe fn altitudeAngle(&self) -> CGFloat;

        #[method(rollAngle)]
        pub unsafe fn rollAngle(&self) -> CGFloat;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPencilInteractionTap;

    unsafe impl ClassType for UIPencilInteractionTap {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIPencilInteractionTap {}

extern_methods!(
    unsafe impl UIPencilInteractionTap {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;

        #[method_id(@__retain_semantics Other hoverPose)]
        pub unsafe fn hoverPose(&self) -> Option<Retained<UIPencilHoverPose>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPencilInteractionSqueeze;

    unsafe impl ClassType for UIPencilInteractionSqueeze {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIPencilInteractionSqueeze {}

extern_methods!(
    unsafe impl UIPencilInteractionSqueeze {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;

        #[method(phase)]
        pub unsafe fn phase(&self) -> UIPencilInteractionPhase;

        #[method_id(@__retain_semantics Other hoverPose)]
        pub unsafe fn hoverPose(&self) -> Option<Retained<UIPencilHoverPose>>;
    }
);

extern_protocol!(
    pub unsafe trait UIPencilInteractionDelegate: NSObjectProtocol + MainThreadOnly {
        #[deprecated = "Use pencilInteraction(_:didReceiveTap:) instead"]
        #[optional]
        #[method(pencilInteractionDidTap:)]
        unsafe fn pencilInteractionDidTap(&self, interaction: &UIPencilInteraction);

        #[optional]
        #[method(pencilInteraction:didReceiveTap:)]
        unsafe fn pencilInteraction_didReceiveTap(
            &self,
            interaction: &UIPencilInteraction,
            tap: &UIPencilInteractionTap,
        );

        #[optional]
        #[method(pencilInteraction:didReceiveSqueeze:)]
        unsafe fn pencilInteraction_didReceiveSqueeze(
            &self,
            interaction: &UIPencilInteraction,
            squeeze: &UIPencilInteractionSqueeze,
        );
    }

    unsafe impl ProtocolType for dyn UIPencilInteractionDelegate {}
);
