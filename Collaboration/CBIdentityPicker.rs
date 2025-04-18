//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A `CBIdentityPicker` object allows a user to select identities—for
    /// example, user or group objects—that it wants one or more services or
    /// shared resources to have access to. An identity picker can be displayed
    /// either as an application-modal dialog or as a sheet attached to a
    /// document window. An identity picker returns the selected records to be
    /// added to access control lists using Collaboration. If a selected record
    /// is not a user or group identity, then an identity picker prompts the
    /// user for additional information—such as a password—to promote that
    /// record to a sharing account.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/collaboration/cbidentitypicker?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CBIdentityPicker;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CBIdentityPicker {}
);

impl CBIdentityPicker {
    extern_methods!(
        /// The title of the identity picker.
        ///
        /// The value of this property is the title text that appears at the top of the
        /// panel. By default, the title is "Select a person to share with:".
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// Setter for [`title`][Self::title].
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        /// A Boolean value indicating whether the user is allowed to select multiple
        /// identities.
        ///
        /// The value of this property is
        /// <doc
        /// ://com.apple.documentation/documentation/objectivec/yes> if the user can
        /// select multiple records; otherwise,
        /// <doc
        /// ://com.apple.documentation/documentation/objectivec/no>. The default
        /// value is
        /// <doc
        /// ://com.apple.documentation/documentation/objectivec/no>.
        #[unsafe(method(allowsMultipleSelection))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;

        /// Setter for [`allowsMultipleSelection`][Self::allowsMultipleSelection].
        #[unsafe(method(setAllowsMultipleSelection:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowsMultipleSelection(&self, allows_multiple_selection: bool);

        /// Runs the receiver as an application-modal dialog.
        ///
        /// The receiver may create identities for selected records if necessary.
        ///
        /// - Returns: `NSOKButton` if the user selected OK; otherwise,
        /// `NSCancelButton`.
        #[unsafe(method(runModal))]
        #[unsafe(method_family = none)]
        pub unsafe fn runModal(&self) -> NSInteger;

        #[cfg(feature = "objc2-app-kit")]
        /// Runs the receiver modally as a sheet attached to a specified window.
        ///
        /// The `didEndSelector` parameter is a selector that takes three arguments. The
        /// corresponding method should have a declaration modeled on the following
        /// example:
        ///
        /// ```swift
        /// - (void)identityPickerDidEnd:(CBIdentityPicker *)identityPicker returnCode:(NSInteger)returnCode contextInfo:(void *)contextInfo;
        /// ```
        ///
        /// where the `identityPicker` argument is the identity picker object, the
        /// `returnCode` argument is the button the user clicked, and `contextInfo` is
        /// the same `contextInfo` argument that was passed in the original message.
        ///
        /// - Parameters:
        /// - window: The parent window for the sheet.
        ///
        /// - delegate: The delegate for the modal session.
        ///
        /// - didEndSelector: A message sent to the delegate after the user responds but
        /// before the sheet is dismissed.
        ///
        /// - contextInfo: Contextual data passed to the delegate in the
        /// `didEndSelector` message.
        #[deprecated = "Use runModalForWindow:completionHandler: instead."]
        #[unsafe(method(runModalForWindow:modalDelegate:didEndSelector:contextInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn runModalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            window: &NSWindow,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(all(feature = "block2", feature = "objc2-app-kit"))]
        /// Runs the identity picker modally as a sheet attached to a specified window.
        ///
        /// - Parameters:
        /// - window: The parent window for the sheet.
        ///
        /// - completionHandler: The handler to run after the return value is known, but
        /// before the sheet is dismissed.
        #[unsafe(method(runModalForWindow:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn runModalForWindow_completionHandler(
            &self,
            window: &NSWindow,
            completion_handler: Option<&block2::DynBlock<dyn Fn(NSModalResponse)>>,
        );

        #[cfg(feature = "CBIdentity")]
        /// The array of identities (represented by `CBIdentity` objects) selected using
        /// the identity picker.
        #[unsafe(method(identities))]
        #[unsafe(method_family = none)]
        pub unsafe fn identities(&self) -> Retained<NSArray<CBIdentity>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl CBIdentityPicker {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
