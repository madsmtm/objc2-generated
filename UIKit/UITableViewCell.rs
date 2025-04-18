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

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcellstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITableViewCellStyle(pub NSInteger);
impl UITableViewCellStyle {
    #[doc(alias = "UITableViewCellStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "UITableViewCellStyleValue1")]
    pub const Value1: Self = Self(1);
    #[doc(alias = "UITableViewCellStyleValue2")]
    pub const Value2: Self = Self(2);
    #[doc(alias = "UITableViewCellStyleSubtitle")]
    pub const Subtitle: Self = Self(3);
}

unsafe impl Encode for UITableViewCellStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITableViewCellStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcellseparatorstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITableViewCellSeparatorStyle(pub NSInteger);
impl UITableViewCellSeparatorStyle {
    #[doc(alias = "UITableViewCellSeparatorStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UITableViewCellSeparatorStyleSingleLine")]
    pub const SingleLine: Self = Self(1);
    #[doc(alias = "UITableViewCellSeparatorStyleSingleLineEtched")]
    #[deprecated = "Use UITableViewCellSeparatorStyleSingleLine for a single line separator."]
    pub const SingleLineEtched: Self = Self(2);
}

unsafe impl Encode for UITableViewCellSeparatorStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITableViewCellSeparatorStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcellselectionstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITableViewCellSelectionStyle(pub NSInteger);
impl UITableViewCellSelectionStyle {
    #[doc(alias = "UITableViewCellSelectionStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UITableViewCellSelectionStyleBlue")]
    pub const Blue: Self = Self(1);
    #[doc(alias = "UITableViewCellSelectionStyleGray")]
    pub const Gray: Self = Self(2);
    #[doc(alias = "UITableViewCellSelectionStyleDefault")]
    pub const Default: Self = Self(3);
}

unsafe impl Encode for UITableViewCellSelectionStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITableViewCellSelectionStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcellfocusstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITableViewCellFocusStyle(pub NSInteger);
impl UITableViewCellFocusStyle {
    #[doc(alias = "UITableViewCellFocusStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "UITableViewCellFocusStyleCustom")]
    pub const Custom: Self = Self(1);
}

unsafe impl Encode for UITableViewCellFocusStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITableViewCellFocusStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcelleditingstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITableViewCellEditingStyle(pub NSInteger);
impl UITableViewCellEditingStyle {
    #[doc(alias = "UITableViewCellEditingStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UITableViewCellEditingStyleDelete")]
    pub const Delete: Self = Self(1);
    #[doc(alias = "UITableViewCellEditingStyleInsert")]
    pub const Insert: Self = Self(2);
}

unsafe impl Encode for UITableViewCellEditingStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITableViewCellEditingStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcellaccessorytype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITableViewCellAccessoryType(pub NSInteger);
impl UITableViewCellAccessoryType {
    #[doc(alias = "UITableViewCellAccessoryNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UITableViewCellAccessoryDisclosureIndicator")]
    pub const DisclosureIndicator: Self = Self(1);
    #[doc(alias = "UITableViewCellAccessoryDetailDisclosureButton")]
    pub const DetailDisclosureButton: Self = Self(2);
    #[doc(alias = "UITableViewCellAccessoryCheckmark")]
    pub const Checkmark: Self = Self(3);
    #[doc(alias = "UITableViewCellAccessoryDetailButton")]
    pub const DetailButton: Self = Self(4);
}

unsafe impl Encode for UITableViewCellAccessoryType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITableViewCellAccessoryType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcellstatemask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITableViewCellStateMask(pub NSUInteger);
bitflags::bitflags! {
    impl UITableViewCellStateMask: NSUInteger {
        #[doc(alias = "UITableViewCellStateDefaultMask")]
        const DefaultMask = 0;
        #[doc(alias = "UITableViewCellStateShowingEditControlMask")]
        const ShowingEditControlMask = 1<<0;
        #[doc(alias = "UITableViewCellStateShowingDeleteConfirmationMask")]
        const ShowingDeleteConfirmationMask = 1<<1;
    }
}

unsafe impl Encode for UITableViewCellStateMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UITableViewCellStateMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcelldragstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITableViewCellDragState(pub NSInteger);
impl UITableViewCellDragState {
    #[doc(alias = "UITableViewCellDragStateNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UITableViewCellDragStateLifting")]
    pub const Lifting: Self = Self(1);
    #[doc(alias = "UITableViewCellDragStateDragging")]
    pub const Dragging: Self = Self(2);
}

unsafe impl Encode for UITableViewCellDragState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITableViewCellDragState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcellconfigurationupdatehandler?language=objc)
#[cfg(all(
    feature = "UICellConfigurationState",
    feature = "UIResponder",
    feature = "UIView",
    feature = "UIViewConfigurationState",
    feature = "block2"
))]
pub type UITableViewCellConfigurationUpdateHandler =
    *mut block2::DynBlock<dyn Fn(NonNull<UITableViewCell>, NonNull<UICellConfigurationState>)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewcell?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UITableViewCell;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
extern_conformance!(
    unsafe impl CALayerDelegate for UITableViewCell {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSCoding for UITableViewCell {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for UITableViewCell {}
);

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIAppearance for UITableViewCell {}
);

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIAppearanceContainer for UITableViewCell {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UICoordinateSpace for UITableViewCell {}
);

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIDynamicItem for UITableViewCell {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusEnvironment for UITableViewCell {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusItem for UITableViewCell {}
);

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIFocusItemContainer for UITableViewCell {}
);

#[cfg(all(
    feature = "UIGestureRecognizer",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIGestureRecognizerDelegate for UITableViewCell {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIResponderStandardEditActions for UITableViewCell {}
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITraitEnvironment for UITableViewCell {}
);

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UITableViewCell {
    extern_methods!(
        #[unsafe(method(initWithStyle:reuseIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithStyle_reuseIdentifier(
            this: Allocated<Self>,
            style: UITableViewCellStyle,
            reuse_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "UICellConfigurationState",
            feature = "UIViewConfigurationState"
        ))]
        /// Returns the current configuration state for the cell.
        /// To add your own custom state(s), override the getter and call super to obtain an instance with the
        /// system properties set, then set your own custom states as desired.
        #[unsafe(method(configurationState))]
        #[unsafe(method_family = none)]
        pub unsafe fn configurationState(&self) -> Retained<UICellConfigurationState>;

        /// Requests the cell update its configuration for its current state. This method is called automatically
        /// when the cell's `configurationState` may have changed, as well as in other circumstances where an
        /// update may be required. Multiple requests may be coalesced into a single update at the appropriate time.
        #[unsafe(method(setNeedsUpdateConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNeedsUpdateConfiguration(&self);

        #[cfg(all(
            feature = "UICellConfigurationState",
            feature = "UIViewConfigurationState"
        ))]
        /// Subclasses should override this method and update the cell's configuration using the state provided.
        /// This method should not be called directly, use `setNeedsUpdateConfiguration` to request an update.
        #[unsafe(method(updateConfigurationUsingState:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateConfigurationUsingState(&self, state: &UICellConfigurationState);

        #[cfg(all(
            feature = "UICellConfigurationState",
            feature = "UIViewConfigurationState",
            feature = "block2"
        ))]
        /// Optional block-based alternative to overriding `-updateConfigurationUsingState:` in a subclass. This handler
        /// is called after `-updateConfigurationUsingState:`. Setting a new handler triggers `setNeedsUpdateConfiguration`.
        #[unsafe(method(configurationUpdateHandler))]
        #[unsafe(method_family = none)]
        pub unsafe fn configurationUpdateHandler(
            &self,
        ) -> UITableViewCellConfigurationUpdateHandler;

        #[cfg(all(
            feature = "UICellConfigurationState",
            feature = "UIViewConfigurationState",
            feature = "block2"
        ))]
        /// Setter for [`configurationUpdateHandler`][Self::configurationUpdateHandler].
        #[unsafe(method(setConfigurationUpdateHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setConfigurationUpdateHandler(
            &self,
            configuration_update_handler: UITableViewCellConfigurationUpdateHandler,
        );

        #[cfg(feature = "UIListContentConfiguration")]
        /// Returns a default list content configuration for the cell's style.
        #[unsafe(method(defaultContentConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultContentConfiguration(&self) -> Retained<UIListContentConfiguration>;

        #[cfg(feature = "UIContentConfiguration")]
        /// Setting a content configuration replaces the existing contentView of the cell with a new content view instance from the configuration,
        /// or directly applies the configuration to the existing content view if the configuration is compatible with the existing content view type.
        /// The default value is nil. After a configuration has been set, setting this property to nil will replace the current content view with a new content view.
        #[unsafe(method(contentConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentConfiguration(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIContentConfiguration>>>;

        #[cfg(feature = "UIContentConfiguration")]
        /// Setter for [`contentConfiguration`][Self::contentConfiguration].
        #[unsafe(method(setContentConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentConfiguration(
            &self,
            content_configuration: Option<&ProtocolObject<dyn UIContentConfiguration>>,
        );

        /// When YES, the cell will automatically call -updatedConfigurationForState: on its `contentConfiguration` when the cell's
        /// configuration state changes, and apply the updated configuration back to the cell. The default value is YES.
        #[unsafe(method(automaticallyUpdatesContentConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn automaticallyUpdatesContentConfiguration(&self) -> bool;

        /// Setter for [`automaticallyUpdatesContentConfiguration`][Self::automaticallyUpdatesContentConfiguration].
        #[unsafe(method(setAutomaticallyUpdatesContentConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutomaticallyUpdatesContentConfiguration(
            &self,
            automatically_updates_content_configuration: bool,
        );

        #[unsafe(method(contentView))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentView(&self) -> Retained<UIView>;

        #[cfg(feature = "UIImageView")]
        #[deprecated = "Use UIListContentConfiguration instead, this property will be deprecated in a future release."]
        #[unsafe(method(imageView))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageView(&self) -> Option<Retained<UIImageView>>;

        #[cfg(feature = "UILabel")]
        #[deprecated = "Use UIListContentConfiguration instead, this property will be deprecated in a future release."]
        #[unsafe(method(textLabel))]
        #[unsafe(method_family = none)]
        pub unsafe fn textLabel(&self) -> Option<Retained<UILabel>>;

        #[cfg(feature = "UILabel")]
        #[deprecated = "Use UIListContentConfiguration instead, this property will be deprecated in a future release."]
        #[unsafe(method(detailTextLabel))]
        #[unsafe(method_family = none)]
        pub unsafe fn detailTextLabel(&self) -> Option<Retained<UILabel>>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// Returns a default background configuration for the cell's style.
        /// This background configuration represents the default appearance that the cell will use.
        #[unsafe(method(defaultBackgroundConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultBackgroundConfiguration(&self) -> Retained<UIBackgroundConfiguration>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// Setting a background configuration supersedes the cell's backgroundView, selectedBackgroundView, and multipleSelectionBackgroundView. The default value is nil.
        #[unsafe(method(backgroundConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn backgroundConfiguration(&self)
            -> Option<Retained<UIBackgroundConfiguration>>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// Setter for [`backgroundConfiguration`][Self::backgroundConfiguration].
        #[unsafe(method(setBackgroundConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBackgroundConfiguration(
            &self,
            background_configuration: Option<&UIBackgroundConfiguration>,
        );

        /// When YES, the cell will automatically call -updatedConfigurationForState: on its `backgroundConfiguration` when the cell's
        /// configuration state changes, and apply the updated configuration back to the cell. The default value is YES.
        #[unsafe(method(automaticallyUpdatesBackgroundConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn automaticallyUpdatesBackgroundConfiguration(&self) -> bool;

        /// Setter for [`automaticallyUpdatesBackgroundConfiguration`][Self::automaticallyUpdatesBackgroundConfiguration].
        #[unsafe(method(setAutomaticallyUpdatesBackgroundConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutomaticallyUpdatesBackgroundConfiguration(
            &self,
            automatically_updates_background_configuration: bool,
        );

        #[unsafe(method(backgroundView))]
        #[unsafe(method_family = none)]
        pub unsafe fn backgroundView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`backgroundView`][Self::backgroundView].
        #[unsafe(method(setBackgroundView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBackgroundView(&self, background_view: Option<&UIView>);

        #[unsafe(method(selectedBackgroundView))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedBackgroundView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`selectedBackgroundView`][Self::selectedBackgroundView].
        #[unsafe(method(setSelectedBackgroundView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedBackgroundView(&self, selected_background_view: Option<&UIView>);

        #[unsafe(method(multipleSelectionBackgroundView))]
        #[unsafe(method_family = none)]
        pub unsafe fn multipleSelectionBackgroundView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`multipleSelectionBackgroundView`][Self::multipleSelectionBackgroundView].
        #[unsafe(method(setMultipleSelectionBackgroundView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMultipleSelectionBackgroundView(
            &self,
            multiple_selection_background_view: Option<&UIView>,
        );

        #[unsafe(method(reuseIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn reuseIdentifier(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(prepareForReuse))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareForReuse(&self);

        #[unsafe(method(selectionStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectionStyle(&self) -> UITableViewCellSelectionStyle;

        /// Setter for [`selectionStyle`][Self::selectionStyle].
        #[unsafe(method(setSelectionStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectionStyle(&self, selection_style: UITableViewCellSelectionStyle);

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

        #[unsafe(method(setSelected:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelected_animated(&self, selected: bool, animated: bool);

        #[unsafe(method(setHighlighted:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHighlighted_animated(&self, highlighted: bool, animated: bool);

        #[unsafe(method(editingStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn editingStyle(&self) -> UITableViewCellEditingStyle;

        #[unsafe(method(showsReorderControl))]
        #[unsafe(method_family = none)]
        pub unsafe fn showsReorderControl(&self) -> bool;

        /// Setter for [`showsReorderControl`][Self::showsReorderControl].
        #[unsafe(method(setShowsReorderControl:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShowsReorderControl(&self, shows_reorder_control: bool);

        #[unsafe(method(shouldIndentWhileEditing))]
        #[unsafe(method_family = none)]
        pub unsafe fn shouldIndentWhileEditing(&self) -> bool;

        /// Setter for [`shouldIndentWhileEditing`][Self::shouldIndentWhileEditing].
        #[unsafe(method(setShouldIndentWhileEditing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShouldIndentWhileEditing(&self, should_indent_while_editing: bool);

        #[unsafe(method(accessoryType))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessoryType(&self) -> UITableViewCellAccessoryType;

        /// Setter for [`accessoryType`][Self::accessoryType].
        #[unsafe(method(setAccessoryType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAccessoryType(&self, accessory_type: UITableViewCellAccessoryType);

        #[unsafe(method(accessoryView))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessoryView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`accessoryView`][Self::accessoryView].
        #[unsafe(method(setAccessoryView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&UIView>);

        #[unsafe(method(editingAccessoryType))]
        #[unsafe(method_family = none)]
        pub unsafe fn editingAccessoryType(&self) -> UITableViewCellAccessoryType;

        /// Setter for [`editingAccessoryType`][Self::editingAccessoryType].
        #[unsafe(method(setEditingAccessoryType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEditingAccessoryType(
            &self,
            editing_accessory_type: UITableViewCellAccessoryType,
        );

        #[unsafe(method(editingAccessoryView))]
        #[unsafe(method_family = none)]
        pub unsafe fn editingAccessoryView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`editingAccessoryView`][Self::editingAccessoryView].
        #[unsafe(method(setEditingAccessoryView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEditingAccessoryView(&self, editing_accessory_view: Option<&UIView>);

        #[unsafe(method(indentationLevel))]
        #[unsafe(method_family = none)]
        pub unsafe fn indentationLevel(&self) -> NSInteger;

        /// Setter for [`indentationLevel`][Self::indentationLevel].
        #[unsafe(method(setIndentationLevel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIndentationLevel(&self, indentation_level: NSInteger);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(indentationWidth))]
        #[unsafe(method_family = none)]
        pub unsafe fn indentationWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`indentationWidth`][Self::indentationWidth].
        #[unsafe(method(setIndentationWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIndentationWidth(&self, indentation_width: CGFloat);

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[unsafe(method(separatorInset))]
        #[unsafe(method_family = none)]
        pub unsafe fn separatorInset(&self) -> UIEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`separatorInset`][Self::separatorInset].
        #[unsafe(method(setSeparatorInset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSeparatorInset(&self, separator_inset: UIEdgeInsets);

        #[unsafe(method(isEditing))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEditing(&self) -> bool;

        /// Setter for [`isEditing`][Self::isEditing].
        #[unsafe(method(setEditing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEditing(&self, editing: bool);

        #[unsafe(method(setEditing:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEditing_animated(&self, editing: bool, animated: bool);

        #[unsafe(method(showingDeleteConfirmation))]
        #[unsafe(method_family = none)]
        pub unsafe fn showingDeleteConfirmation(&self) -> bool;

        #[unsafe(method(focusStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn focusStyle(&self) -> UITableViewCellFocusStyle;

        /// Setter for [`focusStyle`][Self::focusStyle].
        #[unsafe(method(setFocusStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFocusStyle(&self, focus_style: UITableViewCellFocusStyle);

        #[unsafe(method(willTransitionToState:))]
        #[unsafe(method_family = none)]
        pub unsafe fn willTransitionToState(&self, state: UITableViewCellStateMask);

        #[unsafe(method(didTransitionToState:))]
        #[unsafe(method_family = none)]
        pub unsafe fn didTransitionToState(&self, state: UITableViewCellStateMask);

        #[unsafe(method(dragStateDidChange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dragStateDidChange(&self, drag_state: UITableViewCellDragState);

        #[unsafe(method(userInteractionEnabledWhileDragging))]
        #[unsafe(method_family = none)]
        pub unsafe fn userInteractionEnabledWhileDragging(&self) -> bool;

        /// Setter for [`userInteractionEnabledWhileDragging`][Self::userInteractionEnabledWhileDragging].
        #[unsafe(method(setUserInteractionEnabledWhileDragging:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUserInteractionEnabledWhileDragging(
            &self,
            user_interaction_enabled_while_dragging: bool,
        );
    );
}

/// Methods declared on superclass `UIView`.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UITableViewCell {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UITableViewCell {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

/// UIDeprecated.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UITableViewCell {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated]
        #[unsafe(method(initWithFrame:reuseIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame_reuseIdentifier(
            this: Allocated<Self>,
            frame: CGRect,
            reuse_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[deprecated]
        #[unsafe(method(text))]
        #[unsafe(method_family = none)]
        pub unsafe fn text(&self) -> Option<Retained<NSString>>;

        /// Setter for [`text`][Self::text].
        #[deprecated]
        #[unsafe(method(setText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[cfg(feature = "UIFont")]
        #[deprecated]
        #[unsafe(method(font))]
        #[unsafe(method_family = none)]
        pub unsafe fn font(&self) -> Option<Retained<UIFont>>;

        #[cfg(feature = "UIFont")]
        /// Setter for [`font`][Self::font].
        #[deprecated]
        #[unsafe(method(setFont:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFont(&self, font: Option<&UIFont>);

        #[cfg(feature = "NSText")]
        #[deprecated]
        #[unsafe(method(textAlignment))]
        #[unsafe(method_family = none)]
        pub unsafe fn textAlignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        /// Setter for [`textAlignment`][Self::textAlignment].
        #[deprecated]
        #[unsafe(method(setTextAlignment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextAlignment(&self, text_alignment: NSTextAlignment);

        #[cfg(feature = "NSParagraphStyle")]
        #[deprecated]
        #[unsafe(method(lineBreakMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        /// Setter for [`lineBreakMode`][Self::lineBreakMode].
        #[deprecated]
        #[unsafe(method(setLineBreakMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[cfg(feature = "UIColor")]
        #[deprecated]
        #[unsafe(method(textColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn textColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`textColor`][Self::textColor].
        #[deprecated]
        #[unsafe(method(setTextColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextColor(&self, text_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[deprecated]
        #[unsafe(method(selectedTextColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedTextColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`selectedTextColor`][Self::selectedTextColor].
        #[deprecated]
        #[unsafe(method(setSelectedTextColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedTextColor(&self, selected_text_color: Option<&UIColor>);

        #[cfg(feature = "UIImage")]
        #[deprecated]
        #[unsafe(method(image))]
        #[unsafe(method_family = none)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`image`][Self::image].
        #[deprecated]
        #[unsafe(method(setImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[deprecated]
        #[unsafe(method(selectedImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`selectedImage`][Self::selectedImage].
        #[deprecated]
        #[unsafe(method(setSelectedImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedImage(&self, selected_image: Option<&UIImage>);

        #[deprecated]
        #[unsafe(method(hidesAccessoryWhenEditing))]
        #[unsafe(method_family = none)]
        pub unsafe fn hidesAccessoryWhenEditing(&self) -> bool;

        /// Setter for [`hidesAccessoryWhenEditing`][Self::hidesAccessoryWhenEditing].
        #[deprecated]
        #[unsafe(method(setHidesAccessoryWhenEditing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHidesAccessoryWhenEditing(&self, hides_accessory_when_editing: bool);

        #[deprecated]
        #[unsafe(method(target))]
        #[unsafe(method_family = none)]
        pub unsafe fn target(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`target`][Self::target].
        #[deprecated]
        #[unsafe(method(setTarget:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[deprecated]
        #[unsafe(method(editAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn editAction(&self) -> Option<Sel>;

        /// Setter for [`editAction`][Self::editAction].
        #[deprecated]
        #[unsafe(method(setEditAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEditAction(&self, edit_action: Option<Sel>);

        #[deprecated]
        #[unsafe(method(accessoryAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessoryAction(&self) -> Option<Sel>;

        /// Setter for [`accessoryAction`][Self::accessoryAction].
        #[deprecated]
        #[unsafe(method(setAccessoryAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAccessoryAction(&self, accessory_action: Option<Sel>);
    );
}
