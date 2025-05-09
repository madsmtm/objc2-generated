//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An object that describes a reaction to a message.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/intents/inmessagereaction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INMessageReaction;
);

extern_conformance!(
    unsafe impl NSCoding for INMessageReaction {}
);

extern_conformance!(
    unsafe impl NSCopying for INMessageReaction {}
);

unsafe impl CopyingHelper for INMessageReaction {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INMessageReaction {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INMessageReaction {}
);

impl INMessageReaction {
    extern_methods!(
        #[cfg(feature = "INMessageReactionType")]
        /// Creates an INMessageReaction
        ///
        /// - Parameters:
        /// - reactionType: The type of message reaction.
        /// - reactionDescription: Text that describes the reaction.
        /// - emoji: The single emoji character used for an emoji reaction.
        #[unsafe(method(initWithReactionType:reactionDescription:emoji:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithReactionType_reactionDescription_emoji(
            this: Allocated<Self>,
            reaction_type: INMessageReactionType,
            reaction_description: Option<&NSString>,
            emoji: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "INMessageReactionType")]
        /// The type of reaction.
        #[unsafe(method(reactionType))]
        #[unsafe(method_family = none)]
        pub unsafe fn reactionType(&self) -> INMessageReactionType;

        #[unsafe(method(reactionDescription))]
        #[unsafe(method_family = none)]
        pub unsafe fn reactionDescription(&self) -> Option<Retained<NSString>>;

        /// The emoji used to react.
        #[unsafe(method(emoji))]
        #[unsafe(method_family = none)]
        pub unsafe fn emoji(&self) -> Option<Retained<NSString>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl INMessageReaction {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
