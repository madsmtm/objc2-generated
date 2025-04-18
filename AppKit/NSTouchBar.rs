//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstouchbarcustomizationidentifier?language=objc)
pub type NSTouchBarCustomizationIdentifier = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstouchbar?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTouchBar;
);

extern_conformance!(
    unsafe impl NSCoding for NSTouchBar {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTouchBar {}
);

impl NSTouchBar {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(customizationIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn customizationIdentifier(
            &self,
        ) -> Option<Retained<NSTouchBarCustomizationIdentifier>>;

        /// Setter for [`customizationIdentifier`][Self::customizationIdentifier].
        #[unsafe(method(setCustomizationIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCustomizationIdentifier(
            &self,
            customization_identifier: Option<&NSTouchBarCustomizationIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[unsafe(method(customizationAllowedItemIdentifiers))]
        #[unsafe(method_family = none)]
        pub unsafe fn customizationAllowedItemIdentifiers(
            &self,
        ) -> Retained<NSArray<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        /// Setter for [`customizationAllowedItemIdentifiers`][Self::customizationAllowedItemIdentifiers].
        #[unsafe(method(setCustomizationAllowedItemIdentifiers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCustomizationAllowedItemIdentifiers(
            &self,
            customization_allowed_item_identifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[unsafe(method(customizationRequiredItemIdentifiers))]
        #[unsafe(method_family = none)]
        pub unsafe fn customizationRequiredItemIdentifiers(
            &self,
        ) -> Retained<NSArray<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        /// Setter for [`customizationRequiredItemIdentifiers`][Self::customizationRequiredItemIdentifiers].
        #[unsafe(method(setCustomizationRequiredItemIdentifiers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCustomizationRequiredItemIdentifiers(
            &self,
            customization_required_item_identifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[unsafe(method(defaultItemIdentifiers))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultItemIdentifiers(&self) -> Retained<NSArray<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        /// Setter for [`defaultItemIdentifiers`][Self::defaultItemIdentifiers].
        #[unsafe(method(setDefaultItemIdentifiers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDefaultItemIdentifiers(
            &self,
            default_item_identifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[unsafe(method(itemIdentifiers))]
        #[unsafe(method_family = none)]
        pub unsafe fn itemIdentifiers(&self) -> Retained<NSArray<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        #[unsafe(method(principalItemIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn principalItemIdentifier(&self) -> Option<Retained<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        /// Setter for [`principalItemIdentifier`][Self::principalItemIdentifier].
        #[unsafe(method(setPrincipalItemIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPrincipalItemIdentifier(
            &self,
            principal_item_identifier: Option<&NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[unsafe(method(escapeKeyReplacementItemIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn escapeKeyReplacementItemIdentifier(
            &self,
        ) -> Option<Retained<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        /// Setter for [`escapeKeyReplacementItemIdentifier`][Self::escapeKeyReplacementItemIdentifier].
        #[unsafe(method(setEscapeKeyReplacementItemIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEscapeKeyReplacementItemIdentifier(
            &self,
            escape_key_replacement_item_identifier: Option<&NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[unsafe(method(templateItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn templateItems(&self) -> Retained<NSSet<NSTouchBarItem>>;

        #[cfg(feature = "NSTouchBarItem")]
        /// Setter for [`templateItems`][Self::templateItems].
        #[unsafe(method(setTemplateItems:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTemplateItems(&self, template_items: &NSSet<NSTouchBarItem>);

        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSTouchBarDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSTouchBarDelegate>>);

        #[cfg(feature = "NSTouchBarItem")]
        #[unsafe(method(itemForIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn itemForIdentifier(
            &self,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Option<Retained<NSTouchBarItem>>;

        #[unsafe(method(isVisible))]
        #[unsafe(method_family = none)]
        pub unsafe fn isVisible(&self) -> bool;

        #[unsafe(method(isAutomaticCustomizeTouchBarMenuItemEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled(mtm: MainThreadMarker) -> bool;

        /// Setter for [`isAutomaticCustomizeTouchBarMenuItemEnabled`][Self::isAutomaticCustomizeTouchBarMenuItemEnabled].
        #[unsafe(method(setAutomaticCustomizeTouchBarMenuItemEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            automatic_customize_touch_bar_menu_item_enabled: bool,
            mtm: MainThreadMarker,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTouchBar {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstouchbardelegate?language=objc)
    pub unsafe trait NSTouchBarDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "NSTouchBarItem")]
        #[optional]
        #[unsafe(method(touchBar:makeItemForIdentifier:))]
        #[unsafe(method_family = none)]
        unsafe fn touchBar_makeItemForIdentifier(
            &self,
            touch_bar: &NSTouchBar,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Option<Retained<NSTouchBarItem>>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstouchbarprovider?language=objc)
    pub unsafe trait NSTouchBarProvider: NSObjectProtocol + MainThreadOnly {
        #[unsafe(method(touchBar))]
        #[unsafe(method_family = none)]
        unsafe fn touchBar(&self) -> Option<Retained<NSTouchBar>>;
    }
);

/// NSTouchBarProvider.
#[cfg(feature = "NSResponder")]
impl NSResponder {
    extern_methods!(
        #[unsafe(method(touchBar))]
        #[unsafe(method_family = none)]
        pub unsafe fn touchBar(&self) -> Option<Retained<NSTouchBar>>;

        /// Setter for [`touchBar`][Self::touchBar].
        #[unsafe(method(setTouchBar:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTouchBar(&self, touch_bar: Option<&NSTouchBar>);

        #[unsafe(method(makeTouchBar))]
        #[unsafe(method_family = none)]
        pub unsafe fn makeTouchBar(&self) -> Option<Retained<NSTouchBar>>;
    );
}

#[cfg(feature = "NSResponder")]
extern_conformance!(
    unsafe impl NSTouchBarProvider for NSResponder {}
);

/// NSTouchBarCustomization.
#[cfg(all(feature = "NSApplication", feature = "NSResponder"))]
impl NSApplication {
    extern_methods!(
        /// Whether or not a menu item to customize the NSTouchBar can be automatically added to the main menu. It will only actually be added when Touch Bar hardware or simulator is present. Defaults to NO. Setting this property to YES is the recommended way to add the customization menu item. But if non-standard placement of the menu item is needed, creating a menu item with an action of `toggleTouchBarCustomizationPalette:` can be used instead.
        #[unsafe(method(isAutomaticCustomizeTouchBarMenuItemEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled(&self) -> bool;

        /// Setter for [`isAutomaticCustomizeTouchBarMenuItemEnabled`][Self::isAutomaticCustomizeTouchBarMenuItemEnabled].
        #[unsafe(method(setAutomaticCustomizeTouchBarMenuItemEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            &self,
            automatic_customize_touch_bar_menu_item_enabled: bool,
        );

        /// Show or dismiss the customization palette for the currently displayed NSTouchBars. NSApplication validates this selector against whether the current NSTouchBars are customizable and, if configured on a menu item, will standardize and localize the title. If the current system does not have Touch Bar support, the menu item will be automatically hidden.
        #[unsafe(method(toggleTouchBarCustomizationPalette:))]
        #[unsafe(method_family = none)]
        pub unsafe fn toggleTouchBarCustomizationPalette(&self, sender: Option<&AnyObject>);
    );
}
