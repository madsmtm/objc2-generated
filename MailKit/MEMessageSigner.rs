//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Contains information about a message signer
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mailkit/memessagesigner?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEMessageSigner;
);

extern_conformance!(
    unsafe impl NSCoding for MEMessageSigner {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MEMessageSigner {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MEMessageSigner {}
);

impl MEMessageSigner {
    extern_methods!(
        #[cfg(feature = "MEEmailAddress")]
        /// Email addresses associated with the signature.
        #[unsafe(method(emailAddresses))]
        #[unsafe(method_family = none)]
        pub unsafe fn emailAddresses(&self) -> Retained<NSArray<MEEmailAddress>>;

        /// The message signers label. Shown in the message header view. For instance, "John Smith".
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        /// The context for the message signature. This might include the signing certificate. This will be passed back to the extension for
        /// either verifying the signature or if the user wishes to view signature information.
        #[unsafe(method(context))]
        #[unsafe(method_family = none)]
        pub unsafe fn context(&self) -> Retained<NSData>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MEEmailAddress")]
        #[unsafe(method(initWithEmailAddresses:signatureLabel:context:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithEmailAddresses_signatureLabel_context(
            this: Allocated<Self>,
            email_addresses: &NSArray<MEEmailAddress>,
            label: &NSString,
            context: Option<&NSData>,
        ) -> Retained<Self>;
    );
}
