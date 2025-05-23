//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkclusterannotation?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKClusterAnnotation;
);

#[cfg(feature = "MKAnnotation")]
extern_conformance!(
    unsafe impl MKAnnotation for MKClusterAnnotation {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MKClusterAnnotation {}
);

impl MKClusterAnnotation {
    extern_methods!(
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// Setter for [`title`][Self::title].
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[unsafe(method(subtitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        /// Setter for [`subtitle`][Self::subtitle].
        #[unsafe(method(setSubtitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "MKAnnotation")]
        #[unsafe(method(memberAnnotations))]
        #[unsafe(method_family = none)]
        pub unsafe fn memberAnnotations(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn MKAnnotation>>>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MKAnnotation")]
        #[unsafe(method(initWithMemberAnnotations:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithMemberAnnotations(
            this: Allocated<Self>,
            member_annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MKClusterAnnotation {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
