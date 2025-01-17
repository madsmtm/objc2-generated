//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabsidebaritem?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITabSidebarItem;
);

unsafe impl NSCopying for UITabSidebarItem {}

unsafe impl CopyingHelper for UITabSidebarItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UITabSidebarItem {}

extern_methods!(
    unsafe impl UITabSidebarItem {
        #[cfg(feature = "UITab")]
        /// The tab that the receiver represents. Only one of `tab` or `action` will be valid for an item.
        #[unsafe(method_family(none))]
        #[method_id(tab)]
        pub unsafe fn tab(&self) -> Option<Retained<UITab>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        /// The action that the receiver represents. Only one of `tab` or `action` will be valid for an item.
        #[unsafe(method_family(none))]
        #[method_id(action)]
        pub unsafe fn action(&self) -> Option<Retained<UIAction>>;

        #[cfg(all(
            feature = "UICellConfigurationState",
            feature = "UIViewConfigurationState"
        ))]
        /// The current configuration state of the sidebar item.
        #[unsafe(method_family(none))]
        #[method_id(configurationState)]
        pub unsafe fn configurationState(&self) -> Retained<UICellConfigurationState>;

        #[cfg(feature = "UIContentConfiguration")]
        /// The content coinfiguration to use when displaying this item.
        #[unsafe(method_family(none))]
        #[method_id(contentConfiguration)]
        pub unsafe fn contentConfiguration(
            &self,
        ) -> Retained<ProtocolObject<dyn UIContentConfiguration>>;

        #[cfg(feature = "UIContentConfiguration")]
        /// Setter for [`contentConfiguration`][Self::contentConfiguration].
        #[method(setContentConfiguration:)]
        pub unsafe fn setContentConfiguration(
            &self,
            content_configuration: &ProtocolObject<dyn UIContentConfiguration>,
        );

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// The background configuration to use when displaying this item.
        #[unsafe(method_family(none))]
        #[method_id(backgroundConfiguration)]
        pub unsafe fn backgroundConfiguration(&self) -> Retained<UIBackgroundConfiguration>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// Setter for [`backgroundConfiguration`][Self::backgroundConfiguration].
        #[method(setBackgroundConfiguration:)]
        pub unsafe fn setBackgroundConfiguration(
            &self,
            background_configuration: &UIBackgroundConfiguration,
        );

        #[cfg(feature = "UICellAccessory")]
        /// Cell accessories to use when displaying this item. Some accessories may not be shown if it conflicts with system default accessories.
        #[unsafe(method_family(none))]
        #[method_id(accessories)]
        pub unsafe fn accessories(&self) -> Retained<NSArray<UICellAccessory>>;

        #[cfg(feature = "UICellAccessory")]
        /// Setter for [`accessories`][Self::accessories].
        #[method(setAccessories:)]
        pub unsafe fn setAccessories(&self, accessories: &NSArray<UICellAccessory>);

        #[cfg(feature = "UIListContentConfiguration")]
        /// Returns the default content configuration for this item and the configuration state.
        #[unsafe(method_family(none))]
        #[method_id(defaultContentConfiguration)]
        pub unsafe fn defaultContentConfiguration(&self) -> Retained<UIListContentConfiguration>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        /// Returns the default background configuration for this item and the configuration state.
        #[unsafe(method_family(none))]
        #[method_id(defaultBackgroundConfiguration)]
        pub unsafe fn defaultBackgroundConfiguration(&self) -> Retained<UIBackgroundConfiguration>;

        /// Creates a sidebar item from the specified request. The sidebar item will be preconfigured with the appropriate defaults for its content.
        #[unsafe(method_family(none))]
        #[method_id(itemFromRequest:)]
        pub unsafe fn itemFromRequest(request: &UITabSidebarItemRequest) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabsidebaritemrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITabSidebarItemRequest;
);

unsafe impl NSObjectProtocol for UITabSidebarItemRequest {}

extern_methods!(
    unsafe impl UITabSidebarItemRequest {
        #[cfg(feature = "UITab")]
        /// The tab that the receiver represents. Only one of `tab` or `action` will be valid for an item.
        #[unsafe(method_family(none))]
        #[method_id(tab)]
        pub unsafe fn tab(&self) -> Option<Retained<UITab>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        /// The action that the receiver represents. Only one of `tab` or `action` will be valid for an item.
        #[unsafe(method_family(none))]
        #[method_id(action)]
        pub unsafe fn action(&self) -> Option<Retained<UIAction>>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
