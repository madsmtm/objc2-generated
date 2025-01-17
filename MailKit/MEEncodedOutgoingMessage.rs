//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mailkit/meencodedoutgoingmessage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEEncodedOutgoingMessage;
);

unsafe impl NSCoding for MEEncodedOutgoingMessage {}

unsafe impl NSObjectProtocol for MEEncodedOutgoingMessage {}

unsafe impl NSSecureCoding for MEEncodedOutgoingMessage {}

extern_methods!(
    unsafe impl MEEncodedOutgoingMessage {
        #[unsafe(method_family(init))]
        #[method_id(initWithRawData:isSigned:isEncrypted:)]
        pub unsafe fn initWithRawData_isSigned_isEncrypted(
            this: Allocated<Self>,
            raw_data: &NSData,
            is_signed: bool,
            is_encrypted: bool,
        ) -> Retained<Self>;

        /// The full encoded RFC822 message including headers and body.
        #[unsafe(method_family(none))]
        #[method_id(rawData)]
        pub unsafe fn rawData(&self) -> Retained<NSData>;

        /// Whether or not the encoded message is signed
        #[method(isSigned)]
        pub unsafe fn isSigned(&self) -> bool;

        /// Whether or not the encoded message is encrypted
        #[method(isEncrypted)]
        pub unsafe fn isEncrypted(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MEEncodedOutgoingMessage {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
