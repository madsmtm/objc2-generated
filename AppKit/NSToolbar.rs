//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbaridentifier?language=objc)
pub type NSToolbarIdentifier = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbaritemidentifier?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSToolbarItemIdentifier = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbaruserinfokey?language=objc)
// NS_TYPED_ENUM
pub type NSToolbarUserInfoKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbaritemkey?language=objc)
    pub static NSToolbarItemKey: &'static NSToolbarUserInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbarnewindexkey?language=objc)
    pub static NSToolbarNewIndexKey: &'static NSToolbarUserInfoKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbardisplaymode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSToolbarDisplayMode(pub NSUInteger);
impl NSToolbarDisplayMode {
    #[doc(alias = "NSToolbarDisplayModeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSToolbarDisplayModeIconAndLabel")]
    pub const IconAndLabel: Self = Self(1);
    #[doc(alias = "NSToolbarDisplayModeIconOnly")]
    pub const IconOnly: Self = Self(2);
    #[doc(alias = "NSToolbarDisplayModeLabelOnly")]
    pub const LabelOnly: Self = Self(3);
}

unsafe impl Encode for NSToolbarDisplayMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSToolbarDisplayMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbarsizemode?language=objc)
// NS_ENUM
#[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSToolbarSizeMode(pub NSUInteger);
impl NSToolbarSizeMode {
    #[doc(alias = "NSToolbarSizeModeDefault")]
    #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSToolbarSizeModeRegular")]
    #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
    pub const Regular: Self = Self(1);
    #[doc(alias = "NSToolbarSizeModeSmall")]
    #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
    pub const Small: Self = Self(2);
}

unsafe impl Encode for NSToolbarSizeMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSToolbarSizeMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbar?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSToolbar;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSToolbar {}
);

impl NSToolbar {
    extern_methods!(
        /// The identifier is used to form the toolbar's autosave name.
        /// Toolbars with the same identifier are implicitly synchronized so that they maintain the same state.
        #[unsafe(method(initWithIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSToolbarIdentifier,
        ) -> Retained<Self>;

        /// Calls through to -initWithIdentifier: with an empty string identifier.
        /// Customizable toolbars should use `-initWithIdentifier:` with a unique identifier instead.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Inserts an item with the specified identifier in the receiving toolbar at the specified index.
        ///
        /// Any change made will be propagated immediately to all other toolbars with the same identifier.
        #[unsafe(method(insertItemWithItemIdentifier:atIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn insertItemWithItemIdentifier_atIndex(
            &self,
            item_identifier: &NSToolbarItemIdentifier,
            index: NSInteger,
        );

        /// Removes the item at the specified index in the receiving toolbar.
        ///
        /// Any change made will be propagated immediately to all other toolbars with the same identifier.
        #[unsafe(method(removeItemAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        /// Removes the item with matching `itemIdentifier` in the receiving toolbar. If multiple items share the same identifier (as is the case with space items) all matching items will be removed. To remove only a single space item, use `-removeItemAtIndex:` instead.
        ///
        /// Any change made will be propagated immediately to all other toolbars with the same identifier.
        #[unsafe(method(removeItemWithItemIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeItemWithItemIdentifier(
            &self,
            item_identifier: &NSToolbarItemIdentifier,
        );

        /// Customizable toolbars must have a delegate, and must implement the required `NSToolbarDelegate` methods.
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSToolbarDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSToolbarDelegate>>);

        /// Toggles the visibility of the toolbar.
        /// This property may be modified by the user in toolbars with `allowsUserCustomization` enabled.
        /// This property is key value observable on macOS 14.0 and higher.
        #[unsafe(method(isVisible))]
        #[unsafe(method_family = none)]
        pub unsafe fn isVisible(&self) -> bool;

        /// Setter for [`isVisible`][Self::isVisible].
        #[unsafe(method(setVisible:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVisible(&self, visible: bool);

        /// Customizable toolbars (those with delegates) can show a palette which allows users to populate the toolbar with individual items or to reset the toolbar to some default set of items.
        /// The items and item sets in the palette are specified by the delegate (`-toolbarAllowedItemIdentifiers:` and `-toolbarDefaultItemIdentifiers:`).
        /// When the user is done configuring, they will dismiss the palette.
        #[unsafe(method(runCustomizationPalette:))]
        #[unsafe(method_family = none)]
        pub unsafe fn runCustomizationPalette(&self, sender: Option<&AnyObject>);

        /// Whether or not the customization palette is currently running.
        /// On macOS 15.0 and above this property is key value observable.
        #[unsafe(method(customizationPaletteIsRunning))]
        #[unsafe(method_family = none)]
        pub unsafe fn customizationPaletteIsRunning(&self) -> bool;

        /// The current display mode of items in the toolbar.
        /// In toolbars with `allowsDisplayModeCustomization` enabled this is a user modifiable property.
        /// This property is key value observable.
        #[unsafe(method(displayMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn displayMode(&self) -> NSToolbarDisplayMode;

        /// Setter for [`displayMode`][Self::displayMode].
        #[unsafe(method(setDisplayMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDisplayMode(&self, display_mode: NSToolbarDisplayMode);

        /// Sets the toolbar's selected item by identifier.
        /// Use this to force an item identifier to be selected.
        /// Toolbar manages selection of image items automatically.
        /// This method can be used to select identifiers of custom view items, or to force a selection change.
        /// See `-toolbarSelectableItemIdentifiers:` delegate method for more details.
        /// This property is key value observable.
        #[unsafe(method(selectedItemIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedItemIdentifier(&self) -> Option<Retained<NSToolbarItemIdentifier>>;

        /// Setter for [`selectedItemIdentifier`][Self::selectedItemIdentifier].
        #[unsafe(method(setSelectedItemIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedItemIdentifier(
            &self,
            selected_item_identifier: Option<&NSToolbarItemIdentifier>,
        );

        /// This flag controls whether or not users can configure the toolbar by dragging items around, and whether or not the customization palette can be used.
        /// The default value is NO, but can be changed at any time.
        /// For instance, a developer may not want users to be able to edit the toolbar while some event is being processed.
        #[unsafe(method(allowsUserCustomization))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowsUserCustomization(&self) -> bool;

        /// Setter for [`allowsUserCustomization`][Self::allowsUserCustomization].
        #[unsafe(method(setAllowsUserCustomization:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowsUserCustomization(&self, allows_user_customization: bool);

        /// Whether or not the user is allowed to change display modes at run time.
        /// This functionality is independent of customizing the order of the items themselves.
        /// Only disable when the functionality or legibility of your toolbar could not be improved by another display mode.
        /// The user's selection will be persisted using the toolbar's `identifier` when `autosavesConfiguration` is enabled.
        /// The default is YES for apps linked on macOS 15.0 and above.
        #[unsafe(method(allowsDisplayModeCustomization))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowsDisplayModeCustomization(&self) -> bool;

        /// Setter for [`allowsDisplayModeCustomization`][Self::allowsDisplayModeCustomization].
        #[unsafe(method(setAllowsDisplayModeCustomization:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowsDisplayModeCustomization(
            &self,
            allows_display_mode_customization: bool,
        );

        /// All toolbars with the same name will share the same display attributes, and item order.
        /// If a toolbar autosaves its configuration, the item identifier will be used as the autosave name.
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSToolbarIdentifier>;

        #[cfg(feature = "NSToolbarItem")]
        /// Allows you to access all current items in the toolbar.
        #[unsafe(method(items))]
        #[unsafe(method_family = none)]
        pub unsafe fn items(&self) -> Retained<NSArray<NSToolbarItem>>;

        #[cfg(feature = "NSToolbarItem")]
        /// Allows you to access the current visible items (non clipped).
        #[unsafe(method(visibleItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn visibleItems(&self) -> Option<Retained<NSArray<NSToolbarItem>>>;

        /// An array of itemIdentifiers that represent the current items in the toolbar.
        /// Setting this property will set the current items in the toolbar by diffing against items that already exist.
        /// Use this with great caution if `allowsUserCustomization` is enabled as it will override any customizations the user has made.
        /// This property is key value observable.
        #[unsafe(method(itemIdentifiers))]
        #[unsafe(method_family = none)]
        pub unsafe fn itemIdentifiers(&self) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        /// Setter for [`itemIdentifiers`][Self::itemIdentifiers].
        #[unsafe(method(setItemIdentifiers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setItemIdentifiers(
            &self,
            item_identifiers: &NSArray<NSToolbarItemIdentifier>,
        );

        /// Items with centered identifiers will be centered together in the Toolbar relative to the window assuming space allows.
        /// The order of items is initially defined by the default set of identifiers, but may be customized by the user.
        /// Centered items may not be moved outside of the center set of items by the user.
        /// This property is archived.
        #[unsafe(method(centeredItemIdentifiers))]
        #[unsafe(method_family = none)]
        pub unsafe fn centeredItemIdentifiers(&self) -> Retained<NSSet<NSToolbarItemIdentifier>>;

        /// Setter for [`centeredItemIdentifiers`][Self::centeredItemIdentifiers].
        #[unsafe(method(setCenteredItemIdentifiers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCenteredItemIdentifiers(
            &self,
            centered_item_identifiers: &NSSet<NSToolbarItemIdentifier>,
        );

        /// If `autosavesConfiguration` is YES, the toolbar will automatically write changes the user makes to user defaults.
        /// Customizable toolbars will want to set this flag to YES.
        /// Setting this to NO means changes in configuration are not written automatically, however you can use the `configurationDictionary` method to do it yourself.
        /// Default is NO.
        #[unsafe(method(autosavesConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn autosavesConfiguration(&self) -> bool;

        /// Setter for [`autosavesConfiguration`][Self::autosavesConfiguration].
        #[unsafe(method(setAutosavesConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutosavesConfiguration(&self, autosaves_configuration: bool);

        /// Typically you should not invoke this method.
        /// This method is called on window updates with the purpose of validating each of the visible items.
        /// The toolbar will iterate through the list of visible items, sending each a `-validate` message.
        /// If this method is invoked directly, all visible items including those with `autovalidates` disabled will get a `-validate` message.
        #[unsafe(method(validateVisibleItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn validateVisibleItems(&self);

        /// When YES, the receiver can dynamically create toolbar items for Action extensions in the toolbar configuration panel.
        /// To be included, an extension needs to declare NSExtensionServiceAllowsToolbarItem=YES in its Info.plist.
        /// The default value is NO.
        #[unsafe(method(allowsExtensionItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowsExtensionItems(&self) -> bool;

        /// Setter for [`allowsExtensionItems`][Self::allowsExtensionItems].
        #[unsafe(method(setAllowsExtensionItems:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowsExtensionItems(&self, allows_extension_items: bool);
    );
}

/// Methods declared on superclass `NSObject`.
impl NSToolbar {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbardelegate?language=objc)
    pub unsafe trait NSToolbarDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "NSToolbarItem")]
        /// Given an item identifier, this method returns an item.
        /// Note that it is expected that each toolbar receives its own distinct copies and each time this method is called a new item must be returned.
        /// If the item has a custom view, that view should be in place when the item is returned.
        /// Finally, do not assume the returned item is going to be added as an active item in the toolbar.
        /// In fact, the toolbar may ask for items here in order to construct the customization palette.
        /// If `willBeInsertedIntoToolbar` is YES, the returned item will be inserted, and you can expect `toolbarWillAddItem:` is about to be posted.
        #[optional]
        #[unsafe(method(toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:))]
        #[unsafe(method_family = none)]
        unsafe fn toolbar_itemForItemIdentifier_willBeInsertedIntoToolbar(
            &self,
            toolbar: &NSToolbar,
            item_identifier: &NSToolbarItemIdentifier,
            flag: bool,
        ) -> Option<Retained<NSToolbarItem>>;

        /// Returns the ordered list of items to be shown in the toolbar by default.
        /// If during initialization, no overriding values are found in the user defaults, or if the user chooses to revert to the default items this set will be used.
        #[optional]
        #[unsafe(method(toolbarDefaultItemIdentifiers:))]
        #[unsafe(method_family = none)]
        unsafe fn toolbarDefaultItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        /// Returns the list of all allowed items by identifier.
        /// By default, the toolbar does not assume any items are allowed so every allowed item must be explicitly listed.
        /// The set of allowed items is used to construct the customization palette.
        /// The order of items does not necessarily guarantee the order of appearance in the palette.
        /// At minimum, you should return the default item list.
        #[optional]
        #[unsafe(method(toolbarAllowedItemIdentifiers:))]
        #[unsafe(method_family = none)]
        unsafe fn toolbarAllowedItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        /// Optional method.
        /// Those wishing to indicate item selection in a toolbar should implement this method to return a non-empty array of selectable item identifiers.
        /// If implemented, the toolbar will remember and display the selected item with a special highlight.
        /// A selected item is one whose item identifier matches the current selected item identifier.
        /// Clicking on an item whose identifier is selectable will automatically update the toolbar's `selectedItemIdentifier` when possible.
        /// See `selectedItemIdentifier` for more details.
        #[optional]
        #[unsafe(method(toolbarSelectableItemIdentifiers:))]
        #[unsafe(method_family = none)]
        unsafe fn toolbarSelectableItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        /// Items in this set cannot be dragged or removed by the user.
        #[optional]
        #[unsafe(method(toolbarImmovableItemIdentifiers:))]
        #[unsafe(method_family = none)]
        unsafe fn toolbarImmovableItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSSet<NSToolbarItemIdentifier>>;

        /// Whether or not an item can be moved to a specified position in the toolbar.
        /// If implemented, this method will be called during a user drag and does not necessarily indicate the final position of an item.
        /// An index of NSNotFound indicates the item would be removed from the toolbar.
        #[optional]
        #[unsafe(method(toolbar:itemIdentifier:canBeInsertedAtIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn toolbar_itemIdentifier_canBeInsertedAtIndex(
            &self,
            toolbar: &NSToolbar,
            item_identifier: &NSToolbarItemIdentifier,
            index: NSInteger,
        ) -> bool;

        /// Before a new item is added to the toolbar, this notification is posted.
        /// This is the best place to notice a new item is going into the toolbar.
        /// For instance, if you need to cache a reference to the toolbar item or need to set up some initial state, this is the best place to do it.
        /// The notification object is the toolbar to which the item is being added.
        /// The item being added and its new index can be found by referencing `NSToolbarItemKey` and `NSToolbarNewIndexKey` in the userInfo dictionary respectively.
        #[optional]
        #[unsafe(method(toolbarWillAddItem:))]
        #[unsafe(method_family = none)]
        unsafe fn toolbarWillAddItem(&self, notification: &NSNotification);

        /// After an item is removed from a toolbar the notification is sent.
        /// This allows the chance to tear down information related to the item that may have been cached.
        /// The notification object is the toolbar from which the item is being removed.
        /// The item being removed is found by referencing the `NSToolbarItemKey` in the userInfo.
        #[optional]
        #[unsafe(method(toolbarDidRemoveItem:))]
        #[unsafe(method_family = none)]
        unsafe fn toolbarDidRemoveItem(&self, notification: &NSNotification);
    }
);

extern "C" {
    /// Notifications
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbarwilladditemnotification?language=objc)
    pub static NSToolbarWillAddItemNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbardidremoveitemnotification?language=objc)
    pub static NSToolbarDidRemoveItemNotification: &'static NSNotificationName;
}

/// NSDeprecated.
impl NSToolbar {
    extern_methods!(
        #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
        #[unsafe(method(sizeMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn sizeMode(&self) -> NSToolbarSizeMode;

        /// Setter for [`sizeMode`][Self::sizeMode].
        #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
        #[unsafe(method(setSizeMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSizeMode(&self, size_mode: NSToolbarSizeMode);

        #[deprecated = "Use the centeredItemIdentifiers property instead"]
        #[unsafe(method(centeredItemIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn centeredItemIdentifier(&self) -> Option<Retained<NSToolbarItemIdentifier>>;

        /// Setter for [`centeredItemIdentifier`][Self::centeredItemIdentifier].
        #[deprecated = "Use the centeredItemIdentifiers property instead"]
        #[unsafe(method(setCenteredItemIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCenteredItemIdentifier(
            &self,
            centered_item_identifier: Option<&NSToolbarItemIdentifier>,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use NSTitlebarAccessoryViewController with NSWindow instead"]
        #[unsafe(method(fullScreenAccessoryView))]
        #[unsafe(method_family = none)]
        pub unsafe fn fullScreenAccessoryView(&self) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// Setter for [`fullScreenAccessoryView`][Self::fullScreenAccessoryView].
        #[deprecated = "Use NSTitlebarAccessoryViewController with NSWindow instead"]
        #[unsafe(method(setFullScreenAccessoryView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFullScreenAccessoryView(
            &self,
            full_screen_accessory_view: Option<&NSView>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "Use NSTitlebarAccessoryViewController and its fullScreenMinHeight property with NSWindow instead."]
        #[unsafe(method(fullScreenAccessoryViewMinHeight))]
        #[unsafe(method_family = none)]
        pub unsafe fn fullScreenAccessoryViewMinHeight(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`fullScreenAccessoryViewMinHeight`][Self::fullScreenAccessoryViewMinHeight].
        #[deprecated = "Use NSTitlebarAccessoryViewController and its fullScreenMinHeight property with NSWindow instead."]
        #[unsafe(method(setFullScreenAccessoryViewMinHeight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFullScreenAccessoryViewMinHeight(
            &self,
            full_screen_accessory_view_min_height: CGFloat,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "Use NSTitlebarAccessoryViewController with NSWindow instead. The max height of a titlebar accessory is implied by its view's height."]
        #[unsafe(method(fullScreenAccessoryViewMaxHeight))]
        #[unsafe(method_family = none)]
        pub unsafe fn fullScreenAccessoryViewMaxHeight(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`fullScreenAccessoryViewMaxHeight`][Self::fullScreenAccessoryViewMaxHeight].
        #[deprecated = "Use NSTitlebarAccessoryViewController with NSWindow instead. The max height of a titlebar accessory is implied by its view's height."]
        #[unsafe(method(setFullScreenAccessoryViewMaxHeight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFullScreenAccessoryViewMaxHeight(
            &self,
            full_screen_accessory_view_max_height: CGFloat,
        );

        #[deprecated = "No longer supported"]
        #[unsafe(method(showsBaselineSeparator))]
        #[unsafe(method_family = none)]
        pub unsafe fn showsBaselineSeparator(&self) -> bool;

        /// Setter for [`showsBaselineSeparator`][Self::showsBaselineSeparator].
        #[deprecated = "No longer supported"]
        #[unsafe(method(setShowsBaselineSeparator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShowsBaselineSeparator(&self, shows_baseline_separator: bool);

        #[deprecated = "Use -itemIdentifiers and -displayMode instead."]
        #[unsafe(method(configurationDictionary))]
        #[unsafe(method_family = none)]
        pub unsafe fn configurationDictionary(&self)
            -> Retained<NSDictionary<NSString, AnyObject>>;

        #[deprecated = "Use -setItemIdentifiers: and -setDisplayMode: instead."]
        #[unsafe(method(setConfigurationFromDictionary:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setConfigurationFromDictionary(
            &self,
            config_dict: &NSDictionary<NSString, AnyObject>,
        );
    );
}
