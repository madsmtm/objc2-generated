//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-io-bluetooth")]
use objc2_io_bluetooth::*;

use crate::*;

extern_class!(
    /// An NSWindowController subclass that supports the creation of an IOBluetoothObjectPushUIController object.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iobluetoothui/iobluetoothobjectpushuicontroller?language=objc)
    #[unsafe(super(NSWindowController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct IOBluetoothObjectPushUIController;
);

extern_conformance!(
    unsafe impl NSCoding for IOBluetoothObjectPushUIController {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for IOBluetoothObjectPushUIController {}
);

extern_conformance!(
    unsafe impl NSSeguePerforming for IOBluetoothObjectPushUIController {}
);

impl IOBluetoothObjectPushUIController {
    extern_methods!(
        #[cfg(feature = "objc2-io-bluetooth")]
        /// Creates and returns a new IOBluetoothObjectPush object
        ///
        /// The event delegate should implement a single delegate method:
        ///
        /// - (void) objectPushComplete: (IOBluetoothObjectPushUIController*) inPusher
        ///
        /// The method will be called when the transaction is complete and
        /// should be used to release the push object by the delegate. If no delegate is set
        /// the object will release itself when the transfer is finished.
        ///
        /// Parameter `inDevice`: The remote device to send the files to
        ///
        /// Parameter `inFiles`: An array of file paths to send
        ///
        /// Parameter `inDelegate`: A delegate object that implements the single method above.  If no delegate
        /// is specified this object will release itself when the transaction is complete.
        ///
        /// Returns: An IOBluetoothObjectPushUIController object on success, nil on fail.
        #[unsafe(method(initObjectPushWithBluetoothDevice:withFiles:delegate:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initObjectPushWithBluetoothDevice_withFiles_delegate(
            this: Allocated<Self>,
            in_device: Option<&IOBluetoothDevice>,
            in_files: Option<&NSArray>,
            in_delegate: Option<&AnyObject>,
        ) -> Option<Retained<IOBluetoothObjectPushUIController>>;

        /// Runs the transfer UI panel in a modal session
        ///
        /// Returns when the modal session has ended. This object will call back over the
        /// delegate method (above) when the transfer is complete.  Users should release
        /// the object then. If no delegate is set the object will release itself.
        ///
        /// Returns: The call will stall in this method until the modal session is complete.
        #[unsafe(method(runModal))]
        #[unsafe(method_family = none)]
        pub unsafe fn runModal(&self);

        /// Runs the transfer UI as a panel with no modal session
        ///
        /// Returns immediately.  The object will callback over the delegate method (above)
        /// when the transfer is completed.  If no delegate is set the object will release itself.
        ///
        /// Returns: The method will return immediately.
        #[unsafe(method(runPanel))]
        #[unsafe(method_family = none)]
        pub unsafe fn runPanel(&self);

        /// Runs the  transfer UI as a sheet on the target window.
        ///
        /// This function works the same way as -[NSApplication beginSheet:modalForWindow:modalDelegate:didEndSelector:contextInfo:].
        /// The didEndSelector has a similar prototype as in NSApplication except that the first argument is the
        /// IOBluetoothDeviceSelectorController object instead of the window:
        ///
        /// -(void)sheetDidEnd:(IOBluetoothDeviceSelectorController *)controller returnCode:(int)returnCode contextInfo:(void *)contextInfo.
        /// The returnCode parameter will either be kIOBluetoothUISuccess or kIOBluetoothUIUserCancelledErr as described in
        /// -runModal.
        ///
        ///
        /// Parameter `sheetWindow`: NSWindow to attach the device selector panel to as a sheet.
        ///
        /// Parameter `modalDelegate`: Delegate object that gets sent the didEndSelector when the sheet modal session is finished.
        ///
        /// Parameter `didEndSelector`: Selector sent to the modalDelegate when the sheet modal session is finished.
        ///
        /// Parameter `contextInfo`: User-definied value passed to the modalDelegate in the didEndSelector.
        ///
        /// Returns: Returns kIOReturnSuccess if the sheet modal session was started.
        #[unsafe(method(beginSheetModalForWindow:modalDelegate:didEndSelector:contextInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn beginSheetModalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            sheet_window: Option<&NSWindow>,
            modal_delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        ) -> IOReturn;

        /// Stops the transfer UI
        ///
        /// Returns immediately. The object will callback over the delegate method (above)
        /// when the transfer is completed, or will release itself if no delegate is set.
        ///
        /// Returns: The method will return immediately.
        #[unsafe(method(stop))]
        #[unsafe(method_family = none)]
        pub unsafe fn stop(&self);

        /// Sets the title of the panel when not run as a sheet.
        ///
        /// The panel title should be localized for best user experience.
        ///
        /// Parameter `windowTitle`: Title of the device selector panel.
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, window_title: Option<&NSString>);

        /// Returns the title of the transfer panel (i.e. what was set in -setTitle:).
        ///
        ///
        /// Returns: Returns the title of the transfer panel.
        #[unsafe(method(getTitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn getTitle(&self) -> Option<Retained<NSString>>;

        /// Manually sets the icon used in the panel.
        ///
        /// The panel icon should be set to the icon of the calling application.  If not set, the panel
        /// will try to load up the correct icon for the target device, and will default to the icon of
        /// the running application on fail.
        ///
        /// Parameter `image`: Image to use as the icon.
        #[unsafe(method(setIconImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIconImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "objc2-io-bluetooth")]
        /// Gets the object representing the remote target device in the transfer.
        ///
        ///
        /// Returns: The remote device of the transfer.
        #[unsafe(method(getDevice))]
        #[unsafe(method_family = none)]
        pub unsafe fn getDevice(&self) -> Option<Retained<IOBluetoothDevice>>;

        /// Gets state of the transfer
        ///
        ///
        /// Returns: The state of the transfer
        #[unsafe(method(isTransferInProgress))]
        #[unsafe(method_family = none)]
        pub unsafe fn isTransferInProgress(&self) -> bool;
    );
}

/// Methods declared on superclass `NSWindowController`.
impl IOBluetoothObjectPushUIController {
    extern_methods!(
        #[unsafe(method(initWithWindow:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindow(
            this: Allocated<Self>,
            window: Option<&NSWindow>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(initWithWindowNibName:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindowNibName(
            this: Allocated<Self>,
            window_nib_name: &NSNibName,
        ) -> Retained<Self>;

        #[unsafe(method(initWithWindowNibName:owner:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindowNibName_owner(
            this: Allocated<Self>,
            window_nib_name: &NSNibName,
            owner: &AnyObject,
        ) -> Retained<Self>;

        #[unsafe(method(initWithWindowNibPath:owner:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithWindowNibPath_owner(
            this: Allocated<Self>,
            window_nib_path: &NSString,
            owner: &AnyObject,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSResponder`.
impl IOBluetoothObjectPushUIController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl IOBluetoothObjectPushUIController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
