//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextloupesession?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextLoupeSession;
);

unsafe impl NSObjectProtocol for UITextLoupeSession {}

extern_methods!(
    unsafe impl UITextLoupeSession {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        /// Begins a loupe session at the specified point. The system will animate the presentation of the loupe, as well as its position and hierarchy.
        ///
        ///
        /// Parameter `at`: The point in
        /// `view's`coordinate space where the loupe should begin.
        ///
        /// Parameter `widgetView`: Optionally, a system-provided selection view that the animation can start from.
        ///
        /// Parameter `view`: The coordinate space that all subsequent movement updates are provided in.
        #[unsafe(method_family(none))]
        #[method_id(beginLoupeSessionAtPoint:fromSelectionWidgetView:inView:)]
        pub unsafe fn beginLoupeSessionAtPoint_fromSelectionWidgetView_inView(
            point: CGPoint,
            selection_widget: Option<&UIView>,
            interaction_view: &UIView,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Call this when a text selection gesture moves to a particular point.
        ///
        ///
        /// Parameter `point`: The center point of the touch tracked by the gesture recognizer.
        ///
        /// Parameter `caretRect`: The current position of the caret/range handle. Pass in CGRectNull if there is no current selection/no caret rect visible.
        ///
        /// Parameter `trackingCaret`: Provide YES if the loupe should track the caret instead of the touch.
        #[method(moveToPoint:withCaretRect:trackingCaret:)]
        pub unsafe fn moveToPoint_withCaretRect_trackingCaret(
            &self,
            point: CGPoint,
            caret_rect: CGRect,
            tracks_caret: bool,
        );

        /// Invalidates the loupe session. Hides the loupe and cleans up transient state.
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextLoupeSession {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
