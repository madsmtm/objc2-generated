//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontentunavailablebuttonproperties?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIContentUnavailableButtonProperties;
);

extern_conformance!(
    unsafe impl NSCoding for UIContentUnavailableButtonProperties {}
);

extern_conformance!(
    unsafe impl NSCopying for UIContentUnavailableButtonProperties {}
);

unsafe impl CopyingHelper for UIContentUnavailableButtonProperties {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UIContentUnavailableButtonProperties {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for UIContentUnavailableButtonProperties {}
);

impl UIContentUnavailableButtonProperties {
    extern_methods!(
        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        /// The primary action of the button.
        #[unsafe(method(primaryAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn primaryAction(&self) -> Option<Retained<UIAction>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        /// Setter for [`primaryAction`][Self::primaryAction].
        #[unsafe(method(setPrimaryAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPrimaryAction(&self, primary_action: Option<&UIAction>);

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        /// An optional menu for the button to display.
        #[unsafe(method(menu))]
        #[unsafe(method_family = none)]
        pub unsafe fn menu(&self) -> Option<Retained<UIMenu>>;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        /// Setter for [`menu`][Self::menu].
        #[unsafe(method(setMenu:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMenu(&self, menu: Option<&UIMenu>);

        /// Whether the button is enabled. Default is YES.
        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "UIButton")]
        /// The role of the button.
        #[unsafe(method(role))]
        #[unsafe(method_family = none)]
        pub unsafe fn role(&self) -> UIButtonRole;

        #[cfg(feature = "UIButton")]
        /// Setter for [`role`][Self::role].
        #[unsafe(method(setRole:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRole(&self, role: UIButtonRole);
    );
}

/// Methods declared on superclass `NSObject`.
impl UIContentUnavailableButtonProperties {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
