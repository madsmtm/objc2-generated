//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Contact vCard support.
    ///
    ///
    /// This converts between a contact and its vCard representation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/contacts/cncontactvcardserialization?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactVCardSerialization;
);

unsafe impl NSObjectProtocol for CNContactVCardSerialization {}

extern_methods!(
    unsafe impl CNContactVCardSerialization {
        #[cfg(feature = "CNContact")]
        /// Descriptor for all contact keys required by vCard serialization
        ///
        ///
        /// This descriptor must be passed to the fetch request if the returned
        /// contacts are to be serialized with dataWithContacts:error:.
        #[unsafe(method_family(none))]
        #[method_id(descriptorForRequiredKeys)]
        pub unsafe fn descriptorForRequiredKeys() -> Retained<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(feature = "CNContact")]
        /// Serialize contacts to data.
        ///
        ///
        /// The contacts to be serialized must have been fetched with
        /// `+descriptorForRequiredKeys.`
        ///
        /// Parameter `contacts`: The contacts to serialize.
        ///
        ///
        /// Parameter `error`: An optional outparameter. If the serialization fails, this will be set.
        ///
        ///
        /// Returns: The encoded data. If the serialization fails, this will be
        /// `nil.`
        #[unsafe(method_family(none))]
        #[method_id(dataWithContacts:error:_)]
        pub unsafe fn dataWithContacts_error(
            contacts: &NSArray<CNContact>,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        #[cfg(feature = "CNContact")]
        /// Parse data into contacts.
        ///
        ///
        /// Parameter `data`: The data to parse.
        ///
        ///
        /// Parameter `error`: An optional outparameter. If the parsing fails, this will be set.
        ///
        ///
        /// Returns: The parsed contacts. If the parsing fails, this will be
        /// `nil.`
        #[unsafe(method_family(none))]
        #[method_id(contactsWithData:error:_)]
        pub unsafe fn contactsWithData_error(
            data: &NSData,
        ) -> Result<Retained<NSArray<CNContact>>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactVCardSerialization {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
