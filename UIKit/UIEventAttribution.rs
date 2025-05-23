//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uieventattribution?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIEventAttribution;
);

extern_conformance!(
    unsafe impl NSCopying for UIEventAttribution {}
);

unsafe impl CopyingHelper for UIEventAttribution {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UIEventAttribution {}
);

impl UIEventAttribution {
    extern_methods!(
        /// An identifier that is associated with the source of the attribution. For example, you may choose to use this as a campaign identifier to measure the effectiveness of different advertisement campaigns.
        ///
        /// This field corresponds to `source_id` in the subsequent attribution report.
        #[unsafe(method(sourceIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceIdentifier(&self) -> u8;

        /// The destination URL of an attribution. For example, the link opened when an advertisement is tapped.
        ///
        /// This field corresponds to `attributed_on_site` in the subsequent attribution report.
        #[unsafe(method(destinationURL))]
        #[unsafe(method_family = none)]
        pub unsafe fn destinationURL(&self) -> Retained<NSURL>;

        /// The URL to which the attribution report will be sent. You do not provide this field at creation time. Instead, you must define the URL
        /// as a string in your app's Info.plist under the `NSAdvertisingAttributionReportEndpoint` key.
        ///
        /// This field corresponds to `source_site` in the subsequent attribution report.
        #[unsafe(method(reportEndpoint))]
        #[unsafe(method_family = none)]
        pub unsafe fn reportEndpoint(&self) -> Option<Retained<NSURL>>;

        /// A high-level description of the source of the attribution. For example, this could be a description of the content of an advertisement a user taps on.
        ///
        /// The system may truncate this field if it is too long.
        #[unsafe(method(sourceDescription))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceDescription(&self) -> Retained<NSString>;

        /// The name of the party that purchased the content to be attributed. For example, this could be the name of the party that purchased the placement of an advertisement.
        ///
        /// The system may truncate this field if it is too long.
        #[unsafe(method(purchaser))]
        #[unsafe(method_family = none)]
        pub unsafe fn purchaser(&self) -> Retained<NSString>;

        /// Create a `UIEventAttribution` object.
        ///
        ///
        /// Parameter `sourceIdentifier`: An identifier associated with the attribution.
        ///
        /// Parameter `destinationURL`: The destination URL of the attribution.
        ///
        /// Parameter `sourceDescription`: A high-level description of the content to be attributed. Pass in an empty string if no description is available.
        ///
        /// Parameter `purchaser`: The name of the party that purchased the content to be attributed. Pass in an empty string if no name is available.
        ///
        ///
        /// Returns: An instance of `UIEventAttribution` with the specified values for each field.
        ///
        /// The `sourceDescription` and `purchaser` fields may be truncated by the system if they are too long.
        #[unsafe(method(initWithSourceIdentifier:destinationURL:sourceDescription:purchaser:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSourceIdentifier_destinationURL_sourceDescription_purchaser(
            this: Allocated<Self>,
            source_identifier: u8,
            destination_url: &NSURL,
            source_description: &NSString,
            purchaser: &NSString,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
