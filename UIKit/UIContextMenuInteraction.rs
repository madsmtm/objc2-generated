//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontextmenuinteractioncommitstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIContextMenuInteractionCommitStyle(pub NSInteger);
impl UIContextMenuInteractionCommitStyle {
    #[doc(alias = "UIContextMenuInteractionCommitStyleDismiss")]
    pub const Dismiss: Self = Self(0);
    #[doc(alias = "UIContextMenuInteractionCommitStylePop")]
    pub const Pop: Self = Self(1);
}

unsafe impl Encode for UIContextMenuInteractionCommitStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIContextMenuInteractionCommitStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontextmenuinteractionappearance?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIContextMenuInteractionAppearance(pub NSInteger);
impl UIContextMenuInteractionAppearance {
    #[doc(alias = "UIContextMenuInteractionAppearanceUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "UIContextMenuInteractionAppearanceRich")]
    pub const Rich: Self = Self(1);
    #[doc(alias = "UIContextMenuInteractionAppearanceCompact")]
    pub const Compact: Self = Self(2);
}

unsafe impl Encode for UIContextMenuInteractionAppearance {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIContextMenuInteractionAppearance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontextmenuinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIContextMenuInteraction;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIContextMenuInteraction {}
);

#[cfg(feature = "UIInteraction")]
extern_conformance!(
    unsafe impl UIInteraction for UIContextMenuInteraction {}
);

impl UIContextMenuInteraction {
    extern_methods!(
        /// The interaction's delegate.
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIContextMenuInteractionDelegate>>>;

        /// Appearance of the menu this interaction has presented or is about to present. Since it may be
        /// dependent on the user's input method, the appearance is only known while the interaction is active.
        #[unsafe(method(menuAppearance))]
        #[unsafe(method_family = none)]
        pub unsafe fn menuAppearance(&self) -> UIContextMenuInteractionAppearance;

        #[unsafe(method(initWithDelegate:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: &ProtocolObject<dyn UIContextMenuInteractionDelegate>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        /// Returns the interaction's location within the given view.
        ///
        ///
        /// Parameter `view`: The view in which to locate the interaction.
        #[unsafe(method(locationInView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn locationInView(&self, view: Option<&UIView>) -> CGPoint;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement", feature = "block2"))]
        /// Call to update the currently visible menu. This method does nothing if called before a menu is presented.
        ///
        ///
        /// Parameter `block`: Called with a mutable copy of the currently visible menu. Modify and return this menu (or an entirely
        /// new one) to change the currently visible menu items. Starting in iOS 15, this block is called once for
        /// every visible submenu. For example, in the following hierarchy:
        ///
        /// *- Root Menu
        /// *- Submenu A
        /// *- Submenu B
        /// *- Submenu C
        ///
        /// If Submenu A is visible, the block is called twice (once for the Root Menu and once for Submenu A).
        /// If both A and B are visible, it's called 3 times (for the Root Menu, A, and B).
        #[unsafe(method(updateVisibleMenuWithBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateVisibleMenuWithBlock(
            &self,
            block: &block2::DynBlock<dyn Fn(NonNull<UIMenu>) -> NonNull<UIMenu> + '_>,
        );

        /// Dismisses the currently presented menu (if there is one).
        #[unsafe(method(dismissMenu))]
        #[unsafe(method_family = none)]
        pub unsafe fn dismissMenu(&self);
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontextmenuinteractionanimating?language=objc)
    pub unsafe trait UIContextMenuInteractionAnimating:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        /// Displayed preview view controller.
        #[unsafe(method(previewViewController))]
        #[unsafe(method_family = none)]
        unsafe fn previewViewController(&self) -> Option<Retained<UIViewController>>;

        #[cfg(feature = "block2")]
        #[unsafe(method(addAnimations:))]
        #[unsafe(method_family = none)]
        unsafe fn addAnimations(&self, animations: &block2::DynBlock<dyn Fn()>);

        #[cfg(feature = "block2")]
        #[unsafe(method(addCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn addCompletion(&self, completion: &block2::DynBlock<dyn Fn()>);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontextmenuinteractioncommitanimating?language=objc)
    pub unsafe trait UIContextMenuInteractionCommitAnimating:
        UIContextMenuInteractionAnimating + MainThreadOnly
    {
        /// Preferred animation style for the menu's commit action. Triggered when the user taps the preview.
        #[unsafe(method(preferredCommitStyle))]
        #[unsafe(method_family = none)]
        unsafe fn preferredCommitStyle(&self) -> UIContextMenuInteractionCommitStyle;

        /// Setter for [`preferredCommitStyle`][Self::preferredCommitStyle].
        #[unsafe(method(setPreferredCommitStyle:))]
        #[unsafe(method_family = none)]
        unsafe fn setPreferredCommitStyle(
            &self,
            preferred_commit_style: UIContextMenuInteractionCommitStyle,
        );
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontextmenuinteractiondelegate?language=objc)
    pub unsafe trait UIContextMenuInteractionDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "objc2-core-foundation"
        ))]
        /// Called when the interaction begins.
        ///
        ///
        /// Parameter `interaction`: The UIContextMenuInteraction.
        ///
        /// Parameter `location`: The location of the interaction in its view.
        ///
        ///
        /// Returns: A UIContextMenuConfiguration describing the menu to be presented. Return nil to prevent the interaction from beginning.
        /// Returning an empty configuration causes the interaction to begin then fail with a cancellation effect. You might use this
        /// to indicate to users that it's possible for a menu to be presented from this view, but that there are no actions to
        /// present at this particular time.
        #[unsafe(method(contextMenuInteraction:configurationForMenuAtLocation:))]
        #[unsafe(method_family = none)]
        unsafe fn contextMenuInteraction_configurationForMenuAtLocation(
            &self,
            interaction: &UIContextMenuInteraction,
            location: CGPoint,
        ) -> Option<Retained<UIContextMenuConfiguration>>;

        #[cfg(all(feature = "UIContextMenuConfiguration", feature = "UITargetedPreview"))]
        /// Called when a context menu interaction begins. Return a UITargetedPreview corresponding to the item with the given identifier.
        ///
        ///
        /// Parameter `interaction`: The UIContextMenuInteraction.
        ///
        /// Parameter `configuration`: Configuration of the menu that will be presented if the interaction proceeds.
        ///
        /// Parameter `identifier`: Identifier for the item whose preview is being requested.
        #[optional]
        #[unsafe(method(contextMenuInteraction:configuration:highlightPreviewForItemWithIdentifier:))]
        #[unsafe(method_family = none)]
        unsafe fn contextMenuInteraction_configuration_highlightPreviewForItemWithIdentifier(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            identifier: &ProtocolObject<dyn NSCopying>,
        ) -> Option<Retained<UITargetedPreview>>;

        #[cfg(all(feature = "UIContextMenuConfiguration", feature = "UITargetedPreview"))]
        /// Called when a context menu is dismissed. Return a UITargetedPreview corresponding to the item with the given identifier.
        ///
        ///
        /// Parameter `interaction`: The UIContextMenuInteraction.
        ///
        /// Parameter `configuration`: Configuration of the menu being dismissed.
        ///
        /// Parameter `identifier`: Identifier for the item whose preview is being requested.
        #[optional]
        #[unsafe(method(contextMenuInteraction:configuration:dismissalPreviewForItemWithIdentifier:))]
        #[unsafe(method_family = none)]
        unsafe fn contextMenuInteraction_configuration_dismissalPreviewForItemWithIdentifier(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            identifier: &ProtocolObject<dyn NSCopying>,
        ) -> Option<Retained<UITargetedPreview>>;

        #[cfg(feature = "UIContextMenuConfiguration")]
        /// Called when the interaction is about to "commit" in response to the user tapping the preview.
        ///
        ///
        /// Parameter `interaction`: The UIContextMenuInteraction.
        ///
        /// Parameter `configuration`: Configuration of the currently displayed menu.
        ///
        /// Parameter `animator`: Commit animator. Add animations to this object to run them alongside the commit transition.
        #[optional]
        #[unsafe(method(contextMenuInteraction:willPerformPreviewActionForMenuWithConfiguration:animator:))]
        #[unsafe(method_family = none)]
        unsafe fn contextMenuInteraction_willPerformPreviewActionForMenuWithConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: &ProtocolObject<dyn UIContextMenuInteractionCommitAnimating>,
        );

        #[cfg(feature = "UIContextMenuConfiguration")]
        /// Called when the interaction is about to display a menu.
        ///
        ///
        /// Parameter `interaction`: The UIContextMenuInteraction.
        ///
        /// Parameter `configuration`: The configuration of the menu about to be displayed by this interaction.
        ///
        /// Parameter `animator`: Appearance animator. Add animations to run them alongside the appearance transition.
        #[optional]
        #[unsafe(method(contextMenuInteraction:willDisplayMenuForConfiguration:animator:))]
        #[unsafe(method_family = none)]
        unsafe fn contextMenuInteraction_willDisplayMenuForConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: Option<&ProtocolObject<dyn UIContextMenuInteractionAnimating>>,
        );

        #[cfg(feature = "UIContextMenuConfiguration")]
        /// Called when the interaction is about to end.
        ///
        ///
        /// Parameter `interaction`: The UIContextMenuInteraction.
        ///
        /// Parameter `configuration`: Ending configuration.
        ///
        /// Parameter `animator`: Disappearance animator. Add animations to run them alongside the disappearance transition.
        #[optional]
        #[unsafe(method(contextMenuInteraction:willEndForConfiguration:animator:))]
        #[unsafe(method_family = none)]
        unsafe fn contextMenuInteraction_willEndForConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: Option<&ProtocolObject<dyn UIContextMenuInteractionAnimating>>,
        );

        #[cfg(all(feature = "UIContextMenuConfiguration", feature = "UITargetedPreview"))]
        /// Called when the interaction begins. Return a UITargetedPreview describing the desired highlight preview.
        ///
        ///
        /// Parameter `interaction`: The UIContextMenuInteraction requesting a highlighting preview.
        ///
        /// Parameter `configuration`: The configuration of the menu about to be displayed by this interaction.
        #[deprecated]
        #[optional]
        #[unsafe(method(contextMenuInteraction:previewForHighlightingMenuWithConfiguration:))]
        #[unsafe(method_family = none)]
        unsafe fn contextMenuInteraction_previewForHighlightingMenuWithConfiguration(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
        ) -> Option<Retained<UITargetedPreview>>;

        #[cfg(all(feature = "UIContextMenuConfiguration", feature = "UITargetedPreview"))]
        /// Called when the interaction is about to dismiss. Return a UITargetedPreview describing the desired dismissal target.
        /// The interaction will animate the presented menu to the target. Use this to customize the dismissal animation.
        ///
        ///
        /// Parameter `interaction`: The UIContextMenuInteraction requesting a dismissal preview.
        ///
        /// Parameter `configuration`: The configuration of the menu displayed by this interaction.
        ///
        ///
        /// Returns: Return a UITargetedPreview describing the desired dismissal target. Return nil to cause the menu to
        /// animate away without morphing into a specific view.
        #[deprecated]
        #[optional]
        #[unsafe(method(contextMenuInteraction:previewForDismissingMenuWithConfiguration:))]
        #[unsafe(method_family = none)]
        unsafe fn contextMenuInteraction_previewForDismissingMenuWithConfiguration(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
        ) -> Option<Retained<UITargetedPreview>>;
    }
);
