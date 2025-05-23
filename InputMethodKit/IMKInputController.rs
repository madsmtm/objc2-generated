//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

mod private_NSObjectIMKServerInput {
    pub trait Sealed {}
}

/// Category "IMKServerInput" on [`NSObject`].
/// Informal protocol which is used to send user events to an input method.
///
/// This is not a formal protocol by choice.  The reason for that is that there are three ways to receive events here. An input method should choose one of those ways and  implement the appropriate methods.
///
/// Here are the three approaches:
///
/// 1.  Support keybinding.
/// In this approach the system takes each keydown and trys to map the keydown to an action method that the input method has implemented.  If an action is found the system calls didCommandBySelector:client:.  If no action method is found inputText:client: is called.  An input method choosing this approach should implement
/// -(BOOL)inputText:(NSString*)string client:(id)sender;
/// -(BOOL)didCommandBySelector:(SEL)aSelector client:(id)sender;
///
/// 2. Receive all key events without the keybinding, but do "unpack" the relevant text data.
/// Key events are broken down into the Unicodes, the key code that generated them, and modifier flags.  This data is then sent to the input method's inputText:key:modifiers:client: method.  For this approach implement:
/// -(BOOL)inputText:(NSString*)string key:(NSInteger)keyCode modifiers:(NSUInteger)flags client:(id)sender;
///
/// 3. Receive events directly from the Text Services Manager as NSEvent objects.  For this approach implement:
/// -(BOOL)handleEvent:(NSEvent*)event client:(id)sender;
#[doc(alias = "IMKServerInput")]
pub unsafe trait NSObjectIMKServerInput:
    ClassType + Sized + private_NSObjectIMKServerInput::Sealed
{
    extern_methods!(
        /// Receive the Unicodes, the key code that generated them and modifier flags.
        ///
        /// Input methods implementing this method should return YES if the input was excepted, and NO if not excepted.
        #[unsafe(method(inputText:key:modifiers:client:))]
        #[unsafe(method_family = none)]
        unsafe fn inputText_key_modifiers_client(
            &self,
            string: Option<&NSString>,
            key_code: NSInteger,
            flags: NSUInteger,
            sender: Option<&AnyObject>,
        ) -> bool;

        /// Each keydown that does not map to an action method is delivered to the input method as an NSString.
        ///
        /// If the input string is not excepted the input method should return NO.  When text is accepted return YES.  Input methods should implement this method when they are using keybinding (i.e. have implemented didCommandBySelector:client:).
        #[unsafe(method(inputText:client:))]
        #[unsafe(method_family = none)]
        unsafe fn inputText_client(
            &self,
            string: Option<&NSString>,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[cfg(feature = "objc2-app-kit")]
        /// Receive all keydown and mouse events as an NSEvent (i.e. the event is simply forwarded onto the input method).
        ///
        /// Return YES if the event was handled. NO if not handled.
        #[unsafe(method(handleEvent:client:))]
        #[unsafe(method_family = none)]
        unsafe fn handleEvent_client(
            &self,
            event: Option<&NSEvent>,
            sender: Option<&AnyObject>,
        ) -> bool;

        /// Called when system binds a keyDown event to an action method.
        ///
        /// This method is designed to return YES if the command is handled and NO if the command is not handled. It is called to process a command that was generated by user action such as typing certain keys or mousing down. It is necessary for this method to return YES or NO so the  event can be passed through to the client if it is not handled.  The selector can be an action specified in the input method's dictionary of keys and actions (i.e. an action specific to the input method) or one of the NSResponder action methods such as insertNewline: or deleteBackward:.  By definition such action methods do not return a value.  For that reason if you implement this method you should test if it is appropriate to call the action method before calling it since calling the action method is agreeing to handle the command
        ///
        /// For example.  Suppose you have implemented a version of insertNewline: that terminates the conversion session and sends the fully converted text to the client.  However, if you conversion buffer is empty you want the application to receive the return key that triggered the call to insertNewline:.  In that case when didCommandBySelector:client: is called you should test your buffer before calling your implementation of insertNewline:.  If the buffer is empty you would return NO indicating that the return key should be passed on to the application.  If the buffer is not empty you would call insertNewline: and then return YES as the result of didCommandBySelector:client:.
        #[unsafe(method(didCommandBySelector:client:))]
        #[unsafe(method_family = none)]
        unsafe fn didCommandBySelector_client(
            &self,
            a_selector: Option<Sel>,
            sender: Option<&AnyObject>,
        ) -> bool;

        /// Return the current composed string.  This may be an NSString or NSAttributedString.
        ///
        /// A composed string refers to the buffer that an input method typically maintains to mirror the text contained in the active inline area.  It is called the composed string to reflect the fact that the input method composed the string by converting the characters input by the user.  In addition, using the term composed string makes it easier to differentiate between an input method's buffer and the text in the active inline area that the user sees. The returned object should be an autoreleased object.
        #[unsafe(method(composedString:))]
        #[unsafe(method_family = none)]
        unsafe fn composedString(&self, sender: Option<&AnyObject>) -> Option<Retained<AnyObject>>;

        /// Return the a string consisting of the original pre-composition unicodes.
        ///
        /// If an input method stores the original input text it should return that text here.  The return value is an attributed string so that input method's can potentially restore changes they may have made to the font, etc.  The returned object should be an autoreleased object.
        #[unsafe(method(originalString:))]
        #[unsafe(method_family = none)]
        unsafe fn originalString(
            &self,
            sender: Option<&AnyObject>,
        ) -> Option<Retained<NSAttributedString>>;

        /// Called to inform the controller that the composition should be committed.
        ///
        /// If an input method implements this method it will be called when the client wishes to end the composition session immediately. A typical response would be to call the client's insertText method and then clean up any per-session buffers and variables.  After receiving this message an input method should consider the given composition session finished.
        #[unsafe(method(commitComposition:))]
        #[unsafe(method_family = none)]
        unsafe fn commitComposition(&self, sender: Option<&AnyObject>);

        /// Called to get an array of candidates.
        ///
        /// An input method would look up its currently composed string and return a list of candidate strings that that string might map to. The returned NSArray should be an autoreleased object.
        #[unsafe(method(candidates:))]
        #[unsafe(method_family = none)]
        unsafe fn candidates(&self, sender: Option<&AnyObject>) -> Option<Retained<NSArray>>;
    );
}

impl private_NSObjectIMKServerInput::Sealed for NSObject {}
unsafe impl NSObjectIMKServerInput for NSObject {}

extern_protocol!(
    /// This protocol sets or accesses values that indicate the state of an input method.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/imkstatesetting?language=objc)
    pub unsafe trait IMKStateSetting {
        /// Activates the input method.
        #[unsafe(method(activateServer:))]
        #[unsafe(method_family = none)]
        unsafe fn activateServer(&self, sender: Option<&AnyObject>);

        /// Deactivate the input method.
        #[unsafe(method(deactivateServer:))]
        #[unsafe(method_family = none)]
        unsafe fn deactivateServer(&self, sender: Option<&AnyObject>);

        /// Return a object value whose key is tag.  The returned object should be autoreleased.
        #[unsafe(method(valueForTag:client:))]
        #[unsafe(method_family = none)]
        unsafe fn valueForTag_client(
            &self,
            tag: c_long,
            sender: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;

        /// Set the tagged value to the object specified by value.
        #[unsafe(method(setValue:forTag:client:))]
        #[unsafe(method_family = none)]
        unsafe fn setValue_forTag_client(
            &self,
            value: Option<&AnyObject>,
            tag: c_long,
            sender: Option<&AnyObject>,
        );

        /// This is called to obtain the input method's modes dictionary.
        ///
        /// Typically this is called to to build the text input menu.  By calling the input method rather than reading the modes from the info.plist the input method can dynamically modify he modes supported. The returned NSDictionary should be an autoreleased object.
        #[unsafe(method(modes:))]
        #[unsafe(method_family = none)]
        unsafe fn modes(&self, sender: Option<&AnyObject>) -> Option<Retained<NSDictionary>>;

        /// Returns an unsigned integer containing a union of event masks (see NSEvent.h)
        ///
        /// A client will check with an input method to see if an event is supported by calling the method.  The default implementation returns NSKeyDownMask.
        /// If your input method only handles key downs the InputMethodKit provides default mouse handling.  The default mousedown handling behavior is as follows: if there is an active composition area and the user clicks in the text but outside of the composition area the InputMethodKit will send your input method a commitComposition: message. Note that this will only happen for input methods that return just NSKeyDownMask (i.e. the default value) as the result of recognizedEvents.
        #[unsafe(method(recognizedEvents:))]
        #[unsafe(method_family = none)]
        unsafe fn recognizedEvents(&self, sender: Option<&AnyObject>) -> NSUInteger;

        /// Looks for a nib file containing a windowController class and a preferences utility. If found the panel is displayed.
        ///
        /// To use this method include a menu item whose action is showPreferences: in your input method's menu.  If that is done the method will be called automatically when a user selects the item in the Text Input Menu.
        /// The default implementation looks for a nib file called preferences.nib.  If found a windowController class is allocated and the nib is loaded.  You can provide a custom windowController class by naming the class in your input methods info.plist file.  To do that provide a string value that names the custom class with a key of InputMethodServerPreferencesWindowControllerClass.
        #[unsafe(method(showPreferences:))]
        #[unsafe(method_family = none)]
        unsafe fn showPreferences(&self, sender: Option<&AnyObject>);
    }
);

extern_protocol!(
    /// This protocol receives mouse events.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/imkmousehandling?language=objc)
    pub unsafe trait IMKMouseHandling {
        /// Sends a mouseDown to an input method.
        ///
        /// A mouse down event happened at given index within the sender�s text storage, at the given point, and with modifier keys identified in flags. Return YES if handled.  Set keepTracking to YES if you want to receive subsequent mouseMoved and mouseUp events.
        #[unsafe(method(mouseDownOnCharacterIndex:coordinate:withModifier:continueTracking:client:))]
        #[unsafe(method_family = none)]
        unsafe fn mouseDownOnCharacterIndex_coordinate_withModifier_continueTracking_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            keep_tracking: *mut Bool,
            sender: Option<&AnyObject>,
        ) -> bool;

        /// Sends a mouseUp to an input method.
        ///
        /// A mouse up event happened at given index within the sender text view�s text storage, at the given point, with modifier keys identified in flags. Return YES if handled.
        #[unsafe(method(mouseUpOnCharacterIndex:coordinate:withModifier:client:))]
        #[unsafe(method_family = none)]
        unsafe fn mouseUpOnCharacterIndex_coordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&AnyObject>,
        ) -> bool;

        /// Passes a mouseMoved event to the input method.
        ///
        /// A mouse moved event happened at given index within the sender text view�s text storage, at the given point, with modifier keys identified in flags. Return YES if handled.
        #[unsafe(method(mouseMovedOnCharacterIndex:coordinate:withModifier:client:))]
        #[unsafe(method_family = none)]
        unsafe fn mouseMovedOnCharacterIndex_coordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&AnyObject>,
        ) -> bool;
    }
);

extern_class!(
    /// The basic class that controls input on the input method side.
    ///
    /// IMKInputController implements fully implements the protocols defined above.  Typically a developer does not override this class, but provides a delegate object that implements the methods that developer is interested in.  The IMKInputController versions of the protocol methods check if the delegate object implements a method, and  call the delegate version if it exists.
    ///
    /// The IMKServer class which is allocated in an input method's main function creates a controller class for each input session created by a client application. Therefore for every input session there is a corresponding IMKInputController.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/imkinputcontroller?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct IMKInputController;
);

extern_conformance!(
    unsafe impl IMKMouseHandling for IMKInputController {}
);

extern_conformance!(
    unsafe impl IMKStateSetting for IMKInputController {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for IMKInputController {}
);

impl IMKInputController {
    extern_methods!(
        #[cfg(feature = "IMKServer")]
        /// Initializes the controller class setting the delegate.
        ///
        /// The inputClient parameter is the client side object that will be sending messages to the controller via the IMKServer.  The client object always conforms to the IMKTextInput protocol.
        ///
        /// Methods in the protocols that are implemented by the delegate object always include a client parameter.  Methods in the IMKInputController class do not take a client.  This is because the client is stored as an ivar in the IMKInputController.
        #[unsafe(method(initWithServer:delegate:client:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithServer_delegate_client(
            this: Allocated<Self>,
            server: Option<&IMKServer>,
            delegate: Option<&AnyObject>,
            input_client: Option<&AnyObject>,
        ) -> Option<Retained<Self>>;

        /// Called to inform the controller that the composition has changed.
        ///
        /// This method will call the protocol method composedString: to obtain the current composition. The current composition will be sent to the client by a call to the method setMarkedText:
        #[unsafe(method(updateComposition))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateComposition(&self);

        /// Stops the current composition and replaces marked text with the original text.
        ///
        /// Calls the method originalString to obtain the original text and sends that to the client via a call to IMKInputSession protocol method insertText:
        #[unsafe(method(cancelComposition))]
        #[unsafe(method_family = none)]
        pub unsafe fn cancelComposition(&self);

        /// Called to obtain a dictionary of text attributes.
        ///
        /// The default implementation returns an empty dictionary.  You should override this method if your input method wants to provide font or glyphInformation. The returned object should be an autoreleased object.
        #[unsafe(method(compositionAttributesAtRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn compositionAttributesAtRange(
            &self,
            range: NSRange,
        ) -> Option<Retained<NSMutableDictionary>>;

        /// Returns where the selection should be placed inside markedText.
        ///
        /// This method is called by updateComposition: to obtain the selection range for markedText.  The default implementation sets the selection range at the end of the marked text.
        #[unsafe(method(selectionRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectionRange(&self) -> NSRange;

        /// Returns the range in the client document that text should replace.
        ///
        /// This method is called by updateComposition to obtain the range in the client's document where markedText should be placed.  The default implementation returns an NSRange whose location and length are NSNotFound.  That indicates that the marked text should be placed at the current insertion point.  Input methods that wish to insert marked text somewhere other than at the current insertion point should override this method.
        ///
        /// An example of an input method that might override this method would be one replaced words with synonyms.  That input method would watch for certain words and when one of those words was seen it would be replaced by marked text that was a synonym of the word.
        #[unsafe(method(replacementRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn replacementRange(&self) -> NSRange;

        /// Returns a dictionary of text attributes that can be used to mark a range of an attributed string that is going to be sent to a client.
        ///
        /// This utility function can be called by input methods to mark each range (i.e. clause ) of marked text.  The style parameter should be one of the following values: kTSMHiliteSelectedRawText, kTSMHiliteConvertedText or kTSMHiliteSelectedConvertedText. See AERegistry.h for the definition of these values.
        ///
        /// The default implementation begins by calling compositionAttributesAtRange: to obtain extra attributes that an input method wants to include such as font or  glyph information.  Then the appropriate underline and underline color information is added to the attributes dictionary for the style parameter.
        ///
        /// Finally the style value is added as dictionary value.  The key for the style value is NSMarkedClauseSegment. The returned object should be an autoreleased object.
        #[unsafe(method(markForStyle:atRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn markForStyle_atRange(
            &self,
            style: NSInteger,
            range: NSRange,
        ) -> Option<Retained<NSDictionary>>;

        /// Called to pass commands that are not generated as part of the text input.
        ///
        /// The default implementation checks if the controller object (i.e. self) responds to the selector.  If that is true the message performSelector:withObject  is sent to the controller class.  The object parameter in that case is the infoDictionary.
        ///
        /// This method is called when a user selects a command item from the text input menu.  To support this an input method merely needs to provide actions for each menu item that will be placed in the menu.
        ///
        /// i.e. -(void)menuAction:(id)sender
        ///
        /// Note that the sender in this instance will be the infoDictionary.  The dictionary contains two values:
        ///
        /// kIMKCommandMenuItemName            NSMenuItem  -- the NSMenuItem that was selected
        /// kIMKCommandClientName            id
        /// <IMKTextInput
        /// , NSObject> - the current client
        #[unsafe(method(doCommandBySelector:commandDictionary:))]
        #[unsafe(method_family = none)]
        pub unsafe fn doCommandBySelector_commandDictionary(
            &self,
            a_selector: Option<Sel>,
            info_dictionary: Option<&NSDictionary>,
        );

        /// Called to inform an input method that any visible UI should be closed.
        #[unsafe(method(hidePalettes))]
        #[unsafe(method_family = none)]
        pub unsafe fn hidePalettes(&self);

        #[cfg(feature = "objc2-app-kit")]
        /// Returns a menu of input method specific commands.
        ///
        /// This method is called whenever the menu needs to be drawn so that input methods can update the menu to reflect their current state. The returned NSMenu is an autoreleased object.
        #[unsafe(method(menu))]
        #[unsafe(method_family = none)]
        pub unsafe fn menu(&self, mtm: MainThreadMarker) -> Option<Retained<NSMenu>>;

        /// Returns the input controller's delegate object. The returned id is an autoreleased object.
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(&self) -> Option<Retained<AnyObject>>;

        /// Set the input controller's delegate object.
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(&self, new_delegate: Option<&AnyObject>);

        #[cfg(feature = "IMKServer")]
        /// Return the server object which is managing this input controller. The returned IMKServer is an autoreleased object.
        #[unsafe(method(server))]
        #[unsafe(method_family = none)]
        pub unsafe fn server(&self) -> Option<Retained<IMKServer>>;

        /// Called to notify an input controller that it is about to be closed.
        #[unsafe(method(inputControllerWillClose))]
        #[unsafe(method_family = none)]
        pub unsafe fn inputControllerWillClose(&self);

        /// Called when a user has selected a annotation in a candidate window.
        ///
        /// When a candidate window is displayed and the user selects an annotation the selected annotation is sent to the input controller via this method.  The currently selected candidateString is also sent to the input method.
        #[unsafe(method(annotationSelected:forCandidate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn annotationSelected_forCandidate(
            &self,
            annotation_string: Option<&NSAttributedString>,
            candidate_string: Option<&NSAttributedString>,
        );

        /// Informs an input controller that the current candidate selection in the candidate window has changed.
        ///
        /// The candidate parameter is the candidate string that the selection changed to.  Note this method is called to indicate that the user is moving around in the candidate window.  The candidate object is not a final selection.
        #[unsafe(method(candidateSelectionChanged:))]
        #[unsafe(method_family = none)]
        pub unsafe fn candidateSelectionChanged(
            &self,
            candidate_string: Option<&NSAttributedString>,
        );

        /// Called when a new candidate has been finally selected.
        ///
        /// The candidate parameter is the users final choice from the candidate window. The candidate window will have been closed before this method is called.
        #[unsafe(method(candidateSelected:))]
        #[unsafe(method_family = none)]
        pub unsafe fn candidateSelected(&self, candidate_string: Option<&NSAttributedString>);
    );
}

/// Methods declared on superclass `NSObject`.
impl IMKInputController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
