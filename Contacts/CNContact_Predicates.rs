//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// Predicates
    /// The predicates to match contacts against.
    ///
    ///
    /// Can only use these predicates with CNContactStore and CNContactFetchRequest.
    #[cfg(feature = "CNContact")]
    unsafe impl CNContact {
        /// To fetch contacts matching a name.
        ///
        ///
        /// The name can contain any number of words.
        #[unsafe(method_family(none))]
        #[method_id(predicateForContactsMatchingName:)]
        pub unsafe fn predicateForContactsMatchingName(name: &NSString) -> Retained<NSPredicate>;

        /// Fetch contacts matching an email address.
        ///
        ///
        /// Use this predicate to find the contact(s) which contain the specified
        /// email address. The search is not case-sensitive.
        ///
        ///
        /// Parameter `emailAddress`: The email address to search for. Do not include a scheme (e.g., "mailto:").
        #[unsafe(method_family(none))]
        #[method_id(predicateForContactsMatchingEmailAddress:)]
        pub unsafe fn predicateForContactsMatchingEmailAddress(
            email_address: &NSString,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "CNPhoneNumber")]
        /// Fetch contacts matching a phone number.
        ///
        ///
        /// If the predicate and contact differ in their use or presence of country
        /// codes, a best effort will be made to match results; however, inexact
        /// matches are not guaranteed.
        ///
        ///
        /// Parameter `phoneNumber`: A
        /// `CNPhoneNumber`representing the phone number to search for.
        /// Do not include a scheme (e.g., "tel:").
        #[unsafe(method_family(none))]
        #[method_id(predicateForContactsMatchingPhoneNumber:)]
        pub unsafe fn predicateForContactsMatchingPhoneNumber(
            phone_number: &CNPhoneNumber,
        ) -> Retained<NSPredicate>;

        /// To fetch contacts matching contact identifiers.
        #[unsafe(method_family(none))]
        #[method_id(predicateForContactsWithIdentifiers:)]
        pub unsafe fn predicateForContactsWithIdentifiers(
            identifiers: &NSArray<NSString>,
        ) -> Retained<NSPredicate>;

        #[unsafe(method_family(none))]
        #[method_id(predicateForContactsInGroupWithIdentifier:)]
        pub unsafe fn predicateForContactsInGroupWithIdentifier(
            group_identifier: &NSString,
        ) -> Retained<NSPredicate>;

        #[unsafe(method_family(none))]
        #[method_id(predicateForContactsInContainerWithIdentifier:)]
        pub unsafe fn predicateForContactsInContainerWithIdentifier(
            container_identifier: &NSString,
        ) -> Retained<NSPredicate>;
    }
);
