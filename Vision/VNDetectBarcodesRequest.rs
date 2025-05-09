//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A request that detects barcodes in an image.
    ///
    ///
    /// This request will return zero or more VNBarcodeObservation objects objects which describe the barcodes detected in an image.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vndetectbarcodesrequest?language=objc)
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNDetectBarcodesRequest;
);

#[cfg(feature = "VNRequest")]
extern_conformance!(
    unsafe impl NSCopying for VNDetectBarcodesRequest {}
);

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNDetectBarcodesRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
extern_conformance!(
    unsafe impl NSObjectProtocol for VNDetectBarcodesRequest {}
);

#[cfg(feature = "VNRequest")]
impl VNDetectBarcodesRequest {
    extern_methods!(
        #[cfg(feature = "VNTypes")]
        /// Obtain the collection of barcode symbologies currently recognized by the Vision framework.
        ///
        ///
        /// Calling this method could be a potentially expensive operation.
        ///
        ///
        /// Returns: An array of VNBarcodeSymbology objects describing the symbologies currently supported by the Vision framework.
        #[deprecated]
        #[unsafe(method(supportedSymbologies))]
        #[unsafe(method_family = none)]
        pub unsafe fn supportedSymbologies() -> Retained<NSArray<VNBarcodeSymbology>>;

        #[cfg(feature = "VNTypes")]
        /// Obtain the collection of barcode symbologies that can be recognized by the request in its current configuration.
        ///
        ///
        /// Calling this method could be a potentially expensive operation.
        ///
        ///
        /// Returns: An array of VNBarcodeSymbology objects describing the symbologies recognized by the request in its current configuration.
        #[unsafe(method(supportedSymbologiesAndReturnError:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn supportedSymbologiesAndReturnError(
            &self,
        ) -> Result<Retained<NSArray<VNBarcodeSymbology>>, Retained<NSError>>;

        #[cfg(feature = "VNTypes")]
        /// The collection of barcode symbologies that are to be detected in the image.  The default is to scan for all possible symbologies. Setting a revision on the request will reset the symbologies to all symbologies for the specified revision.
        #[unsafe(method(symbologies))]
        #[unsafe(method_family = none)]
        pub unsafe fn symbologies(&self) -> Retained<NSArray<VNBarcodeSymbology>>;

        #[cfg(feature = "VNTypes")]
        /// Setter for [`symbologies`][Self::symbologies].
        #[unsafe(method(setSymbologies:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSymbologies(&self, symbologies: &NSArray<VNBarcodeSymbology>);

        /// An option to coalesce multiple codes if applicable based on the symbology
        #[unsafe(method(coalesceCompositeSymbologies))]
        #[unsafe(method_family = none)]
        pub unsafe fn coalesceCompositeSymbologies(&self) -> bool;

        /// Setter for [`coalesceCompositeSymbologies`][Self::coalesceCompositeSymbologies].
        #[unsafe(method(setCoalesceCompositeSymbologies:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCoalesceCompositeSymbologies(&self, coalesce_composite_symbologies: bool);

        #[cfg(feature = "VNObservation")]
        /// VNBarcodeObservation results.
        #[unsafe(method(results))]
        #[unsafe(method_family = none)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNBarcodeObservation>>>;
    );
}

/// Methods declared on superclass `VNRequest`.
#[cfg(feature = "VNRequest")]
impl VNDetectBarcodesRequest {
    extern_methods!(
        /// Creates a new VNRequest with no completion handler.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Creates a new VNRequest with an optional completion handler.
        ///
        ///
        /// Parameter `completionHandler`: The block to be invoked after the request has completed its processing. The completion handler gets executed on the same dispatch queue as the request being executed.
        #[unsafe(method(initWithCompletionHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "VNRequest")]
impl VNDetectBarcodesRequest {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetectbarcodesrequestrevision1?language=objc)
pub static VNDetectBarcodesRequestRevision1: NSUInteger = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetectbarcodesrequestrevision2?language=objc)
pub static VNDetectBarcodesRequestRevision2: NSUInteger = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetectbarcodesrequestrevision3?language=objc)
pub static VNDetectBarcodesRequestRevision3: NSUInteger = 3;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetectbarcodesrequestrevision4?language=objc)
pub static VNDetectBarcodesRequestRevision4: NSUInteger = 4;
