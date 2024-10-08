//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXDiagnostic")]
    pub struct MXAppLaunchDiagnostic;

    #[cfg(feature = "MXDiagnostic")]
    unsafe impl ClassType for MXAppLaunchDiagnostic {
        #[inherits(NSObject)]
        type Super = MXDiagnostic;
    }
);

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSCoding for MXAppLaunchDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSObjectProtocol for MXAppLaunchDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSSecureCoding for MXAppLaunchDiagnostic {}

extern_methods!(
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXAppLaunchDiagnostic {
        #[cfg(feature = "MXCallStackTree")]
        #[method_id(@__retain_semantics Other callStackTree)]
        pub unsafe fn callStackTree(&self) -> Retained<MXCallStackTree>;

        #[method_id(@__retain_semantics Other launchDuration)]
        pub unsafe fn launchDuration(&self) -> Retained<NSMeasurement<NSUnitDuration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXAppLaunchDiagnostic {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
