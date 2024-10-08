//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static CBUUIDCharacteristicExtendedPropertiesString: &'static NSString;
}

extern "C" {
    pub static CBUUIDCharacteristicUserDescriptionString: &'static NSString;
}

extern "C" {
    pub static CBUUIDClientCharacteristicConfigurationString: &'static NSString;
}

extern "C" {
    pub static CBUUIDServerCharacteristicConfigurationString: &'static NSString;
}

extern "C" {
    pub static CBUUIDCharacteristicFormatString: &'static NSString;
}

extern "C" {
    pub static CBUUIDCharacteristicAggregateFormatString: &'static NSString;
}

extern "C" {
    pub static CBUUIDCharacteristicValidRangeString: &'static NSString;
}

extern "C" {
    pub static CBUUIDCharacteristicObservationScheduleString: &'static NSString;
}

extern "C" {
    pub static CBUUIDL2CAPPSMCharacteristicString: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CBUUID;

    unsafe impl ClassType for CBUUID {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for CBUUID {}

unsafe impl CopyingHelper for CBUUID {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CBUUID {}

extern_methods!(
    unsafe impl CBUUID {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other UUIDString)]
        pub unsafe fn UUIDString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other UUIDWithString:)]
        pub unsafe fn UUIDWithString(the_string: &NSString) -> Retained<CBUUID>;

        #[method_id(@__retain_semantics Other UUIDWithData:)]
        pub unsafe fn UUIDWithData(the_data: &NSData) -> Retained<CBUUID>;

        #[method_id(@__retain_semantics Other UUIDWithNSUUID:)]
        pub unsafe fn UUIDWithNSUUID(the_uuid: &NSUUID) -> Retained<CBUUID>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CBUUID {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
