//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextinput?language=objc)
    pub unsafe trait NSTextInput {
        #[deprecated]
        #[method(insertText:)]
        unsafe fn insertText(&self, string: Option<&AnyObject>);

        #[deprecated]
        #[method(doCommandBySelector:)]
        unsafe fn doCommandBySelector(&self, selector: Option<Sel>);

        #[deprecated]
        #[method(setMarkedText:selectedRange:)]
        unsafe fn setMarkedText_selectedRange(
            &self,
            string: Option<&AnyObject>,
            sel_range: NSRange,
        );

        #[deprecated]
        #[method(unmarkText)]
        unsafe fn unmarkText(&self);

        #[deprecated]
        #[method(hasMarkedText)]
        unsafe fn hasMarkedText(&self) -> bool;

        #[deprecated]
        #[method(conversationIdentifier)]
        unsafe fn conversationIdentifier(&self) -> NSInteger;

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(attributedSubstringFromRange:)]
        unsafe fn attributedSubstringFromRange(
            &self,
            range: NSRange,
        ) -> Option<Retained<NSAttributedString>>;

        #[deprecated]
        #[method(markedRange)]
        unsafe fn markedRange(&self) -> NSRange;

        #[deprecated]
        #[method(selectedRange)]
        unsafe fn selectedRange(&self) -> NSRange;

        #[deprecated]
        #[method(firstRectForCharacterRange:)]
        unsafe fn firstRectForCharacterRange(&self, range: NSRange) -> NSRect;

        #[deprecated]
        #[method(characterIndexForPoint:)]
        unsafe fn characterIndexForPoint(&self, point: NSPoint) -> NSUInteger;

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(validAttributesForMarkedText)]
        unsafe fn validAttributesForMarkedText(&self) -> Option<Retained<NSArray>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsinputmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use NSTextInputContext instead"]
    pub struct NSInputManager;
);

unsafe impl NSObjectProtocol for NSInputManager {}

unsafe impl NSTextInput for NSInputManager {}

extern_methods!(
    unsafe impl NSInputManager {
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(currentInputManager)]
        pub unsafe fn currentInputManager() -> Option<Retained<NSInputManager>>;

        #[deprecated]
        #[method(cycleToNextInputLanguage:)]
        pub unsafe fn cycleToNextInputLanguage(sender: Option<&AnyObject>);

        #[deprecated]
        #[method(cycleToNextInputServerInLanguage:)]
        pub unsafe fn cycleToNextInputServerInLanguage(sender: Option<&AnyObject>);

        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(initWithName:host:)]
        pub unsafe fn initWithName_host(
            this: Allocated<Self>,
            input_server_name: Option<&NSString>,
            host_name: Option<&NSString>,
        ) -> Option<Retained<NSInputManager>>;

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(localizedInputManagerName)]
        pub unsafe fn localizedInputManagerName(&self) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method(markedTextAbandoned:)]
        pub unsafe fn markedTextAbandoned(&self, cli: Option<&AnyObject>);

        #[deprecated]
        #[method(markedTextSelectionChanged:client:)]
        pub unsafe fn markedTextSelectionChanged_client(
            &self,
            new_sel: NSRange,
            cli: Option<&AnyObject>,
        );

        #[deprecated]
        #[method(wantsToInterpretAllKeystrokes)]
        pub unsafe fn wantsToInterpretAllKeystrokes(&self) -> bool;

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(language)]
        pub unsafe fn language(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSImage")]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSInputServer")]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(server)]
        pub unsafe fn server(&self) -> Option<Retained<NSInputServer>>;

        #[deprecated]
        #[method(wantsToHandleMouseEvents)]
        pub unsafe fn wantsToHandleMouseEvents(&self) -> bool;

        #[cfg(feature = "NSEvent")]
        #[deprecated]
        #[method(handleMouseEvent:)]
        pub unsafe fn handleMouseEvent(&self, mouse_event: Option<&NSEvent>) -> bool;

        #[deprecated]
        #[method(wantsToDelayTextChangeNotifications)]
        pub unsafe fn wantsToDelayTextChangeNotifications(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSInputManager {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
