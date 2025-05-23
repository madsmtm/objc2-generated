//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextcontentmanagerenumerationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextContentManagerEnumerationOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTextContentManagerEnumerationOptions: NSUInteger {
        #[doc(alias = "NSTextContentManagerEnumerationOptionsNone")]
        const None = 0;
        #[doc(alias = "NSTextContentManagerEnumerationOptionsReverse")]
        const Reverse = 1<<0;
    }
}

unsafe impl Encode for NSTextContentManagerEnumerationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextContentManagerEnumerationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextelementprovider?language=objc)
    pub unsafe trait NSTextElementProvider: NSObjectProtocol {
        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(documentRange))]
        #[unsafe(method_family = none)]
        unsafe fn documentRange(&self) -> Retained<NSTextRange>;

        #[cfg(all(feature = "NSTextElement", feature = "NSTextRange", feature = "block2"))]
        #[unsafe(method(enumerateTextElementsFromLocation:options:usingBlock:))]
        #[unsafe(method_family = none)]
        unsafe fn enumerateTextElementsFromLocation_options_usingBlock(
            &self,
            text_location: Option<&ProtocolObject<dyn NSTextLocation>>,
            options: NSTextContentManagerEnumerationOptions,
            block: &block2::DynBlock<dyn Fn(NonNull<NSTextElement>) -> Bool + '_>,
        ) -> Option<Retained<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(all(feature = "NSTextElement", feature = "NSTextRange"))]
        #[unsafe(method(replaceContentsInRange:withTextElements:))]
        #[unsafe(method_family = none)]
        unsafe fn replaceContentsInRange_withTextElements(
            &self,
            range: &NSTextRange,
            text_elements: Option<&NSArray<NSTextElement>>,
        );

        #[cfg(feature = "block2")]
        #[unsafe(method(synchronizeToBackingStore:))]
        #[unsafe(method_family = none)]
        unsafe fn synchronizeToBackingStore(
            &self,
            completion_handler: Option<&block2::DynBlock<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "NSTextRange")]
        #[optional]
        #[unsafe(method(locationFromLocation:withOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn locationFromLocation_withOffset(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            offset: NSInteger,
        ) -> Option<Retained<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(feature = "NSTextRange")]
        #[optional]
        #[unsafe(method(offsetFromLocation:toLocation:))]
        #[unsafe(method_family = none)]
        unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &ProtocolObject<dyn NSTextLocation>,
            to: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSInteger;

        #[cfg(feature = "NSTextRange")]
        #[optional]
        #[unsafe(method(adjustedRangeFromRange:forEditingTextSelection:))]
        #[unsafe(method_family = none)]
        unsafe fn adjustedRangeFromRange_forEditingTextSelection(
            &self,
            text_range: &NSTextRange,
            for_editing_text_selection: bool,
        ) -> Option<Retained<NSTextRange>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextcontentmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextContentManager;
);

extern_conformance!(
    unsafe impl NSCoding for NSTextContentManager {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTextContentManager {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for NSTextContentManager {}
);

extern_conformance!(
    unsafe impl NSTextElementProvider for NSTextContentManager {}
);

impl NSTextContentManager {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTextContentManagerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextContentManagerDelegate>>,
        );

        #[cfg(feature = "NSTextLayoutManager")]
        #[unsafe(method(textLayoutManagers))]
        #[unsafe(method_family = none)]
        pub unsafe fn textLayoutManagers(&self) -> Retained<NSArray<NSTextLayoutManager>>;

        #[cfg(feature = "NSTextLayoutManager")]
        #[unsafe(method(addTextLayoutManager:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addTextLayoutManager(&self, text_layout_manager: &NSTextLayoutManager);

        #[cfg(feature = "NSTextLayoutManager")]
        #[unsafe(method(removeTextLayoutManager:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeTextLayoutManager(&self, text_layout_manager: &NSTextLayoutManager);

        #[cfg(feature = "NSTextLayoutManager")]
        #[unsafe(method(primaryTextLayoutManager))]
        #[unsafe(method_family = none)]
        pub unsafe fn primaryTextLayoutManager(&self) -> Option<Retained<NSTextLayoutManager>>;

        #[cfg(feature = "NSTextLayoutManager")]
        /// Setter for [`primaryTextLayoutManager`][Self::primaryTextLayoutManager].
        #[unsafe(method(setPrimaryTextLayoutManager:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPrimaryTextLayoutManager(
            &self,
            primary_text_layout_manager: Option<&NSTextLayoutManager>,
        );

        #[cfg(feature = "block2")]
        #[unsafe(method(synchronizeTextLayoutManagers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn synchronizeTextLayoutManagers(
            &self,
            completion_handler: Option<&block2::DynBlock<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(feature = "NSTextElement", feature = "NSTextRange"))]
        #[unsafe(method(textElementsForRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textElementsForRange(
            &self,
            range: &NSTextRange,
        ) -> Retained<NSArray<NSTextElement>>;

        #[unsafe(method(hasEditingTransaction))]
        #[unsafe(method_family = none)]
        pub unsafe fn hasEditingTransaction(&self) -> bool;

        #[cfg(feature = "block2")]
        #[unsafe(method(performEditingTransactionUsingBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn performEditingTransactionUsingBlock(
            &self,
            transaction: &block2::DynBlock<dyn Fn() + '_>,
        );

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(recordEditActionInRange:newTextRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordEditActionInRange_newTextRange(
            &self,
            original_text_range: &NSTextRange,
            new_text_range: &NSTextRange,
        );

        #[unsafe(method(automaticallySynchronizesTextLayoutManagers))]
        #[unsafe(method_family = none)]
        pub unsafe fn automaticallySynchronizesTextLayoutManagers(&self) -> bool;

        /// Setter for [`automaticallySynchronizesTextLayoutManagers`][Self::automaticallySynchronizesTextLayoutManagers].
        #[unsafe(method(setAutomaticallySynchronizesTextLayoutManagers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutomaticallySynchronizesTextLayoutManagers(
            &self,
            automatically_synchronizes_text_layout_managers: bool,
        );

        #[unsafe(method(automaticallySynchronizesToBackingStore))]
        #[unsafe(method_family = none)]
        pub unsafe fn automaticallySynchronizesToBackingStore(&self) -> bool;

        /// Setter for [`automaticallySynchronizesToBackingStore`][Self::automaticallySynchronizesToBackingStore].
        #[unsafe(method(setAutomaticallySynchronizesToBackingStore:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutomaticallySynchronizesToBackingStore(
            &self,
            automatically_synchronizes_to_backing_store: bool,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTextContentManager {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextcontentmanagerdelegate?language=objc)
    pub unsafe trait NSTextContentManagerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "NSTextElement", feature = "NSTextRange"))]
        #[optional]
        #[unsafe(method(textContentManager:textElementAtLocation:))]
        #[unsafe(method_family = none)]
        unsafe fn textContentManager_textElementAtLocation(
            &self,
            text_content_manager: &NSTextContentManager,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Retained<NSTextElement>>;

        #[cfg(feature = "NSTextElement")]
        #[optional]
        #[unsafe(method(textContentManager:shouldEnumerateTextElement:options:))]
        #[unsafe(method_family = none)]
        unsafe fn textContentManager_shouldEnumerateTextElement_options(
            &self,
            text_content_manager: &NSTextContentManager,
            text_element: &NSTextElement,
            options: NSTextContentManagerEnumerationOptions,
        ) -> bool;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextcontentstoragedelegate?language=objc)
    pub unsafe trait NSTextContentStorageDelegate: NSTextContentManagerDelegate {
        #[cfg(feature = "NSTextElement")]
        #[optional]
        #[unsafe(method(textContentStorage:textParagraphWithRange:))]
        #[unsafe(method_family = none)]
        unsafe fn textContentStorage_textParagraphWithRange(
            &self,
            text_content_storage: &NSTextContentStorage,
            range: NSRange,
        ) -> Option<Retained<NSTextParagraph>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextcontentstorage?language=objc)
    #[unsafe(super(NSTextContentManager, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextContentStorage;
);

extern_conformance!(
    unsafe impl NSCoding for NSTextContentStorage {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTextContentStorage {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for NSTextContentStorage {}
);

extern_conformance!(
    unsafe impl NSTextElementProvider for NSTextContentStorage {}
);

#[cfg(feature = "NSTextStorage")]
extern_conformance!(
    unsafe impl NSTextStorageObserving for NSTextContentStorage {}
);

impl NSTextContentStorage {
    extern_methods!(
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTextContentStorageDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextContentStorageDelegate>>,
        );

        #[unsafe(method(attributedString))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedString(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedString`][Self::attributedString].
        #[unsafe(method(setAttributedString:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedString(&self, attributed_string: Option<&NSAttributedString>);

        #[cfg(feature = "NSTextElement")]
        #[unsafe(method(attributedStringForTextElement:))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedStringForTextElement(
            &self,
            text_element: &NSTextElement,
        ) -> Option<Retained<NSAttributedString>>;

        #[cfg(feature = "NSTextElement")]
        #[unsafe(method(textElementForAttributedString:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textElementForAttributedString(
            &self,
            attributed_string: &NSAttributedString,
        ) -> Option<Retained<NSTextElement>>;

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(locationFromLocation:withOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn locationFromLocation_withOffset(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            offset: NSInteger,
        ) -> Option<Retained<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(offsetFromLocation:toLocation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &ProtocolObject<dyn NSTextLocation>,
            to: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSInteger;

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(adjustedRangeFromRange:forEditingTextSelection:))]
        #[unsafe(method_family = none)]
        pub unsafe fn adjustedRangeFromRange_forEditingTextSelection(
            &self,
            text_range: &NSTextRange,
            for_editing_text_selection: bool,
        ) -> Option<Retained<NSTextRange>>;
    );
}

/// Methods declared on superclass `NSTextContentManager`.
impl NSTextContentStorage {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTextContentStorage {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextcontentstorageunsupportedattributeaddednotification?language=objc)
    pub static NSTextContentStorageUnsupportedAttributeAddedNotification:
        &'static NSNotificationName;
}
