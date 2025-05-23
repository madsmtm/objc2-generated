//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextdragoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextDragOptions(pub NSInteger);
bitflags::bitflags! {
    impl UITextDragOptions: NSInteger {
        #[doc(alias = "UITextDragOptionsNone")]
        const OptionsNone = 0;
        #[doc(alias = "UITextDragOptionStripTextColorFromPreviews")]
        const OptionStripTextColorFromPreviews = 1<<0;
    }
}

unsafe impl Encode for UITextDragOptions {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextDragOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextdraggable?language=objc)
    #[cfg(all(feature = "UITextInput", feature = "UITextInputTraits"))]
    pub unsafe trait UITextDraggable: UITextInput + MainThreadOnly {
        #[unsafe(method(textDragDelegate))]
        #[unsafe(method_family = none)]
        unsafe fn textDragDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITextDragDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`textDragDelegate`][Self::textDragDelegate].
        #[unsafe(method(setTextDragDelegate:))]
        #[unsafe(method_family = none)]
        unsafe fn setTextDragDelegate(
            &self,
            text_drag_delegate: Option<&ProtocolObject<dyn UITextDragDelegate>>,
        );

        #[cfg(feature = "UIDragInteraction")]
        #[unsafe(method(textDragInteraction))]
        #[unsafe(method_family = none)]
        unsafe fn textDragInteraction(&self) -> Option<Retained<UIDragInteraction>>;

        #[unsafe(method(isTextDragActive))]
        #[unsafe(method_family = none)]
        unsafe fn isTextDragActive(&self) -> bool;

        #[unsafe(method(textDragOptions))]
        #[unsafe(method_family = none)]
        unsafe fn textDragOptions(&self) -> UITextDragOptions;

        /// Setter for [`textDragOptions`][Self::textDragOptions].
        #[unsafe(method(setTextDragOptions:))]
        #[unsafe(method_family = none)]
        unsafe fn setTextDragOptions(&self, text_drag_options: UITextDragOptions);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextdragdelegate?language=objc)
    pub unsafe trait UITextDragDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "UIDragItem",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UIView"
        ))]
        #[optional]
        #[unsafe(method(textDraggableView:itemsForDrag:))]
        #[unsafe(method_family = none)]
        unsafe fn textDraggableView_itemsForDrag(
            &self,
            text_draggable_view: &UIView,
            drag_request: &ProtocolObject<dyn UITextDragRequest>,
        ) -> Retained<NSArray<UIDragItem>>;

        #[cfg(all(
            feature = "UIDragItem",
            feature = "UIDragSession",
            feature = "UIResponder",
            feature = "UITargetedDragPreview",
            feature = "UITargetedPreview",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UIView"
        ))]
        #[optional]
        #[unsafe(method(textDraggableView:dragPreviewForLiftingItem:session:))]
        #[unsafe(method_family = none)]
        unsafe fn textDraggableView_dragPreviewForLiftingItem_session(
            &self,
            text_draggable_view: &UIView,
            item: &UIDragItem,
            session: &ProtocolObject<dyn UIDragSession>,
        ) -> Option<Retained<UITargetedDragPreview>>;

        #[cfg(all(
            feature = "UIDragInteraction",
            feature = "UIDragSession",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UIView"
        ))]
        #[optional]
        #[unsafe(method(textDraggableView:willAnimateLiftWithAnimator:session:))]
        #[unsafe(method_family = none)]
        unsafe fn textDraggableView_willAnimateLiftWithAnimator_session(
            &self,
            text_draggable_view: &UIView,
            animator: &ProtocolObject<dyn UIDragAnimating>,
            session: &ProtocolObject<dyn UIDragSession>,
        );

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UIView"
        ))]
        #[optional]
        #[unsafe(method(textDraggableView:dragSessionWillBegin:))]
        #[unsafe(method_family = none)]
        unsafe fn textDraggableView_dragSessionWillBegin(
            &self,
            text_draggable_view: &UIView,
            session: &ProtocolObject<dyn UIDragSession>,
        );

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIDropInteraction",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UIView"
        ))]
        #[optional]
        #[unsafe(method(textDraggableView:dragSessionDidEnd:withOperation:))]
        #[unsafe(method_family = none)]
        unsafe fn textDraggableView_dragSessionDidEnd_withOperation(
            &self,
            text_draggable_view: &UIView,
            session: &ProtocolObject<dyn UIDragSession>,
            operation: UIDropOperation,
        );
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextdragrequest?language=objc)
    pub unsafe trait UITextDragRequest: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "UITextInput")]
        #[unsafe(method(dragRange))]
        #[unsafe(method_family = none)]
        unsafe fn dragRange(&self) -> Retained<UITextRange>;

        #[cfg(feature = "UIDragItem")]
        #[unsafe(method(suggestedItems))]
        #[unsafe(method_family = none)]
        unsafe fn suggestedItems(&self) -> Retained<NSArray<UIDragItem>>;

        #[cfg(feature = "UIDragItem")]
        #[unsafe(method(existingItems))]
        #[unsafe(method_family = none)]
        unsafe fn existingItems(&self) -> Retained<NSArray<UIDragItem>>;

        #[unsafe(method(isSelected))]
        #[unsafe(method_family = none)]
        unsafe fn isSelected(&self) -> bool;

        #[cfg(feature = "UIDragSession")]
        #[unsafe(method(dragSession))]
        #[unsafe(method_family = none)]
        unsafe fn dragSession(&self) -> Retained<ProtocolObject<dyn UIDragSession>>;
    }
);
