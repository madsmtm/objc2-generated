//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsformcell?language=objc)
    #[unsafe(super(NSActionCell, NSCell, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    pub struct NSFormCell;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibility for NSFormCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSFormCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCoding for NSFormCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCopying for NSFormCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl CopyingHelper for NSFormCell {
    type Result = Self;
}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSObjectProtocol for NSFormCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSFormCell {}

extern_methods!(
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSFormCell {
        #[unsafe(method_family(init))]
        #[method_id(initTextCell:)]
        pub unsafe fn initTextCell(
            this: Allocated<Self>,
            string: Option<&NSString>,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[unsafe(method_family(init))]
        #[method_id(initImageCell:)]
        pub unsafe fn initImageCell(
            this: Allocated<Self>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(titleWidth:)]
        pub unsafe fn titleWidth_(&self, size: NSSize) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(titleWidth)]
        pub unsafe fn titleWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`titleWidth`][Self::titleWidth].
        #[method(setTitleWidth:)]
        pub unsafe fn setTitleWidth(&self, title_width: CGFloat);

        #[unsafe(method_family(none))]
        #[method_id(title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// Setter for [`title`][Self::title].
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "NSFont")]
        #[unsafe(method_family(none))]
        #[method_id(titleFont)]
        pub unsafe fn titleFont(&self) -> Retained<NSFont>;

        #[cfg(feature = "NSFont")]
        /// Setter for [`titleFont`][Self::titleFont].
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, title_font: &NSFont);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[unsafe(method_family(none))]
        #[method_id(placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Retained<NSString>>;

        /// Setter for [`placeholderString`][Self::placeholderString].
        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholder_string: Option<&NSString>);

        #[unsafe(method_family(none))]
        #[method_id(placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`placeholderAttributedString`][Self::placeholderAttributedString].
        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholder_attributed_string: Option<&NSAttributedString>,
        );

        #[cfg(feature = "NSText")]
        #[method(titleAlignment)]
        pub unsafe fn titleAlignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        /// Setter for [`titleAlignment`][Self::titleAlignment].
        #[method(setTitleAlignment:)]
        pub unsafe fn setTitleAlignment(&self, title_alignment: NSTextAlignment);

        #[cfg(feature = "NSText")]
        #[method(titleBaseWritingDirection)]
        pub unsafe fn titleBaseWritingDirection(&self) -> NSWritingDirection;

        #[cfg(feature = "NSText")]
        /// Setter for [`titleBaseWritingDirection`][Self::titleBaseWritingDirection].
        #[method(setTitleBaseWritingDirection:)]
        pub unsafe fn setTitleBaseWritingDirection(
            &self,
            title_base_writing_direction: NSWritingDirection,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredTextFieldWidth)]
        pub unsafe fn preferredTextFieldWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`preferredTextFieldWidth`][Self::preferredTextFieldWidth].
        #[method(setPreferredTextFieldWidth:)]
        pub unsafe fn setPreferredTextFieldWidth(&self, preferred_text_field_width: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSFormCell {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSFormCell {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSKeyboardUI
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSFormCell {
        #[deprecated]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: Option<&NSString>);
    }
);

extern_methods!(
    /// NSFormCellAttributedStringMethods
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSFormCell {
        #[unsafe(method_family(none))]
        #[method_id(attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Retained<NSAttributedString>;

        /// Setter for [`attributedTitle`][Self::attributedTitle].
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: &NSAttributedString);
    }
);
