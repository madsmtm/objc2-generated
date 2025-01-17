//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsscriptexecutioncontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptExecutionContext;
);

unsafe impl NSObjectProtocol for NSScriptExecutionContext {}

extern_methods!(
    unsafe impl NSScriptExecutionContext {
        #[unsafe(method_family(none))]
        #[method_id(sharedScriptExecutionContext)]
        pub unsafe fn sharedScriptExecutionContext() -> Retained<NSScriptExecutionContext>;

        #[unsafe(method_family(none))]
        #[method_id(topLevelObject)]
        pub unsafe fn topLevelObject(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`topLevelObject`][Self::topLevelObject].
        #[method(setTopLevelObject:)]
        pub unsafe fn setTopLevelObject(&self, top_level_object: Option<&AnyObject>);

        #[unsafe(method_family(none))]
        #[method_id(objectBeingTested)]
        pub unsafe fn objectBeingTested(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`objectBeingTested`][Self::objectBeingTested].
        #[method(setObjectBeingTested:)]
        pub unsafe fn setObjectBeingTested(&self, object_being_tested: Option<&AnyObject>);

        #[unsafe(method_family(none))]
        #[method_id(rangeContainerObject)]
        pub unsafe fn rangeContainerObject(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`rangeContainerObject`][Self::rangeContainerObject].
        #[method(setRangeContainerObject:)]
        pub unsafe fn setRangeContainerObject(&self, range_container_object: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSScriptExecutionContext {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
