//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Sensitive Analysis Results object is returned after sensitivity analysis is performed on media
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/sensitivecontentanalysis/scsensitivityanalysis?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCSensitivityAnalysis;
);

unsafe impl Send for SCSensitivityAnalysis {}

unsafe impl Sync for SCSensitivityAnalysis {}

unsafe impl NSObjectProtocol for SCSensitivityAnalysis {}

extern_methods!(
    unsafe impl SCSensitivityAnalysis {
        /// Set to YES if analyzed media contains sensitive content
        #[method(isSensitive)]
        pub unsafe fn isSensitive(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCSensitivityAnalysis {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);