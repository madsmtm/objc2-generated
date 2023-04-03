//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub enum MEMessageState {
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageStateReceived = 0,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageStateDraft = 1,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageStateSending = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub enum MEMessageEncryptionState {
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageEncryptionStateUnknown = 0,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageEncryptionStateNotEncrypted = 1,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageEncryptionStateEncrypted = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEMessage")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub struct MEMessage;

    #[cfg(feature = "MailKit_MEMessage")]
    unsafe impl ClassType for MEMessage {
        type Super = NSObject;
    }
);

#[cfg(feature = "MailKit_MEMessage")]
unsafe impl NSCoding for MEMessage {}

#[cfg(feature = "MailKit_MEMessage")]
unsafe impl NSObjectProtocol for MEMessage {}

#[cfg(feature = "MailKit_MEMessage")]
unsafe impl NSSecureCoding for MEMessage {}

extern_methods!(
    #[cfg(feature = "MailKit_MEMessage")]
    unsafe impl MEMessage {
        #[method(state)]
        pub unsafe fn state(&self) -> MEMessageState;

        #[method(encryptionState)]
        pub unsafe fn encryptionState(&self) -> MEMessageEncryptionState;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subject)]
        pub unsafe fn subject(&self) -> Id<NSString>;

        #[cfg(feature = "MailKit_MEEmailAddress")]
        #[method_id(@__retain_semantics Other fromAddress)]
        pub unsafe fn fromAddress(&self) -> Id<MEEmailAddress>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MailKit_MEEmailAddress"))]
        #[method_id(@__retain_semantics Other toAddresses)]
        pub unsafe fn toAddresses(&self) -> Id<NSArray<MEEmailAddress>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MailKit_MEEmailAddress"))]
        #[method_id(@__retain_semantics Other ccAddresses)]
        pub unsafe fn ccAddresses(&self) -> Id<NSArray<MEEmailAddress>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MailKit_MEEmailAddress"))]
        #[method_id(@__retain_semantics Other bccAddresses)]
        pub unsafe fn bccAddresses(&self) -> Id<NSArray<MEEmailAddress>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MailKit_MEEmailAddress"))]
        #[method_id(@__retain_semantics Other replyToAddresses)]
        pub unsafe fn replyToAddresses(&self) -> Id<NSArray<MEEmailAddress>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MailKit_MEEmailAddress"))]
        #[method_id(@__retain_semantics Other allRecipientAddresses)]
        pub unsafe fn allRecipientAddresses(&self) -> Id<NSArray<MEEmailAddress>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateSent)]
        pub unsafe fn dateSent(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateReceived)]
        pub unsafe fn dateReceived(&self) -> Option<Id<NSDate>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other headers)]
        pub unsafe fn headers(&self) -> Option<Id<NSDictionary<NSString, NSArray<NSString>>>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other rawData)]
        pub unsafe fn rawData(&self) -> Option<Id<NSData>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);