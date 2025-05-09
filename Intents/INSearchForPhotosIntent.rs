//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/insearchforphotosintent?language=objc)
    #[unsafe(super(INIntent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntent")]
    #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
    pub struct INSearchForPhotosIntent;
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCoding for INSearchForPhotosIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCopying for INSearchForPhotosIntent {}
);

#[cfg(feature = "INIntent")]
unsafe impl CopyingHelper for INSearchForPhotosIntent {
    type Result = Self;
}

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INSearchForPhotosIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSSecureCoding for INSearchForPhotosIntent {}
);

#[cfg(feature = "INIntent")]
impl INSearchForPhotosIntent {
    extern_methods!(
        #[cfg(all(
            feature = "INDateComponentsRange",
            feature = "INPerson",
            feature = "INPhotoAttributeOptions",
            feature = "objc2-core-location"
        ))]
        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(initWithDateCreated:locationCreated:albumName:searchTerms:includedAttributes:excludedAttributes:peopleInPhoto:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDateCreated_locationCreated_albumName_searchTerms_includedAttributes_excludedAttributes_peopleInPhoto(
            this: Allocated<Self>,
            date_created: Option<&INDateComponentsRange>,
            location_created: Option<&CLPlacemark>,
            album_name: Option<&NSString>,
            search_terms: Option<&NSArray<NSString>>,
            included_attributes: INPhotoAttributeOptions,
            excluded_attributes: INPhotoAttributeOptions,
            people_in_photo: Option<&NSArray<INPerson>>,
        ) -> Retained<Self>;

        #[cfg(feature = "INDateComponentsRange")]
        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(dateCreated))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateCreated(&self) -> Option<Retained<INDateComponentsRange>>;

        #[cfg(feature = "objc2-core-location")]
        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(locationCreated))]
        #[unsafe(method_family = none)]
        pub unsafe fn locationCreated(&self) -> Option<Retained<CLPlacemark>>;

        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(albumName))]
        #[unsafe(method_family = none)]
        pub unsafe fn albumName(&self) -> Option<Retained<NSString>>;

        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(searchTerms))]
        #[unsafe(method_family = none)]
        pub unsafe fn searchTerms(&self) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(feature = "INConditionalOperator")]
        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(searchTermsOperator))]
        #[unsafe(method_family = none)]
        pub unsafe fn searchTermsOperator(&self) -> INConditionalOperator;

        #[cfg(feature = "INPhotoAttributeOptions")]
        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(includedAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn includedAttributes(&self) -> INPhotoAttributeOptions;

        #[cfg(feature = "INPhotoAttributeOptions")]
        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(excludedAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn excludedAttributes(&self) -> INPhotoAttributeOptions;

        #[cfg(feature = "INPerson")]
        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(peopleInPhoto))]
        #[unsafe(method_family = none)]
        pub unsafe fn peopleInPhoto(&self) -> Option<Retained<NSArray<INPerson>>>;

        #[cfg(feature = "INConditionalOperator")]
        #[deprecated = "INSearchForPhotosIntent is deprecated. There is no replacement."]
        #[unsafe(method(peopleInPhotoOperator))]
        #[unsafe(method_family = none)]
        pub unsafe fn peopleInPhotoOperator(&self) -> INConditionalOperator;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntent")]
impl INSearchForPhotosIntent {
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
    /// Protocol to declare support for handling an INSearchForPhotosIntent. By implementing this protocol, a class can provide logic for resolving, confirming and handling the intent.
    ///
    /// The minimum requirement for an implementing class is that it should be able to handle the intent. The resolution and confirmation methods are optional. The handling method is always called last, after resolving and confirming the intent.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/intents/insearchforphotosintenthandling?language=objc)
    #[deprecated = "INSearchForPhotosIntentHandling is deprecated. There is no replacement."]
    pub unsafe trait INSearchForPhotosIntentHandling: NSObjectProtocol {
        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "INSearchForPhotosIntentResponse",
            feature = "block2"
        ))]
        /// Handling method - Execute the task represented by the INSearchForPhotosIntent that's passed in
        ///
        /// Called to actually execute the intent. The app must return a response for this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response handling block takes a INSearchForPhotosIntentResponse containing the details of the result of having executed the intent
        ///
        ///
        /// See: INSearchForPhotosIntentResponse
        #[deprecated = "INSearchForPhotosIntentHandling is deprecated. There is no replacement."]
        #[unsafe(method(handleSearchForPhotos:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn handleSearchForPhotos_completion(
            &self,
            intent: &INSearchForPhotosIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INSearchForPhotosIntentResponse>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "INSearchForPhotosIntentResponse",
            feature = "block2"
        ))]
        /// Confirmation method - Validate that this intent is ready for the next step (i.e. handling)
        ///
        /// Called prior to asking the app to handle the intent. The app should return a response object that contains additional information about the intent, which may be relevant for the system to show the user prior to handling. If unimplemented, the system will assume the intent is valid following resolution, and will assume there is no additional information relevant to this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response block contains an INSearchForPhotosIntentResponse containing additional details about the intent that may be relevant for the system to show the user prior to handling.
        ///
        ///
        /// See: INSearchForPhotosIntentResponse
        #[deprecated = "INSearchForPhotosIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(confirmSearchForPhotos:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn confirmSearchForPhotos_completion(
            &self,
            intent: &INSearchForPhotosIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INSearchForPhotosIntentResponse>)>,
        );

        #[cfg(all(
            feature = "INDateComponentsRangeResolutionResult",
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
        #[deprecated = "INSearchForPhotosIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(resolveDateCreatedForSearchForPhotos:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveDateCreatedForSearchForPhotos_withCompletion(
            &self,
            intent: &INSearchForPhotosIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INDateComponentsRangeResolutionResult>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INPlacemarkResolutionResult",
            feature = "block2"
        ))]
        #[deprecated = "INSearchForPhotosIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(resolveLocationCreatedForSearchForPhotos:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveLocationCreatedForSearchForPhotos_withCompletion(
            &self,
            intent: &INSearchForPhotosIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INPlacemarkResolutionResult>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INStringResolutionResult",
            feature = "block2"
        ))]
        #[deprecated = "INSearchForPhotosIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(resolveAlbumNameForSearchForPhotos:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveAlbumNameForSearchForPhotos_withCompletion(
            &self,
            intent: &INSearchForPhotosIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INStringResolutionResult>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INStringResolutionResult",
            feature = "block2"
        ))]
        #[deprecated]
        #[optional]
        #[unsafe(method(resolveSearchTermsForSearchForPhotos:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveSearchTermsForSearchForPhotos_withCompletion(
            &self,
            intent: &INSearchForPhotosIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<NSArray<INStringResolutionResult>>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INPersonResolutionResult",
            feature = "block2"
        ))]
        #[deprecated = "INSearchForPhotosIntentHandling is deprecated. There is no replacement."]
        #[optional]
        #[unsafe(method(resolvePeopleInPhotoForSearchForPhotos:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolvePeopleInPhotoForSearchForPhotos_withCompletion(
            &self,
            intent: &INSearchForPhotosIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<NSArray<INPersonResolutionResult>>)>,
        );
    }
);
