//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextinputsourceidentifier?language=objc)
pub type NSTextInputSourceIdentifier = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextinputcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextInputContext;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTextInputContext {}
);

impl NSTextInputContext {
    extern_methods!(
        #[unsafe(method(currentInputContext))]
        #[unsafe(method_family = none)]
        pub unsafe fn currentInputContext(
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSTextInputContext>>;

        #[cfg(feature = "NSTextInputClient")]
        #[unsafe(method(initWithClient:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithClient(
            this: Allocated<Self>,
            client: &ProtocolObject<dyn NSTextInputClient>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSTextInputClient")]
        /// ** Properties ****
        #[unsafe(method(client))]
        #[unsafe(method_family = none)]
        pub unsafe fn client(&self) -> Retained<ProtocolObject<dyn NSTextInputClient>>;

        #[unsafe(method(acceptsGlyphInfo))]
        #[unsafe(method_family = none)]
        pub unsafe fn acceptsGlyphInfo(&self) -> bool;

        /// Setter for [`acceptsGlyphInfo`][Self::acceptsGlyphInfo].
        #[unsafe(method(setAcceptsGlyphInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAcceptsGlyphInfo(&self, accepts_glyph_info: bool);

        #[unsafe(method(allowedInputSourceLocales))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowedInputSourceLocales(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`allowedInputSourceLocales`][Self::allowedInputSourceLocales].
        #[unsafe(method(setAllowedInputSourceLocales:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowedInputSourceLocales(
            &self,
            allowed_input_source_locales: Option<&NSArray<NSString>>,
        );

        /// ** Activation ****
        #[unsafe(method(activate))]
        #[unsafe(method_family = none)]
        pub unsafe fn activate(&self);

        #[unsafe(method(deactivate))]
        #[unsafe(method_family = none)]
        pub unsafe fn deactivate(&self);

        #[cfg(feature = "NSEvent")]
        /// ** Input source interface ***
        #[unsafe(method(handleEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn handleEvent(&self, event: &NSEvent) -> bool;

        #[unsafe(method(discardMarkedText))]
        #[unsafe(method_family = none)]
        pub fn discardMarkedText(&self);

        #[unsafe(method(invalidateCharacterCoordinates))]
        #[unsafe(method_family = none)]
        pub fn invalidateCharacterCoordinates(&self);

        #[unsafe(method(textInputClientWillStartScrollingOrZooming))]
        #[unsafe(method_family = none)]
        pub unsafe fn textInputClientWillStartScrollingOrZooming(&self);

        #[unsafe(method(textInputClientDidEndScrollingOrZooming))]
        #[unsafe(method_family = none)]
        pub unsafe fn textInputClientDidEndScrollingOrZooming(&self);

        #[unsafe(method(textInputClientDidUpdateSelection))]
        #[unsafe(method_family = none)]
        pub unsafe fn textInputClientDidUpdateSelection(&self);

        #[unsafe(method(textInputClientDidScroll))]
        #[unsafe(method_family = none)]
        pub unsafe fn textInputClientDidScroll(&self);

        /// ** Text Input sources handling ***
        #[unsafe(method(keyboardInputSources))]
        #[unsafe(method_family = none)]
        pub unsafe fn keyboardInputSources(
            &self,
        ) -> Option<Retained<NSArray<NSTextInputSourceIdentifier>>>;

        #[unsafe(method(selectedKeyboardInputSource))]
        #[unsafe(method_family = none)]
        pub fn selectedKeyboardInputSource(&self) -> Option<Retained<NSTextInputSourceIdentifier>>;

        /// Setter for [`selectedKeyboardInputSource`][Self::selectedKeyboardInputSource].
        #[unsafe(method(setSelectedKeyboardInputSource:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedKeyboardInputSource(
            &self,
            selected_keyboard_input_source: Option<&NSTextInputSourceIdentifier>,
        );

        /// ** Text Input source attributes ***
        #[unsafe(method(localizedNameForInputSource:))]
        #[unsafe(method_family = none)]
        pub unsafe fn localizedNameForInputSource(
            input_source_identifier: &NSTextInputSourceIdentifier,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSString>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTextInputContext {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern "C" {
    /// ** Notifications ***
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextinputcontextkeyboardselectiondidchangenotification?language=objc)
    pub static NSTextInputContextKeyboardSelectionDidChangeNotification:
        &'static NSNotificationName;
}
