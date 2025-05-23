//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsresponder?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSResponder;
);

extern_conformance!(
    unsafe impl NSCoding for NSResponder {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSResponder {}
);

impl NSResponder {
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

        #[unsafe(method(nextResponder))]
        #[unsafe(method_family = none)]
        pub unsafe fn nextResponder(&self) -> Option<Retained<NSResponder>>;

        /// Setter for [`nextResponder`][Self::nextResponder].
        #[unsafe(method(setNextResponder:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNextResponder(&self, next_responder: Option<&NSResponder>);

        #[unsafe(method(tryToPerform:with:))]
        #[unsafe(method_family = none)]
        pub unsafe fn tryToPerform_with(&self, action: Sel, object: Option<&AnyObject>) -> bool;

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(performKeyEquivalent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

        #[cfg(feature = "NSPasteboard")]
        #[unsafe(method(validRequestorForSendType:returnType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn validRequestorForSendType_returnType(
            &self,
            send_type: Option<&NSPasteboardType>,
            return_type: Option<&NSPasteboardType>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(mouseDown:))]
        #[unsafe(method_family = none)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(rightMouseDown:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rightMouseDown(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(otherMouseDown:))]
        #[unsafe(method_family = none)]
        pub unsafe fn otherMouseDown(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(mouseUp:))]
        #[unsafe(method_family = none)]
        pub unsafe fn mouseUp(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(rightMouseUp:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rightMouseUp(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(otherMouseUp:))]
        #[unsafe(method_family = none)]
        pub unsafe fn otherMouseUp(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(mouseMoved:))]
        #[unsafe(method_family = none)]
        pub unsafe fn mouseMoved(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(mouseDragged:))]
        #[unsafe(method_family = none)]
        pub unsafe fn mouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(scrollWheel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scrollWheel(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(rightMouseDragged:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rightMouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(otherMouseDragged:))]
        #[unsafe(method_family = none)]
        pub unsafe fn otherMouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(mouseEntered:))]
        #[unsafe(method_family = none)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(mouseExited:))]
        #[unsafe(method_family = none)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(keyDown:))]
        #[unsafe(method_family = none)]
        pub unsafe fn keyDown(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(keyUp:))]
        #[unsafe(method_family = none)]
        pub unsafe fn keyUp(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(flagsChanged:))]
        #[unsafe(method_family = none)]
        pub unsafe fn flagsChanged(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(tabletPoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabletPoint(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(tabletProximity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn tabletProximity(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(cursorUpdate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn cursorUpdate(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(magnifyWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn magnifyWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(rotateWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rotateWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(swipeWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn swipeWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(beginGestureWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn beginGestureWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(endGestureWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn endGestureWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(smartMagnifyWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn smartMagnifyWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(changeModeWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn changeModeWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(touchesBeganWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn touchesBeganWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(touchesMovedWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn touchesMovedWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(touchesEndedWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn touchesEndedWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(touchesCancelledWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn touchesCancelledWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(quickLookWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn quickLookWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(pressureChangeWithEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn pressureChangeWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        /// Handle a key event that should present a context menu at the user focus.
        ///
        /// Most applications should not override this method. Instead, you should customize the context menu displayed from a keyboard event by implementing `menuForEvent:` and `selectionAnchorRect`, or `showContextMenuForSelection:`, rather than this method.
        ///
        /// You should only override this method when you do not want the system-provided default behavior for the context menu hotkey, either for a specific key combination, or for the hotkey in general. For example, if your application already provides a different behavior for control-Return (the default context menu hotkey definition), and you want to preserve that behavior, you should override this method to handle that specific key combination, and then return without calling `super`. Note that the user may customize the hotkey to a different key combination, so in this example, if any other key combination is passed to your method, you would call `super`.
        ///
        /// An implementation of this method should call `[super contextMenuKeyDown:event]` to pass the request up the responder chain. If the message reaches the application object, NSApplication's implementation of this method will send `showContextMenuForSelection:` to the responder chain. If you do not call `super`, then no further handling of the key event will be performed.
        ///
        ///
        /// Note: In some cases, `showContextMenuForSelection:` will be called without a prior call to `contextMenuKeyDown:`. This occurs when a view receives an Accessibility ShowMenu action, or when the user has created a Cocoa Text key binding to map a different key combination to the `showContextMenuForSelection:` action.
        ///
        ///
        /// Parameter `event`: The key down event that matches the system-wide context menu hotkey combination.
        ///
        ///
        /// See also: `showContextMenuForSelection:`
        #[unsafe(method(contextMenuKeyDown:))]
        #[unsafe(method_family = none)]
        pub unsafe fn contextMenuKeyDown(&self, event: &NSEvent);

        #[unsafe(method(noResponderFor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn noResponderFor(&self, event_selector: Sel);

        #[unsafe(method(acceptsFirstResponder))]
        #[unsafe(method_family = none)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;

        #[unsafe(method(becomeFirstResponder))]
        #[unsafe(method_family = none)]
        pub unsafe fn becomeFirstResponder(&self) -> bool;

        #[unsafe(method(resignFirstResponder))]
        #[unsafe(method_family = none)]
        pub unsafe fn resignFirstResponder(&self) -> bool;

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(interpretKeyEvents:))]
        #[unsafe(method_family = none)]
        pub unsafe fn interpretKeyEvents(&self, event_array: &NSArray<NSEvent>);

        #[unsafe(method(flushBufferedKeyEvents))]
        #[unsafe(method_family = none)]
        pub unsafe fn flushBufferedKeyEvents(&self);

        #[cfg(feature = "NSMenu")]
        #[unsafe(method(menu))]
        #[unsafe(method_family = none)]
        pub unsafe fn menu(&self) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        /// Setter for [`menu`][Self::menu].
        #[unsafe(method(setMenu:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[unsafe(method(showContextHelp:))]
        #[unsafe(method_family = none)]
        pub unsafe fn showContextHelp(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(helpRequested:))]
        #[unsafe(method_family = none)]
        pub unsafe fn helpRequested(&self, event_ptr: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(shouldBeTreatedAsInkEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn shouldBeTreatedAsInkEvent(&self, event: &NSEvent) -> bool;

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(wantsScrollEventsForSwipeTrackingOnAxis:))]
        #[unsafe(method_family = none)]
        pub unsafe fn wantsScrollEventsForSwipeTrackingOnAxis(
            &self,
            axis: NSEventGestureAxis,
        ) -> bool;

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(wantsForwardedScrollEventsForAxis:))]
        #[unsafe(method_family = none)]
        pub unsafe fn wantsForwardedScrollEventsForAxis(&self, axis: NSEventGestureAxis) -> bool;

        #[unsafe(method(supplementalTargetForAction:sender:))]
        #[unsafe(method_family = none)]
        pub unsafe fn supplementalTargetForAction_sender(
            &self,
            action: Sel,
            sender: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSResponder {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstandardkeybindingresponding?language=objc)
    pub unsafe trait NSStandardKeyBindingResponding:
        NSObjectProtocol + MainThreadOnly
    {
        /// *********************** Key binding entry-points ************************
        #[optional]
        #[unsafe(method(insertText:))]
        #[unsafe(method_family = none)]
        unsafe fn insertText(&self, insert_string: &AnyObject);

        #[optional]
        #[unsafe(method(doCommandBySelector:))]
        #[unsafe(method_family = none)]
        unsafe fn doCommandBySelector(&self, selector: Sel);

        /// *********************** Standard bindable commands ************************
        #[optional]
        #[unsafe(method(moveForward:))]
        #[unsafe(method_family = none)]
        unsafe fn moveForward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveRight:))]
        #[unsafe(method_family = none)]
        unsafe fn moveRight(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveBackward:))]
        #[unsafe(method_family = none)]
        unsafe fn moveBackward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveLeft:))]
        #[unsafe(method_family = none)]
        unsafe fn moveLeft(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveUp:))]
        #[unsafe(method_family = none)]
        unsafe fn moveUp(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveDown:))]
        #[unsafe(method_family = none)]
        unsafe fn moveDown(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveWordForward:))]
        #[unsafe(method_family = none)]
        unsafe fn moveWordForward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveWordBackward:))]
        #[unsafe(method_family = none)]
        unsafe fn moveWordBackward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToBeginningOfLine:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToBeginningOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToEndOfLine:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToEndOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToBeginningOfParagraph:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToBeginningOfParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToEndOfParagraph:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToEndOfParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToEndOfDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToEndOfDocument(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToBeginningOfDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToBeginningOfDocument(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(pageDown:))]
        #[unsafe(method_family = none)]
        unsafe fn pageDown(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(pageUp:))]
        #[unsafe(method_family = none)]
        unsafe fn pageUp(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(centerSelectionInVisibleArea:))]
        #[unsafe(method_family = none)]
        unsafe fn centerSelectionInVisibleArea(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveBackwardAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveBackwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveForwardAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveForwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveWordForwardAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveWordForwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveWordBackwardAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveWordBackwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveUpAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveUpAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveDownAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveDownAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToBeginningOfLineAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToBeginningOfLineAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToEndOfLineAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToEndOfLineAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToBeginningOfParagraphAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToBeginningOfParagraphAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToEndOfParagraphAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToEndOfParagraphAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToEndOfDocumentAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToEndOfDocumentAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToBeginningOfDocumentAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToBeginningOfDocumentAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(pageDownAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn pageDownAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(pageUpAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn pageUpAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveParagraphForwardAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveParagraphForwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveParagraphBackwardAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveParagraphBackwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveWordRight:))]
        #[unsafe(method_family = none)]
        unsafe fn moveWordRight(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveWordLeft:))]
        #[unsafe(method_family = none)]
        unsafe fn moveWordLeft(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveRightAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveRightAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveLeftAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveLeftAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveWordRightAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveWordRightAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveWordLeftAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveWordLeftAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToLeftEndOfLine:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToLeftEndOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToRightEndOfLine:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToRightEndOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToLeftEndOfLineAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToLeftEndOfLineAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(moveToRightEndOfLineAndModifySelection:))]
        #[unsafe(method_family = none)]
        unsafe fn moveToRightEndOfLineAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(scrollPageUp:))]
        #[unsafe(method_family = none)]
        unsafe fn scrollPageUp(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(scrollPageDown:))]
        #[unsafe(method_family = none)]
        unsafe fn scrollPageDown(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(scrollLineUp:))]
        #[unsafe(method_family = none)]
        unsafe fn scrollLineUp(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(scrollLineDown:))]
        #[unsafe(method_family = none)]
        unsafe fn scrollLineDown(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(scrollToBeginningOfDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn scrollToBeginningOfDocument(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(scrollToEndOfDocument:))]
        #[unsafe(method_family = none)]
        unsafe fn scrollToEndOfDocument(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(transpose:))]
        #[unsafe(method_family = none)]
        unsafe fn transpose(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(transposeWords:))]
        #[unsafe(method_family = none)]
        unsafe fn transposeWords(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(selectAll:))]
        #[unsafe(method_family = none)]
        unsafe fn selectAll(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(selectParagraph:))]
        #[unsafe(method_family = none)]
        unsafe fn selectParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(selectLine:))]
        #[unsafe(method_family = none)]
        unsafe fn selectLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(selectWord:))]
        #[unsafe(method_family = none)]
        unsafe fn selectWord(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(indent:))]
        #[unsafe(method_family = none)]
        unsafe fn indent(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertTab:))]
        #[unsafe(method_family = none)]
        unsafe fn insertTab(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertBacktab:))]
        #[unsafe(method_family = none)]
        unsafe fn insertBacktab(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertNewline:))]
        #[unsafe(method_family = none)]
        unsafe fn insertNewline(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertParagraphSeparator:))]
        #[unsafe(method_family = none)]
        unsafe fn insertParagraphSeparator(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertNewlineIgnoringFieldEditor:))]
        #[unsafe(method_family = none)]
        unsafe fn insertNewlineIgnoringFieldEditor(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertTabIgnoringFieldEditor:))]
        #[unsafe(method_family = none)]
        unsafe fn insertTabIgnoringFieldEditor(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertLineBreak:))]
        #[unsafe(method_family = none)]
        unsafe fn insertLineBreak(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertContainerBreak:))]
        #[unsafe(method_family = none)]
        unsafe fn insertContainerBreak(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertSingleQuoteIgnoringSubstitution:))]
        #[unsafe(method_family = none)]
        unsafe fn insertSingleQuoteIgnoringSubstitution(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(insertDoubleQuoteIgnoringSubstitution:))]
        #[unsafe(method_family = none)]
        unsafe fn insertDoubleQuoteIgnoringSubstitution(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(changeCaseOfLetter:))]
        #[unsafe(method_family = none)]
        unsafe fn changeCaseOfLetter(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(uppercaseWord:))]
        #[unsafe(method_family = none)]
        unsafe fn uppercaseWord(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(lowercaseWord:))]
        #[unsafe(method_family = none)]
        unsafe fn lowercaseWord(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(capitalizeWord:))]
        #[unsafe(method_family = none)]
        unsafe fn capitalizeWord(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteForward:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteForward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteBackward:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteBackward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteBackwardByDecomposingPreviousCharacter:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteBackwardByDecomposingPreviousCharacter(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteWordForward:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteWordForward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteWordBackward:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteWordBackward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteToBeginningOfLine:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteToBeginningOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteToEndOfLine:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteToEndOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteToBeginningOfParagraph:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteToBeginningOfParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteToEndOfParagraph:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteToEndOfParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(yank:))]
        #[unsafe(method_family = none)]
        unsafe fn yank(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(complete:))]
        #[unsafe(method_family = none)]
        unsafe fn complete(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(setMark:))]
        #[unsafe(method_family = none)]
        unsafe fn setMark(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(deleteToMark:))]
        #[unsafe(method_family = none)]
        unsafe fn deleteToMark(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(selectToMark:))]
        #[unsafe(method_family = none)]
        unsafe fn selectToMark(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(swapWithMark:))]
        #[unsafe(method_family = none)]
        unsafe fn swapWithMark(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(cancelOperation:))]
        #[unsafe(method_family = none)]
        unsafe fn cancelOperation(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(makeBaseWritingDirectionNatural:))]
        #[unsafe(method_family = none)]
        unsafe fn makeBaseWritingDirectionNatural(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(makeBaseWritingDirectionLeftToRight:))]
        #[unsafe(method_family = none)]
        unsafe fn makeBaseWritingDirectionLeftToRight(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(makeBaseWritingDirectionRightToLeft:))]
        #[unsafe(method_family = none)]
        unsafe fn makeBaseWritingDirectionRightToLeft(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(makeTextWritingDirectionNatural:))]
        #[unsafe(method_family = none)]
        unsafe fn makeTextWritingDirectionNatural(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(makeTextWritingDirectionLeftToRight:))]
        #[unsafe(method_family = none)]
        unsafe fn makeTextWritingDirectionLeftToRight(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(makeTextWritingDirectionRightToLeft:))]
        #[unsafe(method_family = none)]
        unsafe fn makeTextWritingDirectionRightToLeft(&self, sender: Option<&AnyObject>);

        #[optional]
        #[unsafe(method(quickLookPreviewItems:))]
        #[unsafe(method_family = none)]
        unsafe fn quickLookPreviewItems(&self, sender: Option<&AnyObject>);

        /// Present a context menu at the text cursor position, selection, or wherever is appropriate for your responder.
        ///
        /// NSView has a default implementation of this method. For any view that returns a non-nil value from `-menuForEvent:`, the default implementation will display that menu automatically. The NSView implementation uses the `selectionAnchorRect` method in the `NSViewContentSelectionInfo` protocol to determine the location of the selection and of the menu. The NSView implementation determines the menu to display by calling `menuForEvent:` with a right-mouse-down event that is centered on the anchor rect. If `selectionAnchorRect` is not implemented, then the NSView implementation calls `menuForEvent` with a right-mouse-down event that is centered on the view's bounds, and also displays the menu at that location.
        ///
        /// You should only override this method in a custom view if you need full control over the display of a context menu from the keyboard or Accessibility, beyond what is provided by default by NSView.
        ///
        /// If the view does not support a context menu, then you should call `[[self nextResponder] tryToPerform:_cmd with:sender]` to pass the request up the responder chain.
        ///
        ///
        /// Note: In some cases, this method will be called without a prior call to `contextMenuKeyDown:`. This occurs when a view receives an Accessibility ShowMenu action, or when the user has created a Cocoa Text key binding to map a different key combination to the `showContextMenuForSelection:` action.
        ///
        ///
        /// Parameter `sender`: The object that originated the display of the context menu.
        ///
        ///
        /// See also: `menuForEvent:`
        ///
        /// See also: `selectionAnchorRect`
        ///
        /// See also: `contextMenuKeyDown:`
        #[optional]
        #[unsafe(method(showContextMenuForSelection:))]
        #[unsafe(method_family = none)]
        unsafe fn showContextMenuForSelection(&self, sender: Option<&AnyObject>);
    }
);

/// NSStandardKeyBindingMethods.
impl NSResponder {
    extern_methods!();
}

extern_conformance!(
    unsafe impl NSStandardKeyBindingResponding for NSResponder {}
);

/// NSUndoSupport.
impl NSResponder {
    extern_methods!(
        #[unsafe(method(undoManager))]
        #[unsafe(method_family = none)]
        pub unsafe fn undoManager(&self) -> Option<Retained<NSUndoManager>>;
    );
}

/// NSControlEditingSupport.
impl NSResponder {
    extern_methods!(
        #[cfg(feature = "NSEvent")]
        #[unsafe(method(validateProposedFirstResponder:forEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn validateProposedFirstResponder_forEvent(
            &self,
            responder: &NSResponder,
            event: Option<&NSEvent>,
        ) -> bool;
    );
}

/// NSErrorPresentation.
impl NSResponder {
    extern_methods!(
        #[cfg(feature = "NSWindow")]
        #[unsafe(method(presentError:modalForWindow:delegate:didPresentSelector:contextInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo(
            &self,
            error: &NSError,
            window: &NSWindow,
            delegate: Option<&AnyObject>,
            did_present_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[unsafe(method(presentError:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentError(&self, error: &NSError) -> bool;

        #[unsafe(method(willPresentError:))]
        #[unsafe(method_family = none)]
        pub unsafe fn willPresentError(&self, error: &NSError) -> Retained<NSError>;
    );
}

/// NSTextFinderSupport.
impl NSResponder {
    extern_methods!(
        #[unsafe(method(performTextFinderAction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn performTextFinderAction(&self, sender: Option<&AnyObject>);
    );
}

/// NSWindowTabbing.
impl NSResponder {
    extern_methods!(
        #[unsafe(method(newWindowForTab:))]
        #[unsafe(method_family = none)]
        pub unsafe fn newWindowForTab(&self, sender: Option<&AnyObject>);
    );
}

/// NSWritingToolsSupport.
impl NSResponder {
    extern_methods!(
        #[unsafe(method(showWritingTools:))]
        #[unsafe(method_family = none)]
        pub unsafe fn showWritingTools(&self, sender: Option<&AnyObject>);
    );
}

/// NSDeprecated.
impl NSResponder {
    extern_methods!(
        #[deprecated = "This has always returned NO and had no effect on macOS"]
        #[unsafe(method(performMnemonic:))]
        #[unsafe(method_family = none)]
        pub unsafe fn performMnemonic(&self, string: &NSString) -> bool;
    );
}
