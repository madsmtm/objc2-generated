//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A ``WKWebExtensionAction`` object encapsulates the properties for an individual web extension action.
    ///
    /// Provides access to action properties such as popup, icon, and title, with tab-specific values.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebextensionaction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKWebExtensionAction;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKWebExtensionAction {}
);

impl WKWebExtensionAction {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(&self) -> Retained<Self>;

        #[cfg(feature = "WKWebExtensionContext")]
        /// The extension context to which this action is related.
        #[unsafe(method(webExtensionContext))]
        #[unsafe(method_family = none)]
        pub unsafe fn webExtensionContext(&self) -> Option<Retained<WKWebExtensionContext>>;

        #[cfg(feature = "WKWebExtensionTab")]
        /// The tab that this action is associated with, or `nil` if it is the default action.
        ///
        /// When this property is `nil`, it indicates that the action is the default action and not associated with a specific tab.
        #[unsafe(method(associatedTab))]
        #[unsafe(method_family = none)]
        pub unsafe fn associatedTab(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn WKWebExtensionTab>>>;

        #[cfg(all(feature = "objc2-app-kit", feature = "objc2-core-foundation"))]
        #[cfg(target_os = "macos")]
        #[unsafe(method(iconForSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn iconForSize(&self, size: CGSize) -> Option<Retained<NSImage>>;

        /// The localized display label for the action.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        /// The badge text for the action.
        ///
        /// Provides the text that appears on the badge for the action. An empty string signifies that no badge should be shown.
        #[unsafe(method(badgeText))]
        #[unsafe(method_family = none)]
        pub unsafe fn badgeText(&self) -> Retained<NSString>;

        /// A Boolean value indicating whether the badge text is unread.
        ///
        /// This property is automatically set to `YES` when ``badgeText`` changes and is not empty. If ``badgeText`` becomes empty or the
        /// popup associated with the action is presented, this property is automatically set to `NO`. Additionally, it should be set to `NO` by the app when the badge
        /// has been presented to the user. This property is useful for higher-level notification badges when extensions might be hidden behind an action sheet.
        #[unsafe(method(hasUnreadBadgeText))]
        #[unsafe(method_family = none)]
        pub unsafe fn hasUnreadBadgeText(&self) -> bool;

        /// Setter for [`hasUnreadBadgeText`][Self::hasUnreadBadgeText].
        #[unsafe(method(setHasUnreadBadgeText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHasUnreadBadgeText(&self, has_unread_badge_text: bool);

        /// The name shown when inspecting the popup web view.
        ///
        /// This is the text that will appear when inspecting the popup web view.
        #[unsafe(method(inspectionName))]
        #[unsafe(method_family = none)]
        pub unsafe fn inspectionName(&self) -> Option<Retained<NSString>>;

        /// Setter for [`inspectionName`][Self::inspectionName].
        #[unsafe(method(setInspectionName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInspectionName(&self, inspection_name: Option<&NSString>);

        /// A Boolean value indicating whether the action is enabled.
        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[unsafe(method(menuItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn menuItems(&self) -> Retained<NSArray<NSMenuItem>>;

        /// A Boolean value indicating whether the action has a popup.
        ///
        /// Use this property to check if the action has a popup before attempting to show any popup views.
        #[unsafe(method(presentsPopup))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentsPopup(&self) -> bool;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// A popover that presents a web view loaded with the popup page for this action, or `nil` if no popup is specified.
        ///
        /// This popover contains a view controller with a web view preloaded with the popup page. It automatically adjusts its size to fit
        /// the web view's content size. The ``presentsPopup`` property should be checked to determine the availability of a popup before using this
        /// property.  Dismissing the popover will close the popup and unload the web view.
        ///
        /// See also: presentsPopup
        #[unsafe(method(popupPopover))]
        #[unsafe(method_family = none)]
        pub unsafe fn popupPopover(&self) -> Option<Retained<NSPopover>>;

        #[cfg(all(feature = "WKWebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// A web view loaded with the popup page for this action, or `nil` if no popup is specified.
        ///
        /// The web view will be preloaded with the popup page upon first access or after it has been unloaded. Use the ``presentsPopup``
        /// property to determine whether a popup should be displayed before using this property.
        ///
        /// See also: presentsPopup
        #[unsafe(method(popupWebView))]
        #[unsafe(method_family = none)]
        pub unsafe fn popupWebView(&self) -> Option<Retained<WKWebView>>;

        /// Triggers the dismissal process of the popup.
        ///
        /// Invoke this method to manage the popup's lifecycle, ensuring the web view is unloaded and resources are released once the
        /// popup closes. This method is automatically called upon the dismissal of the action's ``UIViewController`` or ``NSPopover``.  For custom
        /// scenarios where the popup's lifecycle is manually managed, it must be explicitly invoked to ensure proper closure.
        #[unsafe(method(closePopup))]
        #[unsafe(method_family = none)]
        pub unsafe fn closePopup(&self);
    );
}

/// Methods declared on superclass `NSObject`.
impl WKWebExtensionAction {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new_class(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
