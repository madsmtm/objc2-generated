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
unsafe impl MKAnnotation for MKClusterAnnotation {}

unsafe impl NSObjectProtocol for MKClusterAnnotation {}

extern_methods!(
    unsafe impl MKClusterAnnotation {
        #[unsafe(method_family(none))]
        #[method_id(title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// Setter for [`title`][Self::title].
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[unsafe(method_family(none))]
        #[method_id(subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        /// Setter for [`subtitle`][Self::subtitle].
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "MKAnnotation")]
        #[unsafe(method_family(none))]
        #[method_id(memberAnnotations)]
        pub unsafe fn memberAnnotations(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn MKAnnotation>>>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MKAnnotation")]
        #[unsafe(method_family(init))]
        #[method_id(initWithMemberAnnotations:)]
        pub unsafe fn initWithMemberAnnotations(
            this: Allocated<Self>,
            member_annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKClusterAnnotation {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
