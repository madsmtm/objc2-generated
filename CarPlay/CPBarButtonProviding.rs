//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cpbarbuttonproviding?language=objc)
    pub unsafe trait CPBarButtonProviding: NSObjectProtocol {
        #[cfg(feature = "CPBarButton")]
        /// An array of bar buttons to be displayed on the leading side of the navigation bar.
        ///
        ///
        /// Note: The navigation bar may display a maximum of 2 buttons in the leading space.
        /// Setting more than 2 buttons to this property will only display the first 2 buttons.
        #[unsafe(method(leadingNavigationBarButtons))]
        #[unsafe(method_family = none)]
        unsafe fn leadingNavigationBarButtons(&self) -> Retained<NSArray<CPBarButton>>;

        #[cfg(feature = "CPBarButton")]
        /// Setter for [`leadingNavigationBarButtons`][Self::leadingNavigationBarButtons].
        #[unsafe(method(setLeadingNavigationBarButtons:))]
        #[unsafe(method_family = none)]
        unsafe fn setLeadingNavigationBarButtons(
            &self,
            leading_navigation_bar_buttons: &NSArray<CPBarButton>,
        );

        #[cfg(feature = "CPBarButton")]
        /// An array of bar buttons to be displayed on the trailing side of the navigation bar.
        ///
        ///
        /// Note: The navigation bar may display a maximum of 2 buttons in the trailing space.
        /// Setting more than 2 buttons to this property will only display the first 2 buttons.
        #[unsafe(method(trailingNavigationBarButtons))]
        #[unsafe(method_family = none)]
        unsafe fn trailingNavigationBarButtons(&self) -> Retained<NSArray<CPBarButton>>;

        #[cfg(feature = "CPBarButton")]
        /// Setter for [`trailingNavigationBarButtons`][Self::trailingNavigationBarButtons].
        #[unsafe(method(setTrailingNavigationBarButtons:))]
        #[unsafe(method_family = none)]
        unsafe fn setTrailingNavigationBarButtons(
            &self,
            trailing_navigation_bar_buttons: &NSArray<CPBarButton>,
        );

        #[cfg(feature = "CPBarButton")]
        #[unsafe(method(backButton))]
        #[unsafe(method_family = none)]
        unsafe fn backButton(&self) -> Option<Retained<CPBarButton>>;

        #[cfg(feature = "CPBarButton")]
        /// Setter for [`backButton`][Self::backButton].
        #[unsafe(method(setBackButton:))]
        #[unsafe(method_family = none)]
        unsafe fn setBackButton(&self, back_button: Option<&CPBarButton>);
    }
);
