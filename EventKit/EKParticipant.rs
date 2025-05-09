//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Abstract class representing a participant attached to an event.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekparticipant?language=objc)
    #[unsafe(super(EKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EKObject")]
    pub struct EKParticipant;
);

#[cfg(feature = "EKObject")]
extern_conformance!(
    unsafe impl NSCopying for EKParticipant {}
);

#[cfg(feature = "EKObject")]
unsafe impl CopyingHelper for EKParticipant {
    type Result = Self;
}

#[cfg(feature = "EKObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for EKParticipant {}
);

#[cfg(feature = "EKObject")]
impl EKParticipant {
    extern_methods!(
        /// URL representing this participant.
        #[unsafe(method(URL))]
        #[unsafe(method_family = none)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        /// Name of this participant.
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "EKTypes")]
        /// The status of the attendee.
        ///
        /// Returns the status of the attendee as a EKParticipantStatus value.
        #[unsafe(method(participantStatus))]
        #[unsafe(method_family = none)]
        pub unsafe fn participantStatus(&self) -> EKParticipantStatus;

        #[cfg(feature = "EKTypes")]
        /// The role of the attendee.
        ///
        /// Returns the role of the attendee as a EKParticipantRole value.
        #[unsafe(method(participantRole))]
        #[unsafe(method_family = none)]
        pub unsafe fn participantRole(&self) -> EKParticipantRole;

        #[cfg(feature = "EKTypes")]
        /// The type of the attendee.
        ///
        /// Returns the type of the attendee as a EKParticipantType value.
        #[unsafe(method(participantType))]
        #[unsafe(method_family = none)]
        pub unsafe fn participantType(&self) -> EKParticipantType;

        /// A boolean indicating whether this participant represents the
        /// owner of this account.
        #[unsafe(method(isCurrentUser))]
        #[unsafe(method_family = none)]
        pub unsafe fn isCurrentUser(&self) -> bool;

        /// Returns a predicate to use with Contacts.framework to retrieve the corresponding
        /// CNContact instance.
        ///
        /// This method returns a predicate that can be used with a CNContactStore to fetch
        /// a CNContact instance for this participant, if one exists.
        #[unsafe(method(contactPredicate))]
        #[unsafe(method_family = none)]
        pub unsafe fn contactPredicate(&self) -> Retained<NSPredicate>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "EKObject")]
impl EKParticipant {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
