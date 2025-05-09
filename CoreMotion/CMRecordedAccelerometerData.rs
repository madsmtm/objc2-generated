//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coremotion/cmrecordedaccelerometerdata?language=objc)
    #[unsafe(super(CMAccelerometerData, CMLogItem, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
    pub struct CMRecordedAccelerometerData;
);

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
extern_conformance!(
    unsafe impl NSCoding for CMRecordedAccelerometerData {}
);

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
extern_conformance!(
    unsafe impl NSCopying for CMRecordedAccelerometerData {}
);

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
unsafe impl CopyingHelper for CMRecordedAccelerometerData {
    type Result = Self;
}

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for CMRecordedAccelerometerData {}
);

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
extern_conformance!(
    unsafe impl NSSecureCoding for CMRecordedAccelerometerData {}
);

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
impl CMRecordedAccelerometerData {
    extern_methods!(
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> u64;

        #[unsafe(method(startDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn startDate(&self) -> Retained<NSDate>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
impl CMRecordedAccelerometerData {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
