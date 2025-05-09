//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifeedbackgenerator?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFeedbackGenerator;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIFeedbackGenerator {}
);

impl UIFeedbackGenerator {
    extern_methods!(
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// initalize the generator with a view to attach it to the provided view as an interaction.
        #[unsafe(method(feedbackGeneratorForView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn feedbackGeneratorForView(view: &UIView) -> Retained<Self>;

        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// informs self that it will likely receive events soon, so that it can ensure minimal latency for any feedback generated
        /// safe to call more than once before the generator receives an event, if events are still imminently possible
        #[unsafe(method(prepare))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepare(&self);
    );
}

/// Methods declared on superclass `NSObject`.
impl UIFeedbackGenerator {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

impl UIFeedbackGenerator {
    extern_methods!();
}

#[cfg(feature = "UIInteraction")]
extern_conformance!(
    unsafe impl UIInteraction for UIFeedbackGenerator {}
);
