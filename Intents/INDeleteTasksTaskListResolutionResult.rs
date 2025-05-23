//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/indeletetaskstasklistunsupportedreason?language=objc)
// NS_ENUM
#[deprecated = "INDeleteTasksTaskListUnsupportedReason is deprecated. There is no replacement."]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INDeleteTasksTaskListUnsupportedReason(pub NSInteger);
impl INDeleteTasksTaskListUnsupportedReason {
    #[doc(alias = "INDeleteTasksTaskListUnsupportedReasonNoTaskListFound")]
    #[deprecated = "INDeleteTasksTaskListUnsupportedReason is deprecated. There is no replacement."]
    pub const NoTaskListFound: Self = Self(1);
}

unsafe impl Encode for INDeleteTasksTaskListUnsupportedReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INDeleteTasksTaskListUnsupportedReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/indeletetaskstasklistresolutionresult?language=objc)
    #[unsafe(super(INTaskListResolutionResult, INIntentResolutionResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "INIntentResolutionResult",
        feature = "INTaskListResolutionResult"
    ))]
    #[deprecated = "INDeleteTasksTaskListResolutionResult is deprecated. There is no replacement."]
    pub struct INDeleteTasksTaskListResolutionResult;
);

#[cfg(all(
    feature = "INIntentResolutionResult",
    feature = "INTaskListResolutionResult"
))]
extern_conformance!(
    unsafe impl NSObjectProtocol for INDeleteTasksTaskListResolutionResult {}
);

#[cfg(all(
    feature = "INIntentResolutionResult",
    feature = "INTaskListResolutionResult"
))]
impl INDeleteTasksTaskListResolutionResult {
    extern_methods!(
        #[deprecated = "INDeleteTasksTaskListResolutionResult is deprecated. There is no replacement."]
        #[unsafe(method(unsupportedForReason:))]
        #[unsafe(method_family = none)]
        pub unsafe fn unsupportedForReason(
            reason: INDeleteTasksTaskListUnsupportedReason,
        ) -> Retained<Self>;

        #[deprecated = "INDeleteTasksTaskListResolutionResult is deprecated. There is no replacement."]
        #[unsafe(method(initWithTaskListResolutionResult:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTaskListResolutionResult(
            this: Allocated<Self>,
            task_list_resolution_result: &INTaskListResolutionResult,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `INTaskListResolutionResult`.
#[cfg(all(
    feature = "INIntentResolutionResult",
    feature = "INTaskListResolutionResult"
))]
impl INDeleteTasksTaskListResolutionResult {
    extern_methods!(
        #[cfg(feature = "INTaskList")]
        #[unsafe(method(successWithResolvedTaskList:))]
        #[unsafe(method_family = none)]
        pub unsafe fn successWithResolvedTaskList(
            resolved_task_list: &INTaskList,
        ) -> Retained<Self>;

        #[cfg(feature = "INTaskList")]
        #[unsafe(method(disambiguationWithTaskListsToDisambiguate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn disambiguationWithTaskListsToDisambiguate(
            task_lists_to_disambiguate: &NSArray<INTaskList>,
        ) -> Retained<Self>;

        #[cfg(feature = "INTaskList")]
        #[unsafe(method(confirmationRequiredWithTaskListToConfirm:))]
        #[unsafe(method_family = none)]
        pub unsafe fn confirmationRequiredWithTaskListToConfirm(
            task_list_to_confirm: Option<&INTaskList>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `INIntentResolutionResult`.
#[cfg(all(
    feature = "INIntentResolutionResult",
    feature = "INTaskListResolutionResult"
))]
impl INDeleteTasksTaskListResolutionResult {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(needsValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn needsValue() -> Retained<Self>;

        #[unsafe(method(notRequired))]
        #[unsafe(method_family = none)]
        pub unsafe fn notRequired() -> Retained<Self>;

        #[unsafe(method(unsupported))]
        #[unsafe(method_family = none)]
        pub unsafe fn unsupported() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(
    feature = "INIntentResolutionResult",
    feature = "INTaskListResolutionResult"
))]
impl INDeleteTasksTaskListResolutionResult {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
