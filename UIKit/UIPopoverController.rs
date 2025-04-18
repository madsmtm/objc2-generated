//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipopovercontroller?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
    pub struct UIPopoverController;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIPopoverController {}
);

#[cfg(feature = "UIAppearance")]
extern_conformance!(
    unsafe impl UIAppearanceContainer for UIPopoverController {}
);

impl UIPopoverController {
    extern_methods!(
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(initWithContentViewController:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithContentViewController(
            this: Allocated<Self>,
            view_controller: &UIViewController,
        ) -> Retained<Self>;

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPopoverControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIPopoverControllerDelegate>>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(contentViewController))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentViewController(&self) -> Retained<UIViewController>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        /// Setter for [`contentViewController`][Self::contentViewController].
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(setContentViewController:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentViewController(&self, content_view_controller: &UIViewController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(setContentViewController:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentViewController_animated(
            &self,
            view_controller: &UIViewController,
            animated: bool,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(popoverContentSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn popoverContentSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`popoverContentSize`][Self::popoverContentSize].
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(setPopoverContentSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPopoverContentSize(&self, popover_content_size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(setPopoverContentSize:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPopoverContentSize_animated(&self, size: CGSize, animated: bool);

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(isPopoverVisible))]
        #[unsafe(method_family = none)]
        pub unsafe fn isPopoverVisible(&self) -> bool;

        #[cfg(feature = "UIPopoverSupport")]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(popoverArrowDirection))]
        #[unsafe(method_family = none)]
        pub unsafe fn popoverArrowDirection(&self) -> UIPopoverArrowDirection;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(passthroughViews))]
        #[unsafe(method_family = none)]
        pub unsafe fn passthroughViews(&self) -> Option<Retained<NSArray<UIView>>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// Setter for [`passthroughViews`][Self::passthroughViews].
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(setPassthroughViews:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPassthroughViews(&self, passthrough_views: Option<&NSArray<UIView>>);

        #[cfg(all(
            feature = "UIPopoverSupport",
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(presentPopoverFromRect:inView:permittedArrowDirections:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentPopoverFromRect_inView_permittedArrowDirections_animated(
            &self,
            rect: CGRect,
            view: &UIView,
            arrow_directions: UIPopoverArrowDirection,
            animated: bool,
        );

        #[cfg(all(
            feature = "UIBarButtonItem",
            feature = "UIBarItem",
            feature = "UIPopoverSupport"
        ))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(presentPopoverFromBarButtonItem:permittedArrowDirections:animated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentPopoverFromBarButtonItem_permittedArrowDirections_animated(
            &self,
            item: &UIBarButtonItem,
            arrow_directions: UIPopoverArrowDirection,
            animated: bool,
        );

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method(dismissPopoverAnimated:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dismissPopoverAnimated(&self, animated: bool);

        #[cfg(feature = "UIColor")]
        #[unsafe(method(backgroundColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`backgroundColor`][Self::backgroundColor].
        #[unsafe(method(setBackgroundColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&UIColor>);

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[unsafe(method(popoverLayoutMargins))]
        #[unsafe(method_family = none)]
        pub unsafe fn popoverLayoutMargins(&self) -> UIEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`popoverLayoutMargins`][Self::popoverLayoutMargins].
        #[unsafe(method(setPopoverLayoutMargins:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPopoverLayoutMargins(&self, popover_layout_margins: UIEdgeInsets);

        #[unsafe(method(popoverBackgroundViewClass))]
        #[unsafe(method_family = none)]
        pub unsafe fn popoverBackgroundViewClass(&self) -> Option<&'static AnyClass>;

        /// Setter for [`popoverBackgroundViewClass`][Self::popoverBackgroundViewClass].
        #[unsafe(method(setPopoverBackgroundViewClass:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPopoverBackgroundViewClass(
            &self,
            popover_background_view_class: Option<&AnyClass>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl UIPopoverController {
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
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipopovercontrollerdelegate?language=objc)
    pub unsafe trait UIPopoverControllerDelegate: NSObjectProtocol + MainThreadOnly {
        #[deprecated]
        #[optional]
        #[unsafe(method(popoverControllerShouldDismissPopover:))]
        #[unsafe(method_family = none)]
        unsafe fn popoverControllerShouldDismissPopover(
            &self,
            popover_controller: &UIPopoverController,
        ) -> bool;

        #[deprecated]
        #[optional]
        #[unsafe(method(popoverControllerDidDismissPopover:))]
        #[unsafe(method_family = none)]
        unsafe fn popoverControllerDidDismissPopover(
            &self,
            popover_controller: &UIPopoverController,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[deprecated]
        #[optional]
        #[unsafe(method(popoverController:willRepositionPopoverToRect:inView:))]
        #[unsafe(method_family = none)]
        unsafe fn popoverController_willRepositionPopoverToRect_inView(
            &self,
            popover_controller: &UIPopoverController,
            rect: NonNull<CGRect>,
            view: &mut Retained<UIView>,
        );
    }
);
