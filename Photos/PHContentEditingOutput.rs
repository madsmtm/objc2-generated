//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-uniform-type-identifiers")]
use objc2_uniform_type_identifiers::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phcontenteditingoutput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHContentEditingOutput;
);

unsafe impl NSObjectProtocol for PHContentEditingOutput {}

extern_methods!(
    unsafe impl PHContentEditingOutput {
        #[cfg(feature = "PHContentEditingInput")]
        #[unsafe(method_family(init))]
        #[method_id(initWithContentEditingInput:)]
        pub unsafe fn initWithContentEditingInput(
            this: Allocated<Self>,
            content_editing_input: &PHContentEditingInput,
        ) -> Retained<Self>;

        #[cfg(feature = "PHAdjustmentData")]
        #[unsafe(method_family(none))]
        #[method_id(adjustmentData)]
        pub unsafe fn adjustmentData(&self) -> Option<Retained<PHAdjustmentData>>;

        #[cfg(feature = "PHAdjustmentData")]
        /// Setter for [`adjustmentData`][Self::adjustmentData].
        #[method(setAdjustmentData:)]
        pub unsafe fn setAdjustmentData(&self, adjustment_data: Option<&PHAdjustmentData>);

        /// File URL where the rendered output in the default format, with adjustments baked-in, needs to be written to.
        #[unsafe(method_family(none))]
        #[method_id(renderedContentURL)]
        pub unsafe fn renderedContentURL(&self) -> Retained<NSURL>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        /// Returns the default type for the rendered content output
        #[unsafe(method_family(none))]
        #[method_id(defaultRenderedContentType)]
        pub unsafe fn defaultRenderedContentType(&self) -> Option<Retained<UTType>>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        /// Returns the supported types for the rendered content output
        #[unsafe(method_family(none))]
        #[method_id(supportedRenderedContentTypes)]
        pub unsafe fn supportedRenderedContentTypes(&self) -> Retained<NSArray<UTType>>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        /// Returns a file URL where the rendered output in the specified format, with adjustments baked-in, needs to be written to. Returns nil and provides an error identifying the reason if the format is unsupported or the requested content URL cannot be provided
        #[unsafe(method_family(none))]
        #[method_id(renderedContentURLForType:error:_)]
        pub unsafe fn renderedContentURLForType_error(
            &self,
            r#type: &UTType,
        ) -> Result<Retained<NSURL>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHContentEditingOutput {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
