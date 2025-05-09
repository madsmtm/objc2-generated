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

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontrolevents?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIControlEvents(pub NSUInteger);
bitflags::bitflags! {
    impl UIControlEvents: NSUInteger {
        #[doc(alias = "UIControlEventTouchDown")]
        const TouchDown = 1<<0;
        #[doc(alias = "UIControlEventTouchDownRepeat")]
        const TouchDownRepeat = 1<<1;
        #[doc(alias = "UIControlEventTouchDragInside")]
        const TouchDragInside = 1<<2;
        #[doc(alias = "UIControlEventTouchDragOutside")]
        const TouchDragOutside = 1<<3;
        #[doc(alias = "UIControlEventTouchDragEnter")]
        const TouchDragEnter = 1<<4;
        #[doc(alias = "UIControlEventTouchDragExit")]
        const TouchDragExit = 1<<5;
        #[doc(alias = "UIControlEventTouchUpInside")]
        const TouchUpInside = 1<<6;
        #[doc(alias = "UIControlEventTouchUpOutside")]
        const TouchUpOutside = 1<<7;
        #[doc(alias = "UIControlEventTouchCancel")]
        const TouchCancel = 1<<8;
        #[doc(alias = "UIControlEventValueChanged")]
        const ValueChanged = 1<<12;
        #[doc(alias = "UIControlEventPrimaryActionTriggered")]
        const PrimaryActionTriggered = 1<<13;
        #[doc(alias = "UIControlEventMenuActionTriggered")]
        const MenuActionTriggered = 1<<14;
        #[doc(alias = "UIControlEventEditingDidBegin")]
        const EditingDidBegin = 1<<16;
        #[doc(alias = "UIControlEventEditingChanged")]
        const EditingChanged = 1<<17;
        #[doc(alias = "UIControlEventEditingDidEnd")]
        const EditingDidEnd = 1<<18;
        #[doc(alias = "UIControlEventEditingDidEndOnExit")]
        const EditingDidEndOnExit = 1<<19;
        #[doc(alias = "UIControlEventAllTouchEvents")]
        const AllTouchEvents = 0x00000FFF;
        #[doc(alias = "UIControlEventAllEditingEvents")]
        const AllEditingEvents = 0x000F0000;
        #[doc(alias = "UIControlEventApplicationReserved")]
        const ApplicationReserved = 0x0F000000;
        #[doc(alias = "UIControlEventSystemReserved")]
        const SystemReserved = 0xF0000000;
        #[doc(alias = "UIControlEventAllEvents")]
        const AllEvents = 0xFFFFFFFF;
    }
}

unsafe impl Encode for UIControlEvents {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIControlEvents {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontrolcontentverticalalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIControlContentVerticalAlignment(pub NSInteger);
impl UIControlContentVerticalAlignment {
    #[doc(alias = "UIControlContentVerticalAlignmentCenter")]
    pub const Center: Self = Self(0);
    #[doc(alias = "UIControlContentVerticalAlignmentTop")]
    pub const Top: Self = Self(1);
    #[doc(alias = "UIControlContentVerticalAlignmentBottom")]
    pub const Bottom: Self = Self(2);
    #[doc(alias = "UIControlContentVerticalAlignmentFill")]
    pub const Fill: Self = Self(3);
}

unsafe impl Encode for UIControlContentVerticalAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIControlContentVerticalAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontrolcontenthorizontalalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIControlContentHorizontalAlignment(pub NSInteger);
impl UIControlContentHorizontalAlignment {
    #[doc(alias = "UIControlContentHorizontalAlignmentCenter")]
    pub const Center: Self = Self(0);
    #[doc(alias = "UIControlContentHorizontalAlignmentLeft")]
    pub const Left: Self = Self(1);
    #[doc(alias = "UIControlContentHorizontalAlignmentRight")]
    pub const Right: Self = Self(2);
    #[doc(alias = "UIControlContentHorizontalAlignmentFill")]
    pub const Fill: Self = Self(3);
    #[doc(alias = "UIControlContentHorizontalAlignmentLeading")]
    pub const Leading: Self = Self(4);
    #[doc(alias = "UIControlContentHorizontalAlignmentTrailing")]
    pub const Trailing: Self = Self(5);
}

unsafe impl Encode for UIControlContentHorizontalAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIControlContentHorizontalAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontrolstate?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIControlState(pub NSUInteger);
bitflags::bitflags! {
    impl UIControlState: NSUInteger {
        #[doc(alias = "UIControlStateNormal")]
        const Normal = 0;
        #[doc(alias = "UIControlStateHighlighted")]
        const Highlighted = 1<<0;
        #[doc(alias = "UIControlStateDisabled")]
        const Disabled = 1<<1;
        #[doc(alias = "UIControlStateSelected")]
        const Selected = 1<<2;
        #[doc(alias = "UIControlStateFocused")]
        const Focused = 1<<3;
        #[doc(alias = "UIControlStateApplication")]
        const Application = 0x00FF0000;
        #[doc(alias = "UIControlStateReserved")]
        const Reserved = 0xFF000000;
    }
}

unsafe impl Encode for UIControlState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIControlState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontrol?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIControl;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
extern_conformance!(
    unsafe impl CALayerDelegate for UIControl {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSCoding for UIControl {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for UIControl {}
);

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIAppearance for UIControl {}
);

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIAppearanceContainer for UIControl {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UICoordinateSpace for UIControl {}
);

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIDynamicItem for UIControl {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusEnvironment for UIControl {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusItem for UIControl {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusItemContainer for UIControl {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIResponderStandardEditActions for UIControl {}
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITraitEnvironment for UIControl {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UIControl {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "UIAction",
            feature = "UIMenuElement",
            feature = "objc2-core-foundation"
        ))]
        /// Initializes the control and adds primaryAction for the UIControlEventPrimaryActionTriggered control event. Subclasses of UIControl may alter or add behaviors around the usage of primaryAction, see subclass documentation of this initializer for additional information.
        #[unsafe(method(initWithFrame:primaryAction:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Retained<Self>;

        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[unsafe(method(isSelected))]
        #[unsafe(method_family = none)]
        pub unsafe fn isSelected(&self) -> bool;

        /// Setter for [`isSelected`][Self::isSelected].
        #[unsafe(method(setSelected:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[unsafe(method(isHighlighted))]
        #[unsafe(method_family = none)]
        pub unsafe fn isHighlighted(&self) -> bool;

        /// Setter for [`isHighlighted`][Self::isHighlighted].
        #[unsafe(method(setHighlighted:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[unsafe(method(contentVerticalAlignment))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentVerticalAlignment(&self) -> UIControlContentVerticalAlignment;

        /// Setter for [`contentVerticalAlignment`][Self::contentVerticalAlignment].
        #[unsafe(method(setContentVerticalAlignment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentVerticalAlignment(
            &self,
            content_vertical_alignment: UIControlContentVerticalAlignment,
        );

        #[unsafe(method(contentHorizontalAlignment))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentHorizontalAlignment(&self) -> UIControlContentHorizontalAlignment;

        /// Setter for [`contentHorizontalAlignment`][Self::contentHorizontalAlignment].
        #[unsafe(method(setContentHorizontalAlignment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentHorizontalAlignment(
            &self,
            content_horizontal_alignment: UIControlContentHorizontalAlignment,
        );

        #[unsafe(method(effectiveContentHorizontalAlignment))]
        #[unsafe(method_family = none)]
        pub unsafe fn effectiveContentHorizontalAlignment(
            &self,
        ) -> UIControlContentHorizontalAlignment;

        #[unsafe(method(state))]
        #[unsafe(method_family = none)]
        pub unsafe fn state(&self) -> UIControlState;

        #[unsafe(method(isTracking))]
        #[unsafe(method_family = none)]
        pub unsafe fn isTracking(&self) -> bool;

        #[unsafe(method(isTouchInside))]
        #[unsafe(method_family = none)]
        pub unsafe fn isTouchInside(&self) -> bool;

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[unsafe(method(beginTrackingWithTouch:withEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn beginTrackingWithTouch_withEvent(
            &self,
            touch: &UITouch,
            event: Option<&UIEvent>,
        ) -> bool;

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[unsafe(method(continueTrackingWithTouch:withEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn continueTrackingWithTouch_withEvent(
            &self,
            touch: &UITouch,
            event: Option<&UIEvent>,
        ) -> bool;

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[unsafe(method(endTrackingWithTouch:withEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn endTrackingWithTouch_withEvent(
            &self,
            touch: Option<&UITouch>,
            event: Option<&UIEvent>,
        );

        #[cfg(feature = "UIEvent")]
        #[unsafe(method(cancelTrackingWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn cancelTrackingWithEvent(&self, event: Option<&UIEvent>);

        #[unsafe(method(addTarget:action:forControlEvents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addTarget_action_forControlEvents(
            &self,
            target: Option<&AnyObject>,
            action: Sel,
            control_events: UIControlEvents,
        );

        #[unsafe(method(removeTarget:action:forControlEvents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeTarget_action_forControlEvents(
            &self,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            control_events: UIControlEvents,
        );

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        /// Adds the UIAction to a given event. UIActions are uniqued based on their identifier, and subsequent actions with the same identifier replace previously added actions. You may add multiple UIActions for corresponding controlEvents, and you may add the same action to multiple controlEvents.
        #[unsafe(method(addAction:forControlEvents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addAction_forControlEvents(
            &self,
            action: &UIAction,
            control_events: UIControlEvents,
        );

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        /// Removes the action from the set of passed control events.
        #[unsafe(method(removeAction:forControlEvents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeAction_forControlEvents(
            &self,
            action: &UIAction,
            control_events: UIControlEvents,
        );

        #[cfg(feature = "UIAction")]
        /// Removes the action with the provided identifier from the set of passed control events.
        #[unsafe(method(removeActionForIdentifier:forControlEvents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeActionForIdentifier_forControlEvents(
            &self,
            action_identifier: &UIActionIdentifier,
            control_events: UIControlEvents,
        );

        /// Performs the control's primary action.
        #[unsafe(method(performPrimaryAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn performPrimaryAction(&self);

        #[unsafe(method(allTargets))]
        #[unsafe(method_family = none)]
        pub unsafe fn allTargets(&self) -> Retained<NSSet>;

        #[unsafe(method(allControlEvents))]
        #[unsafe(method_family = none)]
        pub unsafe fn allControlEvents(&self) -> UIControlEvents;

        #[unsafe(method(actionsForTarget:forControlEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn actionsForTarget_forControlEvent(
            &self,
            target: Option<&AnyObject>,
            control_event: UIControlEvents,
        ) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement", feature = "block2"))]
        /// Iterate over the event handlers installed on this control at the time this method is called. For each call, either actionHandler or action will be non-nil. controlEvents is always non-zero. Setting *stop to YES will terminate the enumeration early. It is legal to manipulate the control's event handlers within the block.
        #[unsafe(method(enumerateEventHandlers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn enumerateEventHandlers(
            &self,
            iterator: &block2::DynBlock<
                dyn Fn(*mut UIAction, *mut AnyObject, Option<Sel>, UIControlEvents, NonNull<Bool>)
                    + '_,
            >,
        );

        #[cfg(feature = "UIEvent")]
        /// Dispatch the target-action pair. This method is called repeatedly by -sendActionsForControlEvents: and is a point at which you can observe or override behavior.
        #[unsafe(method(sendAction:to:forEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sendAction_to_forEvent(
            &self,
            action: Sel,
            target: Option<&AnyObject>,
            event: Option<&UIEvent>,
        );

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        /// Like -sendAction:to:forEvent:, this method is called by -sendActionsForControlEvents:. You may override this method to observe or modify behavior. If you override this method, you should call super precisely once to dispatch the action, or not call super to suppress sending that action.
        #[unsafe(method(sendAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sendAction(&self, action: &UIAction);

        /// send all actions associated with the given control events
        #[unsafe(method(sendActionsForControlEvents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sendActionsForControlEvents(&self, control_events: UIControlEvents);

        #[cfg(feature = "UIContextMenuInteraction")]
        /// Returns a UIContextMenuInteraction with this control set as its delegate. Before constructing the UIContextMenuInteraction, UIControl verifies 'self' is a viable delegate. See 'Implementing UIControl Menus' below for more details.
        #[unsafe(method(contextMenuInteraction))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextMenuInteraction(&self) -> Option<Retained<UIContextMenuInteraction>>;

        /// Specifies if the context menu interaction is enabled. NO by default.
        #[unsafe(method(isContextMenuInteractionEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isContextMenuInteractionEnabled(&self) -> bool;

        /// Setter for [`isContextMenuInteractionEnabled`][Self::isContextMenuInteractionEnabled].
        #[unsafe(method(setContextMenuInteractionEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContextMenuInteractionEnabled(
            &self,
            context_menu_interaction_enabled: bool,
        );

        /// If the contextMenuInteraction is the primary action of the control, invoked on touch-down. NO by default.
        #[unsafe(method(showsMenuAsPrimaryAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn showsMenuAsPrimaryAction(&self) -> bool;

        /// Setter for [`showsMenuAsPrimaryAction`][Self::showsMenuAsPrimaryAction].
        #[unsafe(method(setShowsMenuAsPrimaryAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShowsMenuAsPrimaryAction(&self, shows_menu_as_primary_action: bool);

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "objc2-core-foundation"
        ))]
        /// Return a point in this control's coordinate space to which to attach the given configuration's menu.
        #[unsafe(method(menuAttachmentPointForConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn menuAttachmentPointForConfiguration(
            &self,
            configuration: &UIContextMenuConfiguration,
        ) -> CGPoint;

        /// Assigning a value to this property causes the tool tip to be displayed for the view. Setting the property to nil cancels the display of the tool tip for the view.
        #[unsafe(method(toolTip))]
        #[unsafe(method_family = none)]
        pub unsafe fn toolTip(&self) -> Option<Retained<NSString>>;

        /// Setter for [`toolTip`][Self::toolTip].
        #[unsafe(method(setToolTip:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[cfg(feature = "UIToolTipInteraction")]
        /// Returns the control's default UIToolTipInteraction.
        #[unsafe(method(toolTipInteraction))]
        #[unsafe(method_family = none)]
        pub unsafe fn toolTipInteraction(&self) -> Option<Retained<UIToolTipInteraction>>;

        /// Whether or not symbol animations are enabled for this control.
        /// The default value varies depending on the control.
        #[unsafe(method(isSymbolAnimationEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isSymbolAnimationEnabled(&self) -> bool;

        /// Setter for [`isSymbolAnimationEnabled`][Self::isSymbolAnimationEnabled].
        #[unsafe(method(setSymbolAnimationEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSymbolAnimationEnabled(&self, symbol_animation_enabled: bool);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UIControl {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

/// Implementing UIControl Menus
/// UIControl will only create a UIContextMenuInteraction if you've created a subclass of UIControl with a minimum viable delegate implementation. UIControl extends the contract of UIContextMenuInteractionDelegate for these methods, see each method for more detail.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UIControl {
    extern_methods!(
        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction",
            feature = "objc2-core-foundation"
        ))]
        /// An override is required for UIControl to create a UIContextMenuInteraction. Direct UIControl subclasses do not need to call super.
        #[unsafe(method(contextMenuInteraction:configurationForMenuAtLocation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextMenuInteraction_configurationForMenuAtLocation(
            &self,
            interaction: &UIContextMenuInteraction,
            location: CGPoint,
        ) -> Option<Retained<UIContextMenuConfiguration>>;

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction",
            feature = "UITargetedPreview"
        ))]
        /// Direct UIControl subclasses do not need to call super.
        #[unsafe(method(contextMenuInteraction:previewForHighlightingMenuWithConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextMenuInteraction_previewForHighlightingMenuWithConfiguration(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
        ) -> Option<Retained<UITargetedPreview>>;

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction",
            feature = "UITargetedPreview"
        ))]
        /// Direct UIControl subclasses do not need to call super.
        #[unsafe(method(contextMenuInteraction:previewForDismissingMenuWithConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextMenuInteraction_previewForDismissingMenuWithConfiguration(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
        ) -> Option<Retained<UITargetedPreview>>;

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction"
        ))]
        /// UIControl subclasses should always call super.
        #[unsafe(method(contextMenuInteraction:willDisplayMenuForConfiguration:animator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextMenuInteraction_willDisplayMenuForConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: Option<&ProtocolObject<dyn UIContextMenuInteractionAnimating>>,
        );

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction"
        ))]
        /// UIControl subclasses should always call super.
        #[unsafe(method(contextMenuInteraction:willEndForConfiguration:animator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextMenuInteraction_willEndForConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: Option<&ProtocolObject<dyn UIContextMenuInteractionAnimating>>,
        );

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction"
        ))]
        /// UIControl based menus do not display previews, so this method will not be called even if implemented. UIControl does not have an implementation.
        #[unsafe(method(contextMenuInteraction:willPerformPreviewActionForMenuWithConfiguration:animator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextMenuInteraction_willPerformPreviewActionForMenuWithConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: &ProtocolObject<dyn UIContextMenuInteractionCommitAnimating>,
        );
    );
}

#[cfg(all(
    feature = "UIContextMenuInteraction",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIContextMenuInteractionDelegate for UIControl {}
);
