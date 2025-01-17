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
unsafe impl NSCoding for CMRecordedAccelerometerData {}

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
unsafe impl NSCopying for CMRecordedAccelerometerData {}

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
unsafe impl CopyingHelper for CMRecordedAccelerometerData {
    type Result = Self;
}

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
unsafe impl NSObjectProtocol for CMRecordedAccelerometerData {}

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
unsafe impl NSSecureCoding for CMRecordedAccelerometerData {}

extern_methods!(
    #[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
    unsafe impl CMRecordedAccelerometerData {
        #[method(identifier)]
        pub unsafe fn identifier(&self) -> u64;

        #[unsafe(method_family(none))]
        #[method_id(startDate)]
        pub unsafe fn startDate(&self) -> Retained<NSDate>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
    unsafe impl CMRecordedAccelerometerData {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
