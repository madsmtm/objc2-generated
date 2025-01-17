//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkdirectionshandler?language=objc)
#[cfg(all(feature = "MKDirectionsResponse", feature = "block2"))]
pub type MKDirectionsHandler = *mut block2::Block<dyn Fn(*mut MKDirectionsResponse, *mut NSError)>;

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mketahandler?language=objc)
#[cfg(all(feature = "MKDirectionsResponse", feature = "block2"))]
pub type MKETAHandler = *mut block2::Block<dyn Fn(*mut MKETAResponse, *mut NSError)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkdirections?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKDirections;
);

unsafe impl NSObjectProtocol for MKDirections {}

extern_methods!(
    unsafe impl MKDirections {
        #[cfg(feature = "MKDirectionsRequest")]
        #[unsafe(method_family(init))]
        #[method_id(initWithRequest:)]
        pub unsafe fn initWithRequest(
            this: Allocated<Self>,
            request: &MKDirectionsRequest,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKDirectionsResponse", feature = "block2"))]
        #[method(calculateDirectionsWithCompletionHandler:)]
        pub unsafe fn calculateDirectionsWithCompletionHandler(
            &self,
            completion_handler: MKDirectionsHandler,
        );

        #[cfg(all(feature = "MKDirectionsResponse", feature = "block2"))]
        #[method(calculateETAWithCompletionHandler:)]
        pub unsafe fn calculateETAWithCompletionHandler(&self, completion_handler: MKETAHandler);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isCalculating)]
        pub unsafe fn isCalculating(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKDirections {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
