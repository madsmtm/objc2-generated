//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A base class that represents a message in a conversation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uiconversationentry?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIConversationEntry;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIConversationEntry {}
);

impl UIConversationEntry {
    extern_methods!(
        /// A string that contains the message’s text.
        #[unsafe(method(text))]
        #[unsafe(method_family = none)]
        pub unsafe fn text(&self) -> Retained<NSString>;

        /// Setter for [`text`][Self::text].
        #[unsafe(method(setText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setText(&self, text: &NSString);

        /// A string that identifies the message’s sender.
        #[unsafe(method(senderIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn senderIdentifier(&self) -> Retained<NSString>;

        /// Setter for [`senderIdentifier`][Self::senderIdentifier].
        #[unsafe(method(setSenderIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSenderIdentifier(&self, sender_identifier: &NSString);

        /// A date that notes when the sender added the message to the conversation.
        #[unsafe(method(sentDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn sentDate(&self) -> Retained<NSDate>;

        /// Setter for [`sentDate`][Self::sentDate].
        #[unsafe(method(setSentDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSentDate(&self, sent_date: &NSDate);

        /// A string that uniquely identifies this specific entry in the conversation.
        #[unsafe(method(entryIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn entryIdentifier(&self) -> Retained<NSString>;

        /// Setter for [`entryIdentifier`][Self::entryIdentifier].
        #[unsafe(method(setEntryIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEntryIdentifier(&self, entry_identifier: &NSString);

        /// An optional string that identifies another message in a conversation, when this entry is a reply to that message.
        ///
        /// When an entry is a reply to another conversation entry, this contains the identifier of the conversation entry that the person replied to.
        #[unsafe(method(replyThreadIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn replyThreadIdentifier(&self) -> Option<Retained<NSString>>;

        /// Setter for [`replyThreadIdentifier`][Self::replyThreadIdentifier].
        #[unsafe(method(setReplyThreadIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setReplyThreadIdentifier(&self, reply_thread_identifier: Option<&NSString>);

        /// A set of strings that identifies the primary recipients of the message.
        #[unsafe(method(primaryRecipientIdentifiers))]
        #[unsafe(method_family = none)]
        pub unsafe fn primaryRecipientIdentifiers(&self) -> Retained<NSSet<NSString>>;

        /// Setter for [`primaryRecipientIdentifiers`][Self::primaryRecipientIdentifiers].
        #[unsafe(method(setPrimaryRecipientIdentifiers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPrimaryRecipientIdentifiers(
            &self,
            primary_recipient_identifiers: &NSSet<NSString>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl UIConversationEntry {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
