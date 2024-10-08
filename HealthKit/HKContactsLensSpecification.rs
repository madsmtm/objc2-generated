//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKLensSpecification")]
    pub struct HKContactsLensSpecification;

    #[cfg(feature = "HKLensSpecification")]
    unsafe impl ClassType for HKContactsLensSpecification {
        #[inherits(NSObject)]
        type Super = HKLensSpecification;
    }
);

#[cfg(feature = "HKLensSpecification")]
unsafe impl NSCoding for HKContactsLensSpecification {}

#[cfg(feature = "HKLensSpecification")]
unsafe impl NSCopying for HKContactsLensSpecification {}

#[cfg(feature = "HKLensSpecification")]
unsafe impl CopyingHelper for HKContactsLensSpecification {
    type Result = Self;
}

#[cfg(feature = "HKLensSpecification")]
unsafe impl NSObjectProtocol for HKContactsLensSpecification {}

#[cfg(feature = "HKLensSpecification")]
unsafe impl NSSecureCoding for HKContactsLensSpecification {}

extern_methods!(
    #[cfg(feature = "HKLensSpecification")]
    unsafe impl HKContactsLensSpecification {
        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other baseCurve)]
        pub unsafe fn baseCurve(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other diameter)]
        pub unsafe fn diameter(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Init initWithSphere:cylinder:axis:addPower:baseCurve:diameter:)]
        pub unsafe fn initWithSphere_cylinder_axis_addPower_baseCurve_diameter(
            this: Allocated<Self>,
            sphere: &HKQuantity,
            cylinder: Option<&HKQuantity>,
            axis: Option<&HKQuantity>,
            add_power: Option<&HKQuantity>,
            base_curve: Option<&HKQuantity>,
            diameter: Option<&HKQuantity>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
