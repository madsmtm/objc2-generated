//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKQueryDescriptor;

    unsafe impl ClassType for HKQueryDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for HKQueryDescriptor {}

unsafe impl NSCopying for HKQueryDescriptor {}

unsafe impl CopyingHelper for HKQueryDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKQueryDescriptor {}

unsafe impl NSSecureCoding for HKQueryDescriptor {}

extern_methods!(
    unsafe impl HKQueryDescriptor {
        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other sampleType)]
        pub unsafe fn sampleType(&self) -> Retained<HKSampleType>;

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Retained<NSPredicate>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Init initWithSampleType:predicate:)]
        pub unsafe fn initWithSampleType_predicate(
            this: Allocated<Self>,
            sample_type: &HKSampleType,
            predicate: Option<&NSPredicate>,
        ) -> Retained<Self>;
    }
);
