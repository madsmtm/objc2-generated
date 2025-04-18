//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inmessagelinkmetadata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INMessageLinkMetadata;
);

extern_conformance!(
    unsafe impl NSCoding for INMessageLinkMetadata {}
);

extern_conformance!(
    unsafe impl NSCopying for INMessageLinkMetadata {}
);

unsafe impl CopyingHelper for INMessageLinkMetadata {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INMessageLinkMetadata {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INMessageLinkMetadata {}
);

impl INMessageLinkMetadata {
    extern_methods!(
        #[unsafe(method(initWithSiteName:summary:title:openGraphType:linkURL:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSiteName_summary_title_openGraphType_linkURL(
            this: Allocated<Self>,
            site_name: Option<&NSString>,
            summary: Option<&NSString>,
            title: Option<&NSString>,
            open_graph_type: Option<&NSString>,
            link_url: Option<&NSURL>,
        ) -> Retained<Self>;

        #[unsafe(method(siteName))]
        #[unsafe(method_family = none)]
        pub unsafe fn siteName(&self) -> Option<Retained<NSString>>;

        /// Setter for [`siteName`][Self::siteName].
        #[unsafe(method(setSiteName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSiteName(&self, site_name: Option<&NSString>);

        #[unsafe(method(summary))]
        #[unsafe(method_family = none)]
        pub unsafe fn summary(&self) -> Option<Retained<NSString>>;

        /// Setter for [`summary`][Self::summary].
        #[unsafe(method(setSummary:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSummary(&self, summary: Option<&NSString>);

        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// Setter for [`title`][Self::title].
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[unsafe(method(openGraphType))]
        #[unsafe(method_family = none)]
        pub unsafe fn openGraphType(&self) -> Option<Retained<NSString>>;

        /// Setter for [`openGraphType`][Self::openGraphType].
        #[unsafe(method(setOpenGraphType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOpenGraphType(&self, open_graph_type: Option<&NSString>);

        #[unsafe(method(linkURL))]
        #[unsafe(method_family = none)]
        pub unsafe fn linkURL(&self) -> Option<Retained<NSURL>>;

        /// Setter for [`linkURL`][Self::linkURL].
        #[unsafe(method(setLinkURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLinkURL(&self, link_url: Option<&NSURL>);
    );
}

/// Methods declared on superclass `NSObject`.
impl INMessageLinkMetadata {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
