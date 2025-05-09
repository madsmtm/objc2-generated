//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssharingservicepickertoolbaritem?language=objc)
    #[unsafe(super(NSToolbarItem, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSToolbarItem")]
    pub struct NSSharingServicePickerToolbarItem;
);

#[cfg(feature = "NSToolbarItem")]
extern_conformance!(
    unsafe impl NSCopying for NSSharingServicePickerToolbarItem {}
);

#[cfg(feature = "NSToolbarItem")]
unsafe impl CopyingHelper for NSSharingServicePickerToolbarItem {
    type Result = Self;
}

#[cfg(feature = "NSToolbarItem")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSSharingServicePickerToolbarItem {}
);

#[cfg(feature = "NSToolbarItem")]
impl NSSharingServicePickerToolbarItem {
    extern_methods!(
        #[cfg(feature = "NSSharingService")]
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSSharingServicePickerToolbarItemDelegate>>>;

        #[cfg(feature = "NSSharingService")]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSharingServicePickerToolbarItemDelegate>>,
        );
    );
}

/// Methods declared on superclass `NSToolbarItem`.
#[cfg(feature = "NSToolbarItem")]
impl NSSharingServicePickerToolbarItem {
    extern_methods!(
        #[cfg(feature = "NSToolbar")]
        /// Initialize the toolbar item with an identifier which is a development language string used by the toolbar and its delegate for identification purposes.
        #[unsafe(method(initWithItemIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithItemIdentifier(
            this: Allocated<Self>,
            item_identifier: &NSToolbarItemIdentifier,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "NSToolbarItem")]
impl NSSharingServicePickerToolbarItem {
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
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssharingservicepickertoolbaritemdelegate?language=objc)
    #[cfg(feature = "NSSharingService")]
    pub unsafe trait NSSharingServicePickerToolbarItemDelegate:
        NSSharingServicePickerDelegate + MainThreadOnly
    {
        #[cfg(feature = "NSToolbarItem")]
        /// Return the items that represent the objects to be shared.
        /// They must conform to the
        /// <NSPasteboardWriting
        /// > protocol or be an NSItemProvider. (e.g. NSString, NSImage, NSURL, etc.).
        #[unsafe(method(itemsForSharingServicePickerToolbarItem:))]
        #[unsafe(method_family = none)]
        unsafe fn itemsForSharingServicePickerToolbarItem(
            &self,
            picker_toolbar_item: &NSSharingServicePickerToolbarItem,
        ) -> Retained<NSArray>;
    }
);
