//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// An object subclass representing common lens specification
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hklensspecification?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKLensSpecification;
);

unsafe impl Send for HKLensSpecification {}

unsafe impl Sync for HKLensSpecification {}

unsafe impl NSObjectProtocol for HKLensSpecification {}

extern_methods!(
    unsafe impl HKLensSpecification {
        #[cfg(feature = "HKQuantity")]
        /// The lens power to correct nearsightedness or farsightedness. (-) means nearsighted while (+) farsighted.
        #[method_id(@__retain_semantics Other sphere)]
        pub unsafe fn sphere(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        /// The lens power required to correct astigmatism. Can be positive or negative.
        #[method_id(@__retain_semantics Other cylinder)]
        pub unsafe fn cylinder(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        /// The angle along which cylindrical power should be positioned to correct astigmatism
        #[method_id(@__retain_semantics Other axis)]
        pub unsafe fn axis(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        /// The power adjustment applied to a multifocal lens to correct presbyopia
        #[method_id(@__retain_semantics Other addPower)]
        pub unsafe fn addPower(&self) -> Option<Retained<HKQuantity>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
