//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/insetaudiosourceincarintent?language=objc)
    #[unsafe(super(INIntent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntent")]
    #[deprecated = "INSetAudioSourceInCarIntent is deprecated. There is no replacement."]
    pub struct INSetAudioSourceInCarIntent;
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCoding for INSetAudioSourceInCarIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCopying for INSetAudioSourceInCarIntent {}
);

#[cfg(feature = "INIntent")]
unsafe impl CopyingHelper for INSetAudioSourceInCarIntent {
    type Result = Self;
}

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INSetAudioSourceInCarIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSSecureCoding for INSetAudioSourceInCarIntent {}
);

#[cfg(feature = "INIntent")]
impl INSetAudioSourceInCarIntent {
    extern_methods!(
        #[cfg(all(feature = "INCarAudioSource", feature = "INRelativeReference"))]
        #[deprecated = "INSetAudioSourceInCarIntent is deprecated. There is no replacement."]
        #[unsafe(method(initWithAudioSource:relativeAudioSourceReference:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAudioSource_relativeAudioSourceReference(
            this: Allocated<Self>,
            audio_source: INCarAudioSource,
            relative_audio_source_reference: INRelativeReference,
        ) -> Retained<Self>;

        #[cfg(feature = "INCarAudioSource")]
        #[deprecated = "INSetAudioSourceInCarIntent is deprecated. There is no replacement."]
        #[unsafe(method(audioSource))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioSource(&self) -> INCarAudioSource;

        #[cfg(feature = "INRelativeReference")]
        #[deprecated = "INSetAudioSourceInCarIntent is deprecated. There is no replacement."]
        #[unsafe(method(relativeAudioSourceReference))]
        #[unsafe(method_family = none)]
        pub unsafe fn relativeAudioSourceReference(&self) -> INRelativeReference;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntent")]
impl INSetAudioSourceInCarIntent {
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
    /// Protocol to declare support for handling an INSetAudioSourceInCarIntent. By implementing this protocol, a class can provide logic for resolving, confirming and handling the intent.
    ///
    /// The minimum requirement for an implementing class is that it should be able to handle the intent. The resolution and confirmation methods are optional. The handling method is always called last, after resolving and confirming the intent.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/intents/insetaudiosourceincarintenthandling?language=objc)
    #[deprecated = "INSetAudioSourceInCarIntentHandling is deprecated. There is no replacement."]
    pub unsafe trait INSetAudioSourceInCarIntentHandling: NSObjectProtocol {
        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "INSetAudioSourceInCarIntentResponse",
            feature = "block2"
        ))]
        /// Handling method - Execute the task represented by the INSetAudioSourceInCarIntent that's passed in
        ///
        /// Called to actually execute the intent. The app must return a response for this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response handling block takes a INSetAudioSourceInCarIntentResponse containing the details of the result of having executed the intent
        ///
        ///
        /// See: INSetAudioSourceInCarIntentResponse
        #[deprecated = "INSetAudioSourceInCarIntentHandling is deprecated. There is no replacement."]
        #[unsafe(method(handleSetAudioSourceInCar:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn handleSetAudioSourceInCar_completion(
            &self,
            intent: &INSetAudioSourceInCarIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INSetAudioSourceInCarIntentResponse>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "INSetAudioSourceInCarIntentResponse",
            feature = "block2"
        ))]
        /// Confirmation method - Validate that this intent is ready for the next step (i.e. handling)
        ///
        /// Called prior to asking the app to handle the intent. The app should return a response object that contains additional information about the intent, which may be relevant for the system to show the user prior to handling. If unimplemented, the system will assume the intent is valid following resolution, and will assume there is no additional information relevant to this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response block contains an INSetAudioSourceInCarIntentResponse containing additional details about the intent that may be relevant for the system to show the user prior to handling.
        ///
        ///
        /// See: INSetAudioSourceInCarIntentResponse
        #[deprecated = "INSetAudioSourceInCarIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(confirmSetAudioSourceInCar:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn confirmSetAudioSourceInCar_completion(
            &self,
            intent: &INSetAudioSourceInCarIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INSetAudioSourceInCarIntentResponse>)>,
        );

        #[cfg(all(
            feature = "INCarAudioSourceResolutionResult",
            feature = "INIntent",
            feature = "INIntentResolutionResult",
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
        #[deprecated = "INSetAudioSourceInCarIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(resolveAudioSourceForSetAudioSourceInCar:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveAudioSourceForSetAudioSourceInCar_withCompletion(
            &self,
            intent: &INSetAudioSourceInCarIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INCarAudioSourceResolutionResult>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INRelativeReferenceResolutionResult",
            feature = "block2"
        ))]
        #[deprecated = "INSetAudioSourceInCarIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(resolveRelativeAudioSourceReferenceForSetAudioSourceInCar:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveRelativeAudioSourceReferenceForSetAudioSourceInCar_withCompletion(
            &self,
            intent: &INSetAudioSourceInCarIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INRelativeReferenceResolutionResult>)>,
        );
    }
);
