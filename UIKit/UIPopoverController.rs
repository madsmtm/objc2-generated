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

unsafe impl NSObjectProtocol for UIPopoverController {}

#[cfg(feature = "UIAppearance")]
unsafe impl UIAppearanceContainer for UIPopoverController {}

extern_methods!(
    unsafe impl UIPopoverController {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method_family(init))]
        #[method_id(initWithContentViewController:)]
        pub unsafe fn initWithContentViewController(
            this: Allocated<Self>,
            view_controller: &UIViewController,
        ) -> Retained<Self>;

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPopoverControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIPopoverControllerDelegate>>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method_family(none))]
        #[method_id(contentViewController)]
        pub unsafe fn contentViewController(&self) -> Retained<UIViewController>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        /// Setter for [`contentViewController`][Self::contentViewController].
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(&self, content_view_controller: &UIViewController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setContentViewController:animated:)]
        pub unsafe fn setContentViewController_animated(
            &self,
            view_controller: &UIViewController,
            animated: bool,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(popoverContentSize)]
        pub unsafe fn popoverContentSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`popoverContentSize`][Self::popoverContentSize].
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setPopoverContentSize:)]
        pub unsafe fn setPopoverContentSize(&self, popover_content_size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setPopoverContentSize:animated:)]
        pub unsafe fn setPopoverContentSize_animated(&self, size: CGSize, animated: bool);

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(isPopoverVisible)]
        pub unsafe fn isPopoverVisible(&self) -> bool;

        #[cfg(feature = "UIPopoverSupport")]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(popoverArrowDirection)]
        pub unsafe fn popoverArrowDirection(&self) -> UIPopoverArrowDirection;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[unsafe(method_family(none))]
        #[method_id(passthroughViews)]
        pub unsafe fn passthroughViews(&self) -> Option<Retained<NSArray<UIView>>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// Setter for [`passthroughViews`][Self::passthroughViews].
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(setPassthroughViews:)]
        pub unsafe fn setPassthroughViews(&self, passthrough_views: Option<&NSArray<UIView>>);

        #[cfg(all(
            feature = "UIPopoverSupport",
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(presentPopoverFromRect:inView:permittedArrowDirections:animated:)]
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
        #[method(presentPopoverFromBarButtonItem:permittedArrowDirections:animated:)]
        pub unsafe fn presentPopoverFromBarButtonItem_permittedArrowDirections_animated(
            &self,
            item: &UIBarButtonItem,
            arrow_directions: UIPopoverArrowDirection,
            animated: bool,
        );

        #[deprecated = "UIPopoverController is deprecated. Popovers are now implemented as UIViewController presentations. Use a modal presentation style of UIModalPresentationPopover and UIPopoverPresentationController."]
        #[method(dismissPopoverAnimated:)]
        pub unsafe fn dismissPopoverAnimated(&self, animated: bool);

        #[cfg(feature = "UIColor")]
        #[unsafe(method_family(none))]
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`backgroundColor`][Self::backgroundColor].
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&UIColor>);

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(popoverLayoutMargins)]
        pub unsafe fn popoverLayoutMargins(&self) -> UIEdgeInsets;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        /// Setter for [`popoverLayoutMargins`][Self::popoverLayoutMargins].
        #[method(setPopoverLayoutMargins:)]
        pub unsafe fn setPopoverLayoutMargins(&self, popover_layout_margins: UIEdgeInsets);

        #[method(popoverBackgroundViewClass)]
        pub unsafe fn popoverBackgroundViewClass(&self) -> Option<&'static AnyClass>;

        /// Setter for [`popoverBackgroundViewClass`][Self::popoverBackgroundViewClass].
        #[method(setPopoverBackgroundViewClass:)]
        pub unsafe fn setPopoverBackgroundViewClass(
            &self,
            popover_background_view_class: Option<&AnyClass>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPopoverController {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipopovercontrollerdelegate?language=objc)
    pub unsafe trait UIPopoverControllerDelegate: NSObjectProtocol + MainThreadOnly {
        #[deprecated]
        #[optional]
        #[method(popoverControllerShouldDismissPopover:)]
        unsafe fn popoverControllerShouldDismissPopover(
            &self,
            popover_controller: &UIPopoverController,
        ) -> bool;

        #[deprecated]
        #[optional]
        #[method(popoverControllerDidDismissPopover:)]
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
        #[method(popoverController:willRepositionPopoverToRect:inView:)]
        unsafe fn popoverController_willRepositionPopoverToRect_inView(
            &self,
            popover_controller: &UIPopoverController,
            rect: NonNull<CGRect>,
            view: &mut Retained<UIView>,
        );
    }
);
