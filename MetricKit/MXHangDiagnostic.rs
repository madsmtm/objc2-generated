//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An MXDiagnostic subclass that encapsulates hang diagnostic reports.
    ///
    /// Applications are considered to be "hanging" when they are unable to handle user input responsively.
    ///
    /// This generally occurs when your applications main thread is blocked.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxhangdiagnostic?language=objc)
    #[unsafe(super(MXDiagnostic, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXDiagnostic")]
    pub struct MXHangDiagnostic;
);

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSCoding for MXHangDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSObjectProtocol for MXHangDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSSecureCoding for MXHangDiagnostic {}

extern_methods!(
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXHangDiagnostic {
        #[cfg(feature = "MXCallStackTree")]
        /// The application call stack tree associated with the hang.
        #[unsafe(method_family(none))]
        #[method_id(callStackTree)]
        pub unsafe fn callStackTree(&self) -> Retained<MXCallStackTree>;

        /// Total hang duration for this diagnostic.
        ///
        /// Dimensioned as NSUnitDuration.
        #[unsafe(method_family(none))]
        #[method_id(hangDuration)]
        pub unsafe fn hangDuration(&self) -> Retained<NSMeasurement<NSUnitDuration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXHangDiagnostic {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
