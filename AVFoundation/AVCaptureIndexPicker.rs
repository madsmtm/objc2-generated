//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An `AVCaptureControl` for selecting from a set of mutually exclusive values by index.
    ///
    ///
    /// `AVCaptureIndexPicker` is ideal when the set of values is provided by an indexed container like `NSArray`, `Array`, or `Sequence`. Controls may be added to an `AVCaptureSession` using `-[AVCaptureSession addControl:]`.
    ///
    /// `AVCaptureIndexPicker` uses zero-based indexing.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureindexpicker?language=objc)
    #[unsafe(super(AVCaptureControl, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVCaptureControl")]
    pub struct AVCaptureIndexPicker;
);

#[cfg(feature = "AVCaptureControl")]
unsafe impl NSObjectProtocol for AVCaptureIndexPicker {}

extern_methods!(
    #[cfg(feature = "AVCaptureControl")]
    unsafe impl AVCaptureIndexPicker {
        /// Initializes an `AVCaptureIndexPicker` to pick between `numberOfIndexes` values.
        ///
        ///
        /// Parameter `localizedTitle`: A localized string that describes the picker's `action`.
        ///
        /// Parameter `symbolName`: The name of a symbol to represent the picker.
        ///
        /// Parameter `numberOfIndexes`: The number of indexes to pick between. `numberOfIndexes` must be greater than 0, otherwise an `NSInvalidArgumentException` is thrown.
        ///
        /// Returns: An `AVCaptureIndexPicker` instance that picks between `numberOfIndexes` values.
        ///
        ///
        /// Suitable when your picked values don't need titles.
        #[method_id(@__retain_semantics Init initWithLocalizedTitle:symbolName:numberOfIndexes:)]
        pub unsafe fn initWithLocalizedTitle_symbolName_numberOfIndexes(
            this: Allocated<Self>,
            localized_title: &NSString,
            symbol_name: &NSString,
            number_of_indexes: NSInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Initializes an `AVCaptureIndexPicker` to pick between `numberOfIndexes` values.
        ///
        ///
        /// Parameter `localizedTitle`: A localized string that describes the picker's `action`.
        ///
        /// Parameter `symbolName`: The name of a symbol to represent the picker.
        ///
        /// Parameter `numberOfIndexes`: The number of indexes to pick between. `numberOfIndexes` must be greater than 0, otherwise an `NSInvalidArgumentException` is thrown.
        ///
        /// Parameter `localizedTitleTransform`: A transformation from index to localized title.
        ///
        /// Returns: An `AVCaptureIndexPicker` instance that picks between `numberOfIndexes` values with a transformation from index to localized title.
        ///
        ///
        /// Suitable when you want to provide a title for each picked value lazily.
        #[method_id(@__retain_semantics Init initWithLocalizedTitle:symbolName:numberOfIndexes:localizedTitleTransform:)]
        pub unsafe fn initWithLocalizedTitle_symbolName_numberOfIndexes_localizedTitleTransform(
            this: Allocated<Self>,
            localized_title: &NSString,
            symbol_name: &NSString,
            number_of_indexes: NSInteger,
            localized_title_transform: &block2::Block<dyn Fn(NSInteger) -> NonNull<NSString> + '_>,
        ) -> Retained<Self>;

        /// Initializes an `AVCaptureIndexPicker` to pick between `localizedIndexTitles.count` values.
        ///
        ///
        /// Parameter `localizedTitle`: A localized string that describes the picker's `action`.
        ///
        /// Parameter `symbolName`: The name of a symbol to represent the picker.
        ///
        /// Parameter `localizedIndexTitles`: The titles to use for each index. `localizedIndexTitles` must be greater than 0, otherwise an `NSInvalidArgumentException` is thrown.
        ///
        /// Returns: An `AVCaptureIndexPicker` instance that picks between `localizedIndexTitles.count` values.
        ///
        ///
        /// Suitable when you already have an array containing a title for each picked value.
        #[method_id(@__retain_semantics Init initWithLocalizedTitle:symbolName:localizedIndexTitles:)]
        pub unsafe fn initWithLocalizedTitle_symbolName_localizedIndexTitles(
            this: Allocated<Self>,
            localized_title: &NSString,
            symbol_name: &NSString,
            localized_index_titles: &NSArray<NSString>,
        ) -> Retained<Self>;

        /// The currently selected index.
        ///
        ///
        /// Because the camera system may be independent from the main thread or `
        /// MainThreadOnly`, `selectedIndex` must be changed on `actionQueue` – the queue provided to `setActionQueue:action:`. The default value is 0. An index may only be set if it is greater than 0 or less than `numberOfIndexes`, otherwise an `NSInvalidArgumentException` is thrown.
        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        /// Setter for [`selectedIndex`][Self::selectedIndex].
        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSInteger);

        /// A localized string that describes the picker's `action`.
        #[method_id(@__retain_semantics Other localizedTitle)]
        pub unsafe fn localizedTitle(&self) -> Retained<NSString>;

        /// The name of a symbol to represent the picker.
        #[method_id(@__retain_semantics Other symbolName)]
        pub unsafe fn symbolName(&self) -> Retained<NSString>;

        /// The number of indexes to pick between.
        #[method(numberOfIndexes)]
        pub unsafe fn numberOfIndexes(&self) -> NSInteger;

        /// The titles used for each index.
        #[method_id(@__retain_semantics Other localizedIndexTitles)]
        pub unsafe fn localizedIndexTitles(&self) -> Retained<NSArray<NSString>>;

        /// A string that identifies the picker.
        #[method_id(@__retain_semantics Other accessibilityIdentifier)]
        pub unsafe fn accessibilityIdentifier(&self) -> Option<Retained<NSString>>;

        /// Setter for [`accessibilityIdentifier`][Self::accessibilityIdentifier].
        #[method(setAccessibilityIdentifier:)]
        pub unsafe fn setAccessibilityIdentifier(
            &self,
            accessibility_identifier: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `AVCaptureControl`
    #[cfg(feature = "AVCaptureControl")]
    unsafe impl AVCaptureIndexPicker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);