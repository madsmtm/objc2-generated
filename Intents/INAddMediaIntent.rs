//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inaddmediaintent?language=objc)
    #[unsafe(super(INIntent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntent")]
    pub struct INAddMediaIntent;
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCoding for INAddMediaIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCopying for INAddMediaIntent {}
);

#[cfg(feature = "INIntent")]
unsafe impl CopyingHelper for INAddMediaIntent {
    type Result = Self;
}

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INAddMediaIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSSecureCoding for INAddMediaIntent {}
);

#[cfg(feature = "INIntent")]
impl INAddMediaIntent {
    extern_methods!(
        #[cfg(all(
            feature = "INMediaDestination",
            feature = "INMediaItem",
            feature = "INMediaSearch"
        ))]
        #[unsafe(method(initWithMediaItems:mediaSearch:mediaDestination:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithMediaItems_mediaSearch_mediaDestination(
            this: Allocated<Self>,
            media_items: Option<&NSArray<INMediaItem>>,
            media_search: Option<&INMediaSearch>,
            media_destination: Option<&INMediaDestination>,
        ) -> Retained<Self>;

        #[cfg(feature = "INMediaItem")]
        #[unsafe(method(mediaItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn mediaItems(&self) -> Option<Retained<NSArray<INMediaItem>>>;

        #[cfg(feature = "INMediaSearch")]
        #[unsafe(method(mediaSearch))]
        #[unsafe(method_family = none)]
        pub unsafe fn mediaSearch(&self) -> Option<Retained<INMediaSearch>>;

        #[cfg(feature = "INMediaDestination")]
        #[unsafe(method(mediaDestination))]
        #[unsafe(method_family = none)]
        pub unsafe fn mediaDestination(&self) -> Option<Retained<INMediaDestination>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntent")]
impl INAddMediaIntent {
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
    /// Protocol to declare support for handling an INAddMediaIntent. By implementing this protocol, a class can provide logic for resolving, confirming and handling the intent.
    ///
    /// The minimum requirement for an implementing class is that it should be able to handle the intent. The resolution and confirmation methods are optional. The handling method is always called last, after resolving and confirming the intent.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/intents/inaddmediaintenthandling?language=objc)
    pub unsafe trait INAddMediaIntentHandling: NSObjectProtocol {
        #[cfg(all(
            feature = "INAddMediaIntentResponse",
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "block2"
        ))]
        /// Handling method - Execute the task represented by the INAddMediaIntent that's passed in
        ///
        /// Called to actually execute the intent. The app must return a response for this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response handling block takes a INAddMediaIntentResponse containing the details of the result of having executed the intent
        ///
        ///
        /// See: INAddMediaIntentResponse
        #[unsafe(method(handleAddMedia:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn handleAddMedia_completion(
            &self,
            intent: &INAddMediaIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INAddMediaIntentResponse>)>,
        );

        #[cfg(all(
            feature = "INAddMediaIntentResponse",
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
        /// Parameter `completion`: The response block contains an INAddMediaIntentResponse containing additional details about the intent that may be relevant for the system to show the user prior to handling.
        ///
        ///
        /// See: INAddMediaIntentResponse
        #[optional]
        #[unsafe(method(confirmAddMedia:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn confirmAddMedia_completion(
            &self,
            intent: &INAddMediaIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INAddMediaIntentResponse>)>,
        );

        #[cfg(all(
            feature = "INAddMediaMediaItemResolutionResult",
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INMediaItemResolutionResult",
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
        #[optional]
        #[unsafe(method(resolveMediaItemsForAddMedia:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveMediaItemsForAddMedia_withCompletion(
            &self,
            intent: &INAddMediaIntent,
            completion: &block2::DynBlock<
                dyn Fn(NonNull<NSArray<INAddMediaMediaItemResolutionResult>>),
            >,
        );

        #[cfg(all(
            feature = "INAddMediaMediaDestinationResolutionResult",
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INMediaDestinationResolutionResult",
            feature = "block2"
        ))]
        #[optional]
        #[unsafe(method(resolveMediaDestinationForAddMedia:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveMediaDestinationForAddMedia_withCompletion(
            &self,
            intent: &INAddMediaIntent,
            completion: &block2::DynBlock<
                dyn Fn(NonNull<INAddMediaMediaDestinationResolutionResult>),
            >,
        );
    }
);
