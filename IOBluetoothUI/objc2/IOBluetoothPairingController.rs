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
    /// A NSWindowController subclass to display a window to initiate pairing to other bluetooth devices.
    ///
    /// Implementation of a window controller to handle pairing with a bluetooth device.  This
    /// class will handle connecting to the Bluetooth Daemon for the purposes of searches, and displaying the results.
    /// When necessary this class will display a sheet asking the user for a PIN code.  This window will not return
    /// anything to the caller if it is canceled or if pairing occurs.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iobluetoothui/iobluetoothpairingcontroller?language=objc)
    #[unsafe(super(NSWindowController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct IOBluetoothPairingController;
);

extern_conformance!(
    unsafe impl NSCoding for IOBluetoothPairingController {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for IOBluetoothPairingController {}
);

extern_conformance!(
    unsafe impl NSSeguePerforming for IOBluetoothPairingController {}
);

impl IOBluetoothPairingController {
    extern_methods!(
        /// Method call to instantiate a new IOBluetoothPairingController object.
        ///
        /// Returns: An IOBluetoothPairingController instance.  Call runPanelWithAttributes
        ///
        /// Success - a new instance of the Pairing Controller
        /// Failure    - nil
        #[unsafe(method(pairingController))]
        #[unsafe(method_family = none)]
        pub unsafe fn pairingController(
            mtm: MainThreadMarker,
        ) -> Option<Retained<IOBluetoothPairingController>>;

        /// Runs the pairing panel in a modal session to allow the user to select a Bluetooth device.
        ///
        /// The controller will use the panel attributes to filter what devices the user sees.  The allowed UUIDs
        /// will be used to validate the selection the user makes.  Only when a selection has been validated (or
        /// the panel cancelled) and the device paired, will this method return.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Returns: Returns kIOBluetoothUISuccess if a successful, validated device selection was made by the user and
        /// that device successfully paired.
        /// Returns kIOBluetoothUIUserCanceledErr if the user cancelled the panel.  These return values are the
        /// same as NSRunStoppedResponse and NSRunAbortedResponse respectively.  They are the standard values
        /// used in a modal session.
        #[unsafe(method(runModal))]
        #[unsafe(method_family = none)]
        pub unsafe fn runModal(&self) -> c_int;

        /// Returns an NSArray of the devices that were paired.
        ///
        /// There will only be results if the panel has been run, the user has successfully made a selection, that
        /// selection has been validated and the selected device paired.  If kIOBluetoothUISuccess was returned for
        /// the session, there should be valid results.  Currently only a single device is allowed to be selected,
        /// so the results array will only contain one object.  However in the future multiple selection may be supported.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Returns: Returns an NSArray of IOBluetoothDevice objects of devices that were paired.  If the user cancelled
        /// the panel, nil will be returned.
        #[unsafe(method(getResults))]
        #[unsafe(method_family = none)]
        pub unsafe fn getResults(&self) -> Option<Retained<NSArray>>;

        #[cfg(feature = "IOBluetoothUIUserLib")]
        /// Sets the option bits that control the panel's behavior.
        ///
        /// The pairing controller options control the behavior of the panel.  Currently
        /// kIOBluetoothServiceBrowserControllerOptionsAutoStartInquiry is the only supported option.
        /// In the future more options will be added to control things like whether the connection to
        /// the device is closed when the controller is finished or if multiple selection is allowed.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Parameter `options`: Options to control the panel's behavior.
        #[unsafe(method(setOptions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOptions(&self, options: IOBluetoothServiceBrowserControllerOptions);

        #[cfg(feature = "IOBluetoothUIUserLib")]
        /// Returns the option bits that control the panel's behavior.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Returns: Returns the option bits set by setOptions:
        #[unsafe(method(getOptions))]
        #[unsafe(method_family = none)]
        pub unsafe fn getOptions(&self) -> IOBluetoothServiceBrowserControllerOptions;

        #[cfg(feature = "objc2-io-bluetooth")]
        /// Sets the search attributes that control the panel's search/inquiry behavior.
        ///
        /// The device search attributes control the inquiry behavior of the panel.  They allow only devices
        /// that match the specified attributes (i.e. class of device) to be displayed to the user.  Note that
        /// this only covers attributes returned in an inquiry result and not actual SDP services on the device.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Parameter `searchAttributes`: Attributes to control the panel's inquiry behavior.
        #[unsafe(method(setSearchAttributes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSearchAttributes(
            &self,
            search_attributes: *const IOBluetoothDeviceSearchAttributes,
        );

        #[cfg(feature = "objc2-io-bluetooth")]
        /// Returns the search attributes that control the panel's search/inquiry behavior.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Returns: Returns the search attributes set by setSearchAttributes:
        #[unsafe(method(getSearchAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn getSearchAttributes(&self) -> *const IOBluetoothDeviceSearchAttributes;

        #[cfg(feature = "objc2-io-bluetooth")]
        /// Adds a UUID to the list of UUIDs that are used to validate the user's selection.
        ///
        /// The user's device selection gets validated against the UUIDs passed to -addAllowedUUID:
        /// addAllowedUUIDArray:.  Each call to those methods essentially adds a filter that the
        /// selected device gets validated with.  If any of the filters match, the device is considered
        /// valid.  If they all fail, the device is not valid and the user is presented with an
        /// error code that the device does not support the required services.  The UUID passed to
        /// -addAllowedUUID: is the only UUID that must be present in the device's SDP service records.
        /// Alternatively, all of the UUIDs in the UUID array passed to -addAllowedUUIDArray must be
        /// present.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Parameter `allowedUUID`: UUID that a device may contain to be selected
        #[unsafe(method(addAllowedUUID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addAllowedUUID(&self, allowed_uuid: Option<&IOBluetoothSDPUUID>);

        /// Adds an array of UUIDs to the list of UUIDs that are used to validate the user's selection.
        ///
        /// The user's device selection gets validated against the UUIDs passed to -addAllowedUUID:
        /// addAllowedUUIDArray:.  Each call to those methods essentially adds a filter that the
        /// selected device gets validated with.  If any of the filters match, the device is considered
        /// valid.  If they all fail, the device is not valid and the user is presented with an
        /// error code that the device does not support the required services.  The UUID passed to
        /// -addAllowedUUID: is the only UUID that must be present in the device's SDP service records.
        /// Alternatively, all of the UUIDs in the UUID array passed to -addAllowedUUIDArray must be
        /// present.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Parameter `allowedUUIDArray`: An NSArray of UUIDs that all must be present in a device for it to be selectable.
        #[unsafe(method(addAllowedUUIDArray:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addAllowedUUIDArray(&self, allowed_uuid_array: Option<&NSArray>);

        /// Resets the controller back to the default state where it will accept any device the user selects.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        #[unsafe(method(clearAllowedUUIDs))]
        #[unsafe(method_family = none)]
        pub unsafe fn clearAllowedUUIDs(&self);

        /// Sets the title of the panel when not run as a sheet.
        ///
        /// The panel title should be localized for best user experience.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Parameter `windowTitle`: Title of the device selector panel.
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, window_title: Option<&NSString>);

        /// Returns the title of the device selector panel (i.e. what was set in -setTitle:).
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Returns: Returns the title of the device selector panel.
        #[unsafe(method(getTitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn getTitle(&self) -> Option<Retained<NSString>>;

        /// Sets the description text that appears in the device selector panel.
        ///
        /// The description text should be localized for best user experience.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Parameter `descriptionText`: String that appears in the description section of the device selector panel.
        #[unsafe(method(setDescriptionText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDescriptionText(&self, description_text: Option<&NSString>);

        /// Returns the description text that appears in the device selector panel (i.e. what was set in -setDescriptionText:).
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Returns: Returns the description text of the device selector panel.
        #[unsafe(method(getDescriptionText))]
        #[unsafe(method_family = none)]
        pub unsafe fn getDescriptionText(&self) -> Option<Retained<NSString>>;

        /// Sets the title of the default/select button in the device selector panel.
        ///
        /// The prompt text should be localized for best user experience.
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Parameter `prompt`: String that appears in the default/select button in the device selector panel.
        #[unsafe(method(setPrompt:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPrompt(&self, prompt: Option<&NSString>);

        /// Returns the title of the default/select button in the device selector panel (i.e. what was set in -setPrompt:).
        ///
        /// NOTE: This method is only available in Mac OS X 10.2.4 (Bluetooth v1.1) or later.
        ///
        /// Returns: Returns the default button title of the device selector panel.
        #[unsafe(method(getPrompt))]
        #[unsafe(method_family = none)]
        pub unsafe fn getPrompt(&self) -> Option<Retained<NSString>>;
    );
}

/// Methods declared on superclass `NSWindowController`.
impl IOBluetoothPairingController {
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
impl IOBluetoothPairingController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl IOBluetoothPairingController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
