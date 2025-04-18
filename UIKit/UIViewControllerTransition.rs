//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollertransition?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIViewControllerTransition;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIViewControllerTransition {}
);

impl UIViewControllerTransition {
    extern_methods!(
        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "UIZoomTransitionOptions",
            feature = "block2"
        ))]
        /// Zoom from the view provided by the `sourceViewProvider` to the presented or pushed view controller's view.
        /// The transition's `sourceViewProvider` is called whenever the transition needs to request a source view.
        /// Note that it may be called multiple times during the transition's lifecycle in order
        /// to ensure that the transition incorporates the most up-to-date visuals.
        ///
        /// **Example**
        ///
        /// Consider an interface where a user may tap a cell representing a city to present a detail view.
        /// In the detail view, they may swipe left or right to go to the next city in the list. When the detail view
        /// is dismissed, it should un-zoom to the currently selected city rather than the one that was first selected.
        /// ```text
        /// cityViewController.preferredTransition = .zoom { context in
        /// let displayed = context.displayedViewController as! CityViewController
        /// let source = context.sourceViewController as! CityListViewController
        /// return source.cell(for: displayed.cityId)
        /// }
        /// present(cityViewController, animated: true)
        /// ```
        #[unsafe(method(zoomWithOptions:sourceViewProvider:))]
        #[unsafe(method_family = none)]
        pub unsafe fn zoomWithOptions_sourceViewProvider(
            options: Option<&UIZoomTransitionOptions>,
            source_view_provider: &block2::DynBlock<
                dyn Fn(NonNull<UIZoomTransitionSourceViewProviderContext>) -> *mut UIView,
            >,
        ) -> Retained<Self>;

        /// View slides up from the bottom of the screen. Same as `UIModalTransitionStyle.coverVertical`.
        #[unsafe(method(coverVerticalTransition))]
        #[unsafe(method_family = none)]
        pub unsafe fn coverVerticalTransition() -> Retained<Self>;

        /// View flips horizontally in 3D. Same as `UIModalTransitionStyle.flipHorizontal`.
        #[unsafe(method(flipHorizontalTransition))]
        #[unsafe(method_family = none)]
        pub unsafe fn flipHorizontalTransition() -> Retained<Self>;

        /// Fades out the current view while fading in the new view. Same as `UIModalTransitionStyle.crossDissolve`.
        #[unsafe(method(crossDissolveTransition))]
        #[unsafe(method_family = none)]
        pub unsafe fn crossDissolveTransition() -> Retained<Self>;

        /// One corner of the current view curls up to reveal the presented view underneath. Same as `UIModalTransitionStyle.partialCurl`.
        #[unsafe(method(partialCurlTransition))]
        #[unsafe(method_family = none)]
        pub unsafe fn partialCurlTransition() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uizoomtransitionsourceviewprovidercontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIZoomTransitionSourceViewProviderContext;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIZoomTransitionSourceViewProviderContext {}
);

impl UIZoomTransitionSourceViewProviderContext {
    extern_methods!(
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        /// View controller that is the source of the zoom transition.
        #[unsafe(method(sourceViewController))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceViewController(
            &self,
            mtm: MainThreadMarker,
        ) -> Retained<UIViewController>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        /// The view controller being zoomed into by the transition.
        #[unsafe(method(zoomedViewController))]
        #[unsafe(method_family = none)]
        pub unsafe fn zoomedViewController(
            &self,
            mtm: MainThreadMarker,
        ) -> Retained<UIViewController>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
