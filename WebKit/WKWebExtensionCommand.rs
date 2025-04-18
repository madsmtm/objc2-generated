//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A ``WKWebExtensionCommand`` object encapsulates the properties for an individual web extension command.
    ///
    /// Provides access to command properties such as a unique identifier, a descriptive title, and shortcut keys. Commands
    /// can be used by a web extension to perform specific actions within a web extension context, such toggling features, or interacting with
    /// web content. These commands enhance the functionality of the extension by allowing users to invoke actions quickly.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebextensioncommand?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKWebExtensionCommand;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKWebExtensionCommand {}
);

impl WKWebExtensionCommand {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "WKWebExtensionContext")]
        /// The web extension context associated with the command.
        #[unsafe(method(webExtensionContext))]
        #[unsafe(method_family = none)]
        pub unsafe fn webExtensionContext(&self) -> Option<Retained<WKWebExtensionContext>>;

        /// A unique identifier for the command.
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        /// Descriptive title for the command aiding discoverability.
        ///
        /// This title can be displayed in user interface elements such as keyboard shortcuts lists or menu items to help users understand its purpose.
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// The primary key used to trigger the command, distinct from any modifier flags.
        ///
        /// This property can be customized within the app to avoid conflicts with existing shortcuts or to enable user personalization.
        /// It should accurately represent the activation key as used by the app, which the extension can use to display the complete shortcut in its interface.
        /// If no shortcut is desired for the command, the property should be set to `nil`. This value should be saved and restored as needed by the app.
        #[unsafe(method(activationKey))]
        #[unsafe(method_family = none)]
        pub unsafe fn activationKey(&self) -> Option<Retained<NSString>>;

        /// Setter for [`activationKey`][Self::activationKey].
        #[unsafe(method(setActivationKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setActivationKey(&self, activation_key: Option<&NSString>);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[unsafe(method(modifierFlags))]
        #[unsafe(method_family = none)]
        pub unsafe fn modifierFlags(&self) -> NSEventModifierFlags;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// Setter for [`modifierFlags`][Self::modifierFlags].
        #[unsafe(method(setModifierFlags:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setModifierFlags(&self, modifier_flags: NSEventModifierFlags);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[unsafe(method(menuItem))]
        #[unsafe(method_family = none)]
        pub unsafe fn menuItem(&self) -> Retained<NSMenuItem>;
    );
}
