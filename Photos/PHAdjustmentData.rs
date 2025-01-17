//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phadjustmentdata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHAdjustmentData;
);

unsafe impl NSObjectProtocol for PHAdjustmentData {}

extern_methods!(
    unsafe impl PHAdjustmentData {
        #[unsafe(method_family(init))]
        #[method_id(initWithFormatIdentifier:formatVersion:data:)]
        pub unsafe fn initWithFormatIdentifier_formatVersion_data(
            this: Allocated<Self>,
            format_identifier: &NSString,
            format_version: &NSString,
            data: &NSData,
        ) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(formatIdentifier)]
        pub unsafe fn formatIdentifier(&self) -> Retained<NSString>;

        #[unsafe(method_family(none))]
        #[method_id(formatVersion)]
        pub unsafe fn formatVersion(&self) -> Retained<NSString>;

        #[unsafe(method_family(none))]
        #[method_id(data)]
        pub unsafe fn data(&self) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHAdjustmentData {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
