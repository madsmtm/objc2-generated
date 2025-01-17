//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentproperties?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDocumentProperties;
);

unsafe impl NSObjectProtocol for UIDocumentProperties {}

extern_methods!(
    unsafe impl UIDocumentProperties {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        /// When initializing with a url, UIKit will automatically lookup metadata based on the data at that url.
        #[unsafe(method_family(init))]
        #[method_id(initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[cfg(all(feature = "UIDragItem", feature = "UIDragSession", feature = "block2"))]
        /// To support drag
        /// &
        /// drop, assign a closure to return an array of drag items corresponding to the represented document.
        #[method(dragItemsProvider)]
        pub unsafe fn dragItemsProvider(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<ProtocolObject<dyn UIDragSession>>) -> NonNull<NSArray<UIDragItem>>,
        >;

        #[cfg(all(feature = "UIDragItem", feature = "UIDragSession", feature = "block2"))]
        /// Setter for [`dragItemsProvider`][Self::dragItemsProvider].
        #[method(setDragItemsProvider:)]
        pub unsafe fn setDragItemsProvider(
            &self,
            drag_items_provider: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIDragSession>>,
                    ) -> NonNull<NSArray<UIDragItem>>,
                >,
            >,
        );

        #[cfg(all(
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        /// To support sharing, assign a closure to return a UIActivityViewController configured to share the represented document.
        #[method(activityViewControllerProvider)]
        pub unsafe fn activityViewControllerProvider(
            &self,
        ) -> *mut block2::Block<dyn Fn() -> NonNull<UIActivityViewController>>;

        #[cfg(all(
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        /// Setter for [`activityViewControllerProvider`][Self::activityViewControllerProvider].
        #[method(setActivityViewControllerProvider:)]
        pub unsafe fn setActivityViewControllerProvider(
            &self,
            activity_view_controller_provider: Option<
                &block2::Block<dyn Fn() -> NonNull<UIActivityViewController>>,
            >,
        );

        /// If enabled, shows an icon representation of the document in the navigation bar.
        #[method(wantsIconRepresentation)]
        pub unsafe fn wantsIconRepresentation(&self) -> bool;

        /// Setter for [`wantsIconRepresentation`][Self::wantsIconRepresentation].
        #[method(setWantsIconRepresentation:)]
        pub unsafe fn setWantsIconRepresentation(&self, wants_icon_representation: bool);
    }
);
