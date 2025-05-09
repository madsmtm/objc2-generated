//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// Used with predicateForObjectsWithDeviceProperty to specify a device name.
    ///
    /// The expected value type is an NSString.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeyname?language=objc)
    pub static HKDevicePropertyKeyName: &'static NSString;
}

extern "C" {
    /// Used with predicateForObjectsWithDeviceProperty to specify a device manufacturer.
    ///
    /// The expected value type is an NSString.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeymanufacturer?language=objc)
    pub static HKDevicePropertyKeyManufacturer: &'static NSString;
}

extern "C" {
    /// Used with predicateForObjectsWithDeviceProperty to specify a device model.
    ///
    /// The expected value type is an NSString.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeymodel?language=objc)
    pub static HKDevicePropertyKeyModel: &'static NSString;
}

extern "C" {
    /// Used with predicateForObjectsWithDeviceProperty to specify a hardware version.
    ///
    /// The expected value type is an NSString.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeyhardwareversion?language=objc)
    pub static HKDevicePropertyKeyHardwareVersion: &'static NSString;
}

extern "C" {
    /// Used with predicateForObjectsWithDeviceProperty to specify a firmware version.
    ///
    /// The expected value type is an NSString.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeyfirmwareversion?language=objc)
    pub static HKDevicePropertyKeyFirmwareVersion: &'static NSString;
}

extern "C" {
    /// Used with predicateForObjectsWithDeviceProperty to specify a software version.
    ///
    /// The expected value type is an NSString.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeysoftwareversion?language=objc)
    pub static HKDevicePropertyKeySoftwareVersion: &'static NSString;
}

extern "C" {
    /// Used with predicateForObjectsWithDeviceProperty to specify a local identifier.
    ///
    /// The expected value type is an NSString.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeylocalidentifier?language=objc)
    pub static HKDevicePropertyKeyLocalIdentifier: &'static NSString;
}

extern "C" {
    /// Used with predicateForObjectsWithDeviceProperty to specify a UDI device identifier.
    ///
    /// The expected value type is an NSString.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevicepropertykeyudideviceidentifier?language=objc)
    pub static HKDevicePropertyKeyUDIDeviceIdentifier: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkdevice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKDevice;
);

unsafe impl Send for HKDevice {}

unsafe impl Sync for HKDevice {}

extern_conformance!(
    unsafe impl NSCoding for HKDevice {}
);

extern_conformance!(
    unsafe impl NSCopying for HKDevice {}
);

unsafe impl CopyingHelper for HKDevice {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for HKDevice {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for HKDevice {}
);

impl HKDevice {
    extern_methods!(
        /// The name of the receiver.
        ///
        /// The user-facing name, such as the one displayed in the Bluetooth Settings for a BLE device.
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        /// The manufacturer of the receiver.
        #[unsafe(method(manufacturer))]
        #[unsafe(method_family = none)]
        pub unsafe fn manufacturer(&self) -> Option<Retained<NSString>>;

        /// The model of the receiver.
        #[unsafe(method(model))]
        #[unsafe(method_family = none)]
        pub unsafe fn model(&self) -> Option<Retained<NSString>>;

        /// The hardware revision of the receiver.
        #[unsafe(method(hardwareVersion))]
        #[unsafe(method_family = none)]
        pub unsafe fn hardwareVersion(&self) -> Option<Retained<NSString>>;

        /// The firmware revision of the receiver.
        #[unsafe(method(firmwareVersion))]
        #[unsafe(method_family = none)]
        pub unsafe fn firmwareVersion(&self) -> Option<Retained<NSString>>;

        /// The software revision of the receiver.
        #[unsafe(method(softwareVersion))]
        #[unsafe(method_family = none)]
        pub unsafe fn softwareVersion(&self) -> Option<Retained<NSString>>;

        /// A unique identifier for the receiver.
        ///
        /// This property is available to clients for a local identifier.
        /// For example, Bluetooth peripherals managed by HealthKit use this
        /// for the CoreBluetooth UUID which is valid only on the local
        /// device and thus distinguish the same Bluetooth peripheral used
        /// between multiple devices.
        #[unsafe(method(localIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn localIdentifier(&self) -> Option<Retained<NSString>>;

        /// Represents the device identifier portion of a device's FDA UDI (Unique Device Identifier).
        ///
        /// The device identifier can be used to reference the FDA's GUDID (Globally Unique Device
        /// Identifier Database). Note that for user privacy concerns this field should not be used to
        /// persist the production identifier portion of the device UDI. HealthKit clients should manage
        /// the production identifier independently, if needed.
        /// See http://www.fda.gov/MedicalDevices/DeviceRegulationandGuidance/UniqueDeviceIdentification/ for more information.
        #[unsafe(method(UDIDeviceIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn UDIDeviceIdentifier(&self) -> Option<Retained<NSString>>;

        /// Initialize a new HKDevice with the specified values.
        ///
        /// This allows initialization of an HKDevice object based on the
        /// information provided.
        #[unsafe(method(initWithName:manufacturer:model:hardwareVersion:firmwareVersion:softwareVersion:localIdentifier:UDIDeviceIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName_manufacturer_model_hardwareVersion_firmwareVersion_softwareVersion_localIdentifier_UDIDeviceIdentifier(
            this: Allocated<Self>,
            name: Option<&NSString>,
            manufacturer: Option<&NSString>,
            model: Option<&NSString>,
            hardware_version: Option<&NSString>,
            firmware_version: Option<&NSString>,
            software_version: Option<&NSString>,
            local_identifier: Option<&NSString>,
            udi_device_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Returns a device representing the host.
        ///
        /// If an app chooses to save samples that were retrieved from the local device, e.g. an HKWorkout with a
        /// totalDistance HKQuantity gathered from CoreLocation GPS distances, then this would be an appropriate
        /// HKDevice to use.
        #[unsafe(method(localDevice))]
        #[unsafe(method_family = none)]
        pub unsafe fn localDevice() -> Retained<HKDevice>;
    );
}

/// Methods declared on superclass `NSObject`.
impl HKDevice {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
