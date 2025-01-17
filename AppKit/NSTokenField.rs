//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstokenfielddelegate?language=objc)
    #[cfg(all(feature = "NSControl", feature = "NSTextField"))]
    pub unsafe trait NSTokenFieldDelegate: NSTextFieldDelegate + MainThreadOnly {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// Allows the delegate to provide an array of appropriate completions for the contents of the receiver
        ///
        /// Parameter `tokenField`: The token field where editing is occurring
        ///
        /// Parameter `substring`: The partial string that is being completed
        ///
        /// Parameter `tokenIndex`: The index of the token being completed
        ///
        /// Parameter `selectedIndex`: Optionally, you can return by reference an index into the returned array that specifies which of the completions should be initially selected. If none are to be selected, return by reference `-1`.
        ///
        /// Returns: An array of strings (`NSString`) that are possible completions, or `nil` to provide no completions
        ///
        /// If the delegate does not implement this method, no completions are provided
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(tokenField:completionsForSubstring:indexOfToken:indexOfSelectedItem:)]
        unsafe fn tokenField_completionsForSubstring_indexOfToken_indexOfSelectedItem(
            &self,
            token_field: &NSTokenField,
            substring: &NSString,
            token_index: NSInteger,
            selected_index: *mut NSInteger,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(tokenField:shouldAddObjects:atIndex:)]
        unsafe fn tokenField_shouldAddObjects_atIndex(
            &self,
            token_field: &NSTokenField,
            tokens: &NSArray,
            index: NSUInteger,
        ) -> Retained<NSArray>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(tokenField:displayStringForRepresentedObject:)]
        unsafe fn tokenField_displayStringForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &AnyObject,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(tokenField:editingStringForRepresentedObject:)]
        unsafe fn tokenField_editingStringForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &AnyObject,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(tokenField:representedObjectForEditingString:)]
        unsafe fn tokenField_representedObjectForEditingString(
            &self,
            token_field: &NSTokenField,
            editing_string: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSPasteboard", feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(tokenField:writeRepresentedObjects:toPasteboard:)]
        unsafe fn tokenField_writeRepresentedObjects_toPasteboard(
            &self,
            token_field: &NSTokenField,
            objects: &NSArray,
            pboard: &NSPasteboard,
        ) -> bool;

        #[cfg(all(feature = "NSPasteboard", feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(tokenField:readFromPasteboard:)]
        unsafe fn tokenField_readFromPasteboard(
            &self,
            token_field: &NSTokenField,
            pboard: &NSPasteboard,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(feature = "NSMenu", feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(tokenField:menuForRepresentedObject:)]
        unsafe fn tokenField_menuForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &AnyObject,
        ) -> Option<Retained<NSMenu>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(tokenField:hasMenuForRepresentedObject:)]
        unsafe fn tokenField_hasMenuForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSTokenFieldCell",
            feature = "NSView"
        ))]
        #[optional]
        #[method(tokenField:styleForRepresentedObject:)]
        unsafe fn tokenField_styleForRepresentedObject(
            &self,
            token_field: &NSTokenField,
            represented_object: &AnyObject,
        ) -> NSTokenStyle;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstokenfield?language=objc)
    #[unsafe(super(NSTextField, NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    pub struct NSTokenField;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSTokenField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSTokenField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibilityNavigableStaticText for NSTokenField {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAccessibilityStaticText for NSTokenField {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSTokenField {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSTokenField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSCoding for NSTokenField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSTokenField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSObjectProtocol for NSTokenField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextContent",
    feature = "NSTextField",
    feature = "NSView"
))]
unsafe impl NSTextContent for NSTokenField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTokenField {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTextField",
    feature = "NSUserInterfaceValidation",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceValidations for NSTokenField {}

extern_methods!(
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSTokenField {
        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Retained<ProtocolObject<dyn NSTokenFieldDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTokenFieldDelegate>>,
        );

        #[cfg(feature = "NSTokenFieldCell")]
        #[method(tokenStyle)]
        pub unsafe fn tokenStyle(&self) -> NSTokenStyle;

        #[cfg(feature = "NSTokenFieldCell")]
        /// Setter for [`tokenStyle`][Self::tokenStyle].
        #[method(setTokenStyle:)]
        pub unsafe fn setTokenStyle(&self, token_style: NSTokenStyle);

        #[method(completionDelay)]
        pub unsafe fn completionDelay(&self) -> NSTimeInterval;

        /// Setter for [`completionDelay`][Self::completionDelay].
        #[method(setCompletionDelay:)]
        pub unsafe fn setCompletionDelay(&self, completion_delay: NSTimeInterval);

        #[method(defaultCompletionDelay)]
        pub unsafe fn defaultCompletionDelay(mtm: MainThreadMarker) -> NSTimeInterval;

        #[unsafe(method_family(none))]
        #[method_id(tokenizingCharacterSet)]
        pub unsafe fn tokenizingCharacterSet(&self) -> Retained<NSCharacterSet>;

        /// Setter for [`tokenizingCharacterSet`][Self::tokenizingCharacterSet].
        #[method(setTokenizingCharacterSet:)]
        pub unsafe fn setTokenizingCharacterSet(
            &self,
            tokenizing_character_set: Option<&NSCharacterSet>,
        );

        #[unsafe(method_family(none))]
        #[method_id(defaultTokenizingCharacterSet)]
        pub unsafe fn defaultTokenizingCharacterSet(
            mtm: MainThreadMarker,
        ) -> Retained<NSCharacterSet>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSTokenField {
        #[unsafe(method_family(init))]
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSTokenField {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSTextField",
        feature = "NSView"
    ))]
    unsafe impl NSTokenField {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
