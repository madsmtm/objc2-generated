//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/increatetasklistintent?language=objc)
    #[unsafe(super(INIntent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntent")]
    #[deprecated = "INCreateTaskListIntent is deprecated. There is no replacement."]
    pub struct INCreateTaskListIntent;
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCoding for INCreateTaskListIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCopying for INCreateTaskListIntent {}
);

#[cfg(feature = "INIntent")]
unsafe impl CopyingHelper for INCreateTaskListIntent {
    type Result = Self;
}

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INCreateTaskListIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSSecureCoding for INCreateTaskListIntent {}
);

#[cfg(feature = "INIntent")]
impl INCreateTaskListIntent {
    extern_methods!(
        #[cfg(feature = "INSpeakableString")]
        #[deprecated = "INCreateTaskListIntent is deprecated. There is no replacement."]
        #[unsafe(method(initWithTitle:taskTitles:groupName:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTitle_taskTitles_groupName(
            this: Allocated<Self>,
            title: Option<&INSpeakableString>,
            task_titles: Option<&NSArray<INSpeakableString>>,
            group_name: Option<&INSpeakableString>,
        ) -> Retained<Self>;

        #[cfg(feature = "INSpeakableString")]
        #[deprecated = "INCreateTaskListIntent is deprecated. There is no replacement."]
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Option<Retained<INSpeakableString>>;

        #[cfg(feature = "INSpeakableString")]
        #[deprecated = "INCreateTaskListIntent is deprecated. There is no replacement."]
        #[unsafe(method(taskTitles))]
        #[unsafe(method_family = none)]
        pub unsafe fn taskTitles(&self) -> Option<Retained<NSArray<INSpeakableString>>>;

        #[cfg(feature = "INSpeakableString")]
        #[deprecated = "INCreateTaskListIntent is deprecated. There is no replacement."]
        #[unsafe(method(groupName))]
        #[unsafe(method_family = none)]
        pub unsafe fn groupName(&self) -> Option<Retained<INSpeakableString>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntent")]
impl INCreateTaskListIntent {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// Protocol to declare support for handling an INCreateTaskListIntent. By implementing this protocol, a class can provide logic for resolving, confirming and handling the intent.
    ///
    /// The minimum requirement for an implementing class is that it should be able to handle the intent. The resolution and confirmation methods are optional. The handling method is always called last, after resolving and confirming the intent.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/intents/increatetasklistintenthandling?language=objc)
    #[deprecated = "INCreateTaskListIntentHandling is deprecated. There is no replacement."]
    pub unsafe trait INCreateTaskListIntentHandling: NSObjectProtocol {
        #[cfg(all(
            feature = "INCreateTaskListIntentResponse",
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "block2"
        ))]
        /// Handling method - Execute the task represented by the INCreateTaskListIntent that's passed in
        ///
        /// Called to actually execute the intent. The app must return a response for this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response handling block takes a INCreateTaskListIntentResponse containing the details of the result of having executed the intent
        ///
        ///
        /// See: INCreateTaskListIntentResponse
        #[deprecated = "INCreateTaskListIntentHandling is deprecated. There is no replacement."]
        #[unsafe(method(handleCreateTaskList:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn handleCreateTaskList_completion(
            &self,
            intent: &INCreateTaskListIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INCreateTaskListIntentResponse>)>,
        );

        #[cfg(all(
            feature = "INCreateTaskListIntentResponse",
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "block2"
        ))]
        /// Confirmation method - Validate that this intent is ready for the next step (i.e. handling)
        ///
        /// Called prior to asking the app to handle the intent. The app should return a response object that contains additional information about the intent, which may be relevant for the system to show the user prior to handling. If unimplemented, the system will assume the intent is valid following resolution, and will assume there is no additional information relevant to this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response block contains an INCreateTaskListIntentResponse containing additional details about the intent that may be relevant for the system to show the user prior to handling.
        ///
        ///
        /// See: INCreateTaskListIntentResponse
        #[deprecated = "INCreateTaskListIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(confirmCreateTaskList:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn confirmCreateTaskList_completion(
            &self,
            intent: &INCreateTaskListIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INCreateTaskListIntentResponse>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INSpeakableStringResolutionResult",
            feature = "block2"
        ))]
        /// Resolution methods - Determine if this intent is ready for the next step (confirmation)
        ///
        /// Called to make sure the app extension is capable of handling this intent in its current form. This method is for validating if the intent needs any further fleshing out.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response block contains an INIntentResolutionResult for the parameter being resolved
        ///
        ///
        /// See: INIntentResolutionResult
        #[deprecated = "INCreateTaskListIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(resolveTitleForCreateTaskList:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveTitleForCreateTaskList_withCompletion(
            &self,
            intent: &INCreateTaskListIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INSpeakableStringResolutionResult>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INSpeakableStringResolutionResult",
            feature = "block2"
        ))]
        #[deprecated = "INCreateTaskListIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(resolveTaskTitlesForCreateTaskList:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveTaskTitlesForCreateTaskList_withCompletion(
            &self,
            intent: &INCreateTaskListIntent,
            completion: &block2::DynBlock<
                dyn Fn(NonNull<NSArray<INSpeakableStringResolutionResult>>),
            >,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INSpeakableStringResolutionResult",
            feature = "block2"
        ))]
        #[deprecated = "INCreateTaskListIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(resolveGroupNameForCreateTaskList:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveGroupNameForCreateTaskList_withCompletion(
            &self,
            intent: &INCreateTaskListIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INSpeakableStringResolutionResult>)>,
        );
    }
);
