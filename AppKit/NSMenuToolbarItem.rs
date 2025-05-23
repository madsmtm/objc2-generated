//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsmenutoolbaritem?language=objc)
    #[unsafe(super(NSToolbarItem, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSToolbarItem")]
    pub struct NSMenuToolbarItem;
);

#[cfg(feature = "NSToolbarItem")]
extern_conformance!(
    unsafe impl NSCopying for NSMenuToolbarItem {}
);

#[cfg(feature = "NSToolbarItem")]
unsafe impl CopyingHelper for NSMenuToolbarItem {
    type Result = Self;
}

#[cfg(feature = "NSToolbarItem")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSMenuToolbarItem {}
);

#[cfg(feature = "NSToolbarItem")]
impl NSMenuToolbarItem {
    extern_methods!(
        #[cfg(feature = "NSMenu")]
        #[unsafe(method(menu))]
        #[unsafe(method_family = none)]
        pub unsafe fn menu(&self) -> Retained<NSMenu>;

        #[cfg(feature = "NSMenu")]
        /// Setter for [`menu`][Self::menu].
        #[unsafe(method(setMenu:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMenu(&self, menu: &NSMenu);

        #[unsafe(method(showsIndicator))]
        #[unsafe(method_family = none)]
        pub unsafe fn showsIndicator(&self) -> bool;

        /// Setter for [`showsIndicator`][Self::showsIndicator].
        #[unsafe(method(setShowsIndicator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShowsIndicator(&self, shows_indicator: bool);
    );
}

/// Methods declared on superclass `NSToolbarItem`.
#[cfg(feature = "NSToolbarItem")]
impl NSMenuToolbarItem {
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
impl NSMenuToolbarItem {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
