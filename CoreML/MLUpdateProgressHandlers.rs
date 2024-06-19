//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLUpdateProgressHandlers;

    unsafe impl ClassType for MLUpdateProgressHandlers {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MLUpdateProgressHandlers {}

extern_methods!(
    unsafe impl MLUpdateProgressHandlers {
        #[cfg(all(
            feature = "MLUpdateContext",
            feature = "MLUpdateProgressEvent",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initForEvents:progressHandler:completionHandler:)]
        pub unsafe fn initForEvents_progressHandler_completionHandler(
            this: Allocated<Self>,
            interested_events: MLUpdateProgressEvent,
            progress_handler: Option<&block2::Block<dyn Fn(NonNull<MLUpdateContext>)>>,
            completion_handler: &block2::Block<dyn Fn(NonNull<MLUpdateContext>)>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);