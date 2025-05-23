//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/inmessagetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INMessageType(pub NSInteger);
impl INMessageType {
    #[doc(alias = "INMessageTypeUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "INMessageTypeText")]
    pub const Text: Self = Self(1);
    #[doc(alias = "INMessageTypeAudio")]
    pub const Audio: Self = Self(2);
    #[doc(alias = "INMessageTypeDigitalTouch")]
    pub const DigitalTouch: Self = Self(3);
    #[doc(alias = "INMessageTypeHandwriting")]
    pub const Handwriting: Self = Self(4);
    #[doc(alias = "INMessageTypeSticker")]
    pub const Sticker: Self = Self(5);
    #[doc(alias = "INMessageTypeTapbackLiked")]
    #[deprecated = "Use INMessageReaction"]
    pub const TapbackLiked: Self = Self(6);
    #[doc(alias = "INMessageTypeTapbackDisliked")]
    #[deprecated = "Use INMessageReaction"]
    pub const TapbackDisliked: Self = Self(7);
    #[doc(alias = "INMessageTypeTapbackEmphasized")]
    #[deprecated = "Use INMessageReaction"]
    pub const TapbackEmphasized: Self = Self(8);
    #[doc(alias = "INMessageTypeTapbackLoved")]
    #[deprecated = "Use INMessageReaction"]
    pub const TapbackLoved: Self = Self(9);
    #[doc(alias = "INMessageTypeTapbackQuestioned")]
    #[deprecated = "Use INMessageReaction"]
    pub const TapbackQuestioned: Self = Self(10);
    #[doc(alias = "INMessageTypeTapbackLaughed")]
    #[deprecated = "Use INMessageReaction"]
    pub const TapbackLaughed: Self = Self(11);
    #[doc(alias = "INMessageTypeMediaCalendar")]
    pub const MediaCalendar: Self = Self(12);
    #[doc(alias = "INMessageTypeMediaLocation")]
    pub const MediaLocation: Self = Self(13);
    #[doc(alias = "INMessageTypeMediaAddressCard")]
    pub const MediaAddressCard: Self = Self(14);
    #[doc(alias = "INMessageTypeMediaImage")]
    pub const MediaImage: Self = Self(15);
    #[doc(alias = "INMessageTypeMediaVideo")]
    pub const MediaVideo: Self = Self(16);
    #[doc(alias = "INMessageTypeMediaPass")]
    pub const MediaPass: Self = Self(17);
    #[doc(alias = "INMessageTypeMediaAudio")]
    pub const MediaAudio: Self = Self(18);
    #[doc(alias = "INMessageTypePaymentSent")]
    pub const PaymentSent: Self = Self(19);
    #[doc(alias = "INMessageTypePaymentRequest")]
    pub const PaymentRequest: Self = Self(20);
    #[doc(alias = "INMessageTypePaymentNote")]
    pub const PaymentNote: Self = Self(21);
    #[doc(alias = "INMessageTypeAnimoji")]
    pub const Animoji: Self = Self(22);
    #[doc(alias = "INMessageTypeActivitySnippet")]
    pub const ActivitySnippet: Self = Self(23);
    #[doc(alias = "INMessageTypeFile")]
    pub const File: Self = Self(24);
    #[doc(alias = "INMessageTypeLink")]
    pub const Link: Self = Self(25);
    /// The message contains a reaction to another message.
    #[doc(alias = "INMessageTypeReaction")]
    pub const Reaction: Self = Self(26);
    /// Media content containing an animated image, such as a GIF.
    #[doc(alias = "INMessageTypeMediaAnimatedImage")]
    pub const MediaAnimatedImage: Self = Self(27);
    /// The message contains an attachment from a third party application.
    #[doc(alias = "INMessageTypeThirdPartyAttachment")]
    pub const ThirdPartyAttachment: Self = Self(28);
}

unsafe impl Encode for INMessageType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INMessageType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inmessage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INMessage;
);

extern_conformance!(
    unsafe impl NSCoding for INMessage {}
);

extern_conformance!(
    unsafe impl NSCopying for INMessage {}
);

unsafe impl CopyingHelper for INMessage {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INMessage {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INMessage {}
);

impl INMessage {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "INFile",
            feature = "INPerson",
            feature = "INSpeakableString"
        ))]
        #[unsafe(method(initWithIdentifier:conversationIdentifier:content:dateSent:sender:recipients:groupName:messageType:serviceName:attachmentFiles:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_conversationIdentifier_content_dateSent_sender_recipients_groupName_messageType_serviceName_attachmentFiles(
            this: Allocated<Self>,
            identifier: &NSString,
            conversation_identifier: Option<&NSString>,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
            group_name: Option<&INSpeakableString>,
            message_type: INMessageType,
            service_name: Option<&NSString>,
            attachment_files: Option<&NSArray<INFile>>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "INFile",
            feature = "INPerson",
            feature = "INSpeakableString"
        ))]
        #[unsafe(method(initWithIdentifier:conversationIdentifier:content:dateSent:sender:recipients:groupName:messageType:serviceName:audioMessageFile:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_conversationIdentifier_content_dateSent_sender_recipients_groupName_messageType_serviceName_audioMessageFile(
            this: Allocated<Self>,
            identifier: &NSString,
            conversation_identifier: Option<&NSString>,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
            group_name: Option<&INSpeakableString>,
            message_type: INMessageType,
            service_name: Option<&NSString>,
            audio_message_file: Option<&INFile>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "INPerson", feature = "INSpeakableString"))]
        #[unsafe(method(initWithIdentifier:conversationIdentifier:content:dateSent:sender:recipients:groupName:messageType:serviceName:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_conversationIdentifier_content_dateSent_sender_recipients_groupName_messageType_serviceName(
            this: Allocated<Self>,
            identifier: &NSString,
            conversation_identifier: Option<&NSString>,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
            group_name: Option<&INSpeakableString>,
            message_type: INMessageType,
            service_name: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "INPerson", feature = "INSpeakableString"))]
        #[unsafe(method(initWithIdentifier:conversationIdentifier:content:dateSent:sender:recipients:groupName:messageType:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_conversationIdentifier_content_dateSent_sender_recipients_groupName_messageType(
            this: Allocated<Self>,
            identifier: &NSString,
            conversation_identifier: Option<&NSString>,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
            group_name: Option<&INSpeakableString>,
            message_type: INMessageType,
        ) -> Retained<Self>;

        #[cfg(feature = "INPerson")]
        #[unsafe(method(initWithIdentifier:conversationIdentifier:content:dateSent:sender:recipients:messageType:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_conversationIdentifier_content_dateSent_sender_recipients_messageType(
            this: Allocated<Self>,
            identifier: &NSString,
            conversation_identifier: Option<&NSString>,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
            message_type: INMessageType,
        ) -> Retained<Self>;

        #[cfg(feature = "INPerson")]
        #[unsafe(method(initWithIdentifier:content:dateSent:sender:recipients:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_content_dateSent_sender_recipients(
            this: Allocated<Self>,
            identifier: &NSString,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "INMessageLinkMetadata",
            feature = "INPerson",
            feature = "INSpeakableString"
        ))]
        #[unsafe(method(initWithIdentifier:conversationIdentifier:content:dateSent:sender:recipients:groupName:serviceName:linkMetadata:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_conversationIdentifier_content_dateSent_sender_recipients_groupName_serviceName_linkMetadata(
            this: Allocated<Self>,
            identifier: &NSString,
            conversation_identifier: Option<&NSString>,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
            group_name: Option<&INSpeakableString>,
            service_name: Option<&NSString>,
            link_metadata: Option<&INMessageLinkMetadata>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "INPerson", feature = "INSpeakableString"))]
        #[unsafe(method(initWithIdentifier:conversationIdentifier:content:dateSent:sender:recipients:groupName:serviceName:messageType:numberOfAttachments:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_conversationIdentifier_content_dateSent_sender_recipients_groupName_serviceName_messageType_numberOfAttachments(
            this: Allocated<Self>,
            identifier: &NSString,
            conversation_identifier: Option<&NSString>,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
            group_name: Option<&INSpeakableString>,
            service_name: Option<&NSString>,
            message_type: INMessageType,
            number_of_attachments: Option<&NSNumber>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "INMessageReaction",
            feature = "INPerson",
            feature = "INSpeakableString"
        ))]
        #[unsafe(method(initWithIdentifier:conversationIdentifier:content:dateSent:sender:recipients:groupName:serviceName:messageType:referencedMessage:reaction:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_conversationIdentifier_content_dateSent_sender_recipients_groupName_serviceName_messageType_referencedMessage_reaction(
            this: Allocated<Self>,
            identifier: &NSString,
            conversation_identifier: Option<&NSString>,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
            group_name: Option<&INSpeakableString>,
            service_name: Option<&NSString>,
            message_type: INMessageType,
            referenced_message: Option<&INMessage>,
            reaction: Option<&INMessageReaction>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "INMessageReaction",
            feature = "INPerson",
            feature = "INSpeakableString",
            feature = "INSticker"
        ))]
        /// Creates a message that includes a reaction and references the original message for the reaction.
        ///
        /// - Parameters:
        /// - identifier: The message’s unique identifier.
        /// - conversationIdentifier: The identifier of the conversation that contains this message.
        /// - content: The text that Siri recites to the message recipient.
        /// - dateSent: The date and time the app sent the message to each recipient.
        /// - sender: The person who sent the message.
        /// - recipients: The people who received the message.
        /// - groupName: The name of the group conversation.
        /// - serviceName: The name of the service that delivers the message.
        /// - messageType: The type of content the message contains.
        /// - referencedMessage: The referenced message that received a reaction if the message object itself was a reaction.
        /// - sticker: The sticker that this message contains.
        /// - reaction: The message reaction that this message contains.
        #[unsafe(method(initWithIdentifier:conversationIdentifier:content:dateSent:sender:recipients:groupName:serviceName:messageType:referencedMessage:sticker:reaction:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_conversationIdentifier_content_dateSent_sender_recipients_groupName_serviceName_messageType_referencedMessage_sticker_reaction(
            this: Allocated<Self>,
            identifier: &NSString,
            conversation_identifier: Option<&NSString>,
            content: Option<&NSString>,
            date_sent: Option<&NSDate>,
            sender: Option<&INPerson>,
            recipients: Option<&NSArray<INPerson>>,
            group_name: Option<&INSpeakableString>,
            service_name: Option<&NSString>,
            message_type: INMessageType,
            referenced_message: Option<&INMessage>,
            sticker: Option<&INSticker>,
            reaction: Option<&INMessageReaction>,
        ) -> Retained<Self>;

        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[unsafe(method(conversationIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn conversationIdentifier(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(content))]
        #[unsafe(method_family = none)]
        pub unsafe fn content(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(dateSent))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateSent(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "INPerson")]
        #[unsafe(method(sender))]
        #[unsafe(method_family = none)]
        pub unsafe fn sender(&self) -> Option<Retained<INPerson>>;

        #[cfg(feature = "INPerson")]
        #[unsafe(method(recipients))]
        #[unsafe(method_family = none)]
        pub unsafe fn recipients(&self) -> Option<Retained<NSArray<INPerson>>>;

        #[cfg(feature = "INSpeakableString")]
        #[unsafe(method(groupName))]
        #[unsafe(method_family = none)]
        pub unsafe fn groupName(&self) -> Option<Retained<INSpeakableString>>;

        #[unsafe(method(messageType))]
        #[unsafe(method_family = none)]
        pub unsafe fn messageType(&self) -> INMessageType;

        #[unsafe(method(serviceName))]
        #[unsafe(method_family = none)]
        pub unsafe fn serviceName(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "INFile")]
        #[unsafe(method(attachmentFiles))]
        #[unsafe(method_family = none)]
        pub unsafe fn attachmentFiles(&self) -> Option<Retained<NSArray<INFile>>>;

        #[unsafe(method(numberOfAttachments))]
        #[unsafe(method_family = none)]
        pub unsafe fn numberOfAttachments(&self) -> Option<Retained<NSNumber>>;

        #[cfg(feature = "INFile")]
        #[deprecated = "Use attachmentFile instead"]
        #[unsafe(method(audioMessageFile))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioMessageFile(&self) -> Option<Retained<INFile>>;

        #[cfg(feature = "INMessageLinkMetadata")]
        #[unsafe(method(linkMetadata))]
        #[unsafe(method_family = none)]
        pub unsafe fn linkMetadata(&self) -> Option<Retained<INMessageLinkMetadata>>;

        #[cfg(feature = "INSticker")]
        /// The sticker that this message contains.
        #[unsafe(method(sticker))]
        #[unsafe(method_family = none)]
        pub unsafe fn sticker(&self) -> Option<Retained<INSticker>>;

        #[cfg(feature = "INSticker")]
        /// Setter for [`sticker`][Self::sticker].
        #[unsafe(method(setSticker:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSticker(&self, sticker: Option<&INSticker>);

        #[cfg(feature = "INMessageReaction")]
        /// The message reaction that this message contains.
        #[unsafe(method(reaction))]
        #[unsafe(method_family = none)]
        pub unsafe fn reaction(&self) -> Option<Retained<INMessageReaction>>;

        #[cfg(feature = "INMessageReaction")]
        /// Setter for [`reaction`][Self::reaction].
        #[unsafe(method(setReaction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setReaction(&self, reaction: Option<&INMessageReaction>);
    );
}

/// Methods declared on superclass `NSObject`.
impl INMessage {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
