//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextsearchdocumentidentifier?language=objc)
pub type UITextSearchDocumentIdentifier = AnyObject;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextsearchfoundtextstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextSearchFoundTextStyle(pub NSInteger);
impl UITextSearchFoundTextStyle {
    /// No style.
    #[doc(alias = "UITextSearchFoundTextStyleNormal")]
    pub const Normal: Self = Self(0);
    /// "Found" style. Used to indicate matches that have been found, but not currently highlighted.
    #[doc(alias = "UITextSearchFoundTextStyleFound")]
    pub const Found: Self = Self(1);
    /// Highlighted style, used to indicate a match that is found and currently highlighted.
    #[doc(alias = "UITextSearchFoundTextStyleHighlighted")]
    pub const Highlighted: Self = Self(2);
}

unsafe impl Encode for UITextSearchFoundTextStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextSearchFoundTextStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextsearchaggregator?language=objc)
    pub unsafe trait UITextSearchAggregator: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "UITextInput")]
        /// Returns all currently reported found ranges via
        /// `foundRange:forSearchString:.`
        #[unsafe(method(allFoundRanges))]
        #[unsafe(method_family = none)]
        unsafe fn allFoundRanges(&self) -> Retained<NSOrderedSet<UITextRange>>;

        #[cfg(feature = "UITextInput")]
        /// Call this method when a range of text is found in your document.
        ///
        ///
        /// Parameter `range`: The range of text that was found.
        ///
        /// Parameter `string`: The query string that was used to locate this range of text.
        ///
        /// Parameter `document`: (Optional) A developer-defined document identifier, later provided when this range
        /// needs to be styled.
        #[unsafe(method(foundRange:forSearchString:inDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn foundRange_forSearchString_inDocument(
            &self,
            range: &UITextRange,
            string: &NSString,
            document: Option<&UITextSearchDocumentIdentifier>,
        );

        #[cfg(feature = "UITextInput")]
        /// Call this method when a found
        /// `range`is no longer in
        /// `document`. This will cause the system find
        /// panel to update it's current state, and if the range provided is the currently highlighted range, will advance
        /// to the next found result.
        ///
        ///
        /// Parameter `range`: The range that is now invalid.
        ///
        /// Parameter `document`: (Optional) If multiple documents are used, the document identifier for the range provided.
        #[unsafe(method(invalidateFoundRange:inDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn invalidateFoundRange_inDocument(
            &self,
            range: &UITextRange,
            document: Option<&UITextSearchDocumentIdentifier>,
        );

        /// Call this method to invalidate all currently shown ranges. This will cause the system find panel to update
        /// it's current state, and may trigger a new search using `performTextSearchWithQueryString:` immediately after.
        #[unsafe(method(invalidate))]
        #[unsafe(method_family = none)]
        unsafe fn invalidate(&self);

        /// Call this method after all documents have been searched.
        #[unsafe(method(finishedSearching))]
        #[unsafe(method_family = none)]
        unsafe fn finishedSearching(&self);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextsearching?language=objc)
    pub unsafe trait UITextSearching: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "UITextInput")]
        /// Overlap from UITextInput: Returns the currently selected text range, if applicable. Nil otherwise.
        #[unsafe(method(selectedTextRange))]
        #[unsafe(method_family = none)]
        unsafe fn selectedTextRange(&self) -> Option<Retained<UITextRange>>;

        #[cfg(feature = "UITextInput")]
        /// Provide a comparison result comparing developer-provided
        /// `fromRange`to
        /// `toRange`. This is used by
        /// the system find panel to know which
        /// `UITextRange`to highlight next when the user taps the "next" or "previous"
        /// result buttons.
        ///
        ///
        /// Parameter `foundRange`: Developer-provided range to compare from.
        ///
        /// Parameter `toRange`: Developer-provided range to compare to.
        ///
        /// Parameter `document`: If multiple documents are used, a document identifier will be provided here. Ranges are only
        /// compared between other ranges of the same document.
        #[unsafe(method(compareFoundRange:toRange:inDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn compareFoundRange_toRange_inDocument(
            &self,
            found_range: &UITextRange,
            to_range: &UITextRange,
            document: Option<&UITextSearchDocumentIdentifier>,
        ) -> NSComparisonResult;

        #[cfg(feature = "UIFindSession")]
        /// Perform a text search (across all available searchable documents) using
        /// `string.`When results are found,
        /// provide the results to
        /// `aggregator.`
        ///
        /// Parameter `string`: The query string to search for
        ///
        /// Parameter `options`: Search options provided by the user.
        ///
        /// Parameter `aggregator`: When results are found, provide them to the aggregator. The aggregator is thread-safe,
        /// so you may send it messages on other threads.
        #[unsafe(method(performTextSearchWithQueryString:usingOptions:resultAggregator:))]
        #[unsafe(method_family = none)]
        unsafe fn performTextSearchWithQueryString_usingOptions_resultAggregator(
            &self,
            string: &NSString,
            options: &UITextSearchOptions,
            aggregator: &ProtocolObject<dyn UITextSearchAggregator>,
        );

        #[cfg(feature = "UITextInput")]
        /// Given a found
        /// `range`, decorate this text appropriately using the provided
        /// `style`type.
        ///
        ///
        /// Parameter `range`: The range of text to decorate.
        ///
        /// Parameter `document`: If multiple documents are used, the relevant document identifier is provided here.
        /// Otherwise nil.
        ///
        /// Parameter `style`: A style hint for how to decorate the text. This is ultimately up to the developer, but developers are
        /// encouraged to match the system's appearance (i.e., UITextView) as close as possible.
        #[unsafe(method(decorateFoundTextRange:inDocument:usingStyle:))]
        #[unsafe(method_family = none)]
        unsafe fn decorateFoundTextRange_inDocument_usingStyle(
            &self,
            range: &UITextRange,
            document: Option<&UITextSearchDocumentIdentifier>,
            style: UITextSearchFoundTextStyle,
        );

        /// Called when the current search session has changed or ended, with the expectation that all decorations
        /// applied via
        /// `decorateFoundTextRange:usingStyle:`are cleared.
        #[unsafe(method(clearAllDecoratedFoundText))]
        #[unsafe(method_family = none)]
        unsafe fn clearAllDecoratedFoundText(&self);

        /// Return YES if your searchable item also supports replacement. If this method is unimplemented, it is
        /// assumed that text replacement is not supported.
        #[optional]
        #[unsafe(method(supportsTextReplacement))]
        #[unsafe(method_family = none)]
        unsafe fn supportsTextReplacement(&self) -> bool;

        #[cfg(feature = "UITextInput")]
        /// Optionally return NO to disallow the replacement of a particular result
        /// `range`. This will disable the "replace"
        /// button in the UI. If this method is unimplemented, it is assumed that all results are replaceable.
        ///
        ///
        /// Parameter `range`: Range to replace.
        ///
        /// Parameter `document`: If multiple documents are used, the document from which range originates.
        ///
        /// Parameter `replacementText`: Text that the user intends to replace with.
        #[optional]
        #[unsafe(method(shouldReplaceFoundTextInRange:inDocument:withText:))]
        #[unsafe(method_family = none)]
        unsafe fn shouldReplaceFoundTextInRange_inDocument_withText(
            &self,
            range: &UITextRange,
            document: Option<&UITextSearchDocumentIdentifier>,
            replacement_text: &NSString,
        ) -> bool;

        #[cfg(feature = "UITextInput")]
        /// If you return YES for `-supportsTextReplacement`, this method will be called whenever the user intends to
        /// replace a range of text.
        ///
        ///
        /// Parameter `range`: The range of text requesting to be replaced.
        ///
        /// Parameter `document`: If multiple search documents are used, the relevant document identifier is provided
        /// here. Otherwise nil.
        ///
        /// Parameter `replacementText`: The replacement string.
        #[optional]
        #[unsafe(method(replaceFoundTextInRange:inDocument:withText:))]
        #[unsafe(method_family = none)]
        unsafe fn replaceFoundTextInRange_inDocument_withText(
            &self,
            range: &UITextRange,
            document: Option<&UITextSearchDocumentIdentifier>,
            replacement_text: &NSString,
        );

        #[cfg(feature = "UIFindSession")]
        /// When replacing all occurrences at once, this method is called instead of the one above.
        ///
        ///
        /// Parameter `queryString`: The search term to replace.
        ///
        /// Parameter `options`: Search options provided by the find panel UI.
        ///
        /// Parameter `replacementText`: The string to replace it with.
        #[optional]
        #[unsafe(method(replaceAllOccurrencesOfQueryString:usingOptions:withText:))]
        #[unsafe(method_family = none)]
        unsafe fn replaceAllOccurrencesOfQueryString_usingOptions_withText(
            &self,
            query_string: &NSString,
            options: &UITextSearchOptions,
            replacement_text: &NSString,
        );

        #[cfg(feature = "UITextInput")]
        /// Called when the highlighted search result is about to change to
        /// `range`.
        #[optional]
        #[unsafe(method(willHighlightFoundTextRange:inDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn willHighlightFoundTextRange_inDocument(
            &self,
            range: &UITextRange,
            document: Option<&UITextSearchDocumentIdentifier>,
        );

        #[cfg(feature = "UITextInput")]
        /// If scrolling is supported, implement this to know when the document should be scrolled to a particular
        /// search result.
        ///
        ///
        /// Parameter `range`: The text range to scroll to.
        ///
        /// Parameter `document`: (Optional) If multiple documents are used, the document identifier to scroll to.
        #[optional]
        #[unsafe(method(scrollRangeToVisible:inDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn scrollRangeToVisible_inDocument(
            &self,
            range: &UITextRange,
            document: Option<&UITextSearchDocumentIdentifier>,
        );

        /// Returns the current search document, if applicable. Nil otherwise.
        #[optional]
        #[unsafe(method(selectedTextSearchDocument))]
        #[unsafe(method_family = none)]
        unsafe fn selectedTextSearchDocument(
            &self,
        ) -> Option<Retained<UITextSearchDocumentIdentifier>>;

        /// Returns the visible ordering from `fromDocument` to `toDocument`. If your document identifiers
        /// are index paths, for example, this would yield the same result as `-[NSIndexPath compare:]`.
        #[optional]
        #[unsafe(method(compareOrderFromDocument:toDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn compareOrderFromDocument_toDocument(
            &self,
            from_document: &UITextSearchDocumentIdentifier,
            to_document: &UITextSearchDocumentIdentifier,
        ) -> NSComparisonResult;
    }
);
