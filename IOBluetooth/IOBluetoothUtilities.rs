//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-foundation")]
use objc2_foundation::*;

use crate::*;

extern "C-unwind" {
    /// Convenience routine to take an NSString and turn it into a BluetoothDeviceAddress structure.
    ///
    /// Parameter `inNameString`: Ptr to an NSString that contains the data to turn into the device address.
    ///
    /// Parameter `outDeviceAddress`: Ptr to an address structure that will be returned.
    ///
    /// Returns: Returns success (0) or failure code.
    ///
    /// Pass in most types of strings, such as "001122334455" or "00-11-22-33-44-55" and the conversion should be successful. Also, you should have 2 characters per byte for the conversion to work properly.
    #[cfg(all(feature = "Bluetooth", feature = "objc2-foundation"))]
    pub fn IOBluetoothNSStringToDeviceAddress(
        in_name_string: Option<&NSString>,
        out_device_address: *mut BluetoothDeviceAddress,
    ) -> IOReturn;
}

/// Convenience routine to take a device address structure and create an NSString.
///
/// Parameter `deviceAddress`: A valid bluetooth device structure.
///
/// Returns: Returns the created address string.
///
/// The resultant string will be in this format: "00-11-22-33-44-55"
#[cfg(all(feature = "Bluetooth", feature = "objc2", feature = "objc2-foundation"))]
#[inline]
pub unsafe extern "C-unwind" fn IOBluetoothNSStringFromDeviceAddress(
    device_address: *const BluetoothDeviceAddress,
) -> Option<Retained<NSString>> {
    extern "C-unwind" {
        fn IOBluetoothNSStringFromDeviceAddress(
            device_address: *const BluetoothDeviceAddress,
        ) -> *mut NSString;
    }
    let ret = unsafe { IOBluetoothNSStringFromDeviceAddress(device_address) };
    unsafe { Retained::retain_autoreleased(ret) }
}

/// Convenience routine to take a device address structure and create an NSString.
///
/// Parameter `deviceAddress`: A valid bluetooth device structure.
///
/// Returns: Returns the created address string.
///
/// The resultant string will be in this format: "00:11:22:33:44:55"
#[cfg(all(feature = "Bluetooth", feature = "objc2", feature = "objc2-foundation"))]
#[inline]
pub unsafe extern "C-unwind" fn IOBluetoothNSStringFromDeviceAddressColon(
    device_address: *const BluetoothDeviceAddress,
) -> Option<Retained<NSString>> {
    extern "C-unwind" {
        fn IOBluetoothNSStringFromDeviceAddressColon(
            device_address: *const BluetoothDeviceAddress,
        ) -> *mut NSString;
    }
    let ret = unsafe { IOBluetoothNSStringFromDeviceAddressColon(device_address) };
    unsafe { Retained::retain_autoreleased(ret) }
}

/// Apple designated PIM data is classified as: .vcard, .vcal, .vcf, .vnote, .vmsg, .vcs
///
/// Parameter `inFileName`: Name of file - should include extension!
///
/// Returns: Yes or no, is it Apple-designated PIM data?
///
/// Not much to talk about.
#[cfg(feature = "objc2-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn IOBluetoothIsFileAppleDesignatedPIMData(
    in_file_name: Option<&NSString>,
) -> bool {
    extern "C-unwind" {
        fn IOBluetoothIsFileAppleDesignatedPIMData(in_file_name: Option<&NSString>) -> Boolean;
    }
    let ret = unsafe { IOBluetoothIsFileAppleDesignatedPIMData(in_file_name) };
    ret != 0
}

/// Parameter `inName`: Name of file that needs unique name in the specified path.
///
/// Parameter `inPath`: Path you are trying to put file into.
///
/// Returns: String with a unique name appended on it for the provided path.
///
/// When passed a VALID filename and a VALID path, this routine will return you a the path with the name
/// appended onto it. If it already exist, it will insert a #1, #2, etc. Example:
/// If you pass
/// &#64
/// "TestFile.txt" and
/// &#64
/// "~/Documents", you will get
/// &#64
/// "~Documents/TestFile.txt".
/// If one already exists, you will be returned:
/// &#64
/// "~Documents/TestFile #1.txt".
#[cfg(all(feature = "objc2", feature = "objc2-foundation"))]
#[inline]
pub unsafe extern "C-unwind" fn IOBluetoothGetUniqueFileNameAndPath(
    in_name: Option<&NSString>,
    in_path: Option<&NSString>,
) -> Option<Retained<NSString>> {
    extern "C-unwind" {
        fn IOBluetoothGetUniqueFileNameAndPath(
            in_name: Option<&NSString>,
            in_path: Option<&NSString>,
        ) -> *mut NSString;
    }
    let ret = unsafe { IOBluetoothGetUniqueFileNameAndPath(in_name, in_path) };
    unsafe { Retained::retain_autoreleased(ret) }
}

extern "C-unwind" {
    /// Returns total number of HID devices on the system (Bluetooth + USB)
    ///
    /// Returns: Number of HID devices.
    pub fn IOBluetoothNumberOfAvailableHIDDevices() -> c_long;
}

extern "C-unwind" {
    /// Returns number of "pointing" HID devices on the system (Bluetooth + USB)
    ///
    /// Returns: Number of HID devices.
    pub fn IOBluetoothNumberOfPointingHIDDevices() -> c_long;
}

extern "C-unwind" {
    /// Returns number of keyboard HID devices on the system (Bluetooth + USB)
    ///
    /// Returns: Number of HID devices.
    pub fn IOBluetoothNumberOfKeyboardHIDDevices() -> c_long;
}

extern "C-unwind" {
    /// Returns number of "Tablet" HID devices on the system (Bluetooth + USB)
    ///
    /// Returns: Number of HID devices.
    pub fn IOBluetoothNumberOfTabletHIDDevices() -> c_long;
}

extern "C-unwind" {
    /// Returns total number of registry entries with the provided device classname. e.g. "IOHIPointing"
    ///
    /// Returns: Number of HID devices.
    pub fn IOBluetoothFindNumberOfRegistryEntriesOfClassName(device_type: *const c_char) -> c_long;
}
