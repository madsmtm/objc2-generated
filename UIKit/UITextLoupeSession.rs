//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextLoupeSession;

    unsafe impl ClassType for UITextLoupeSession {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextLoupeSession {}

extern_methods!(
    unsafe impl UITextLoupeSession {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other beginLoupeSessionAtPoint:fromSelectionWidgetView:inView:)]
        pub unsafe fn beginLoupeSessionAtPoint_fromSelectionWidgetView_inView(
            point: CGPoint,
            selection_widget: Option<&UIView>,
            interaction_view: &UIView,
        ) -> Option<Retained<Self>>;

        #[method(moveToPoint:withCaretRect:trackingCaret:)]
        pub unsafe fn moveToPoint_withCaretRect_trackingCaret(
            &self,
            point: CGPoint,
            caret_rect: CGRect,
            tracks_caret: bool,
        );

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextLoupeSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
