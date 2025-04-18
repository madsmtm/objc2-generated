//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Represent an accessory in the home.
    ///
    ///
    /// This class represents an accessory in the home. There is a one to
    /// one relationship between a physical accessory and an object of this
    /// class. An accessory is composed of one or more services.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmaccessory?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HMAccessory;
);

unsafe impl Send for HMAccessory {}

unsafe impl Sync for HMAccessory {}

extern_conformance!(
    unsafe impl NSObjectProtocol for HMAccessory {}
);

impl HMAccessory {
    extern_methods!(
        /// The name of the accessory.
        ///
        ///
        /// Returns the accessory's name that is associated with HomeKit. The initial value is the name
        /// provided by the accessory information service of the accessory.
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// A unique identifier for the accessory.
        ///
        ///
        /// Use uniqueIdentifier to obtain the identifier for this object.
        #[deprecated = "No longer supported."]
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSUUID>;

        /// A unique identifier for the accessory.
        #[unsafe(method(uniqueIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn uniqueIdentifier(&self) -> Retained<NSUUID>;

        /// Delegate object that receives updates on the state of the accessory.
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn HMAccessoryDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn HMAccessoryDelegate>>,
        );

        /// TRUE if the accessory is currently reachable, FALSE otherwise.
        #[unsafe(method(isReachable))]
        #[unsafe(method_family = none)]
        pub unsafe fn isReachable(&self) -> bool;

        /// This property indicates whether this accessory is behind a bridge. If it is TRUE,
        /// the accessory cannot be removed from the home directly. Only the bridge that owns
        /// this accessory can be removed and removing the bridge will remove this accessory
        /// from the home.
        #[unsafe(method(isBridged))]
        #[unsafe(method_family = none)]
        pub unsafe fn isBridged(&self) -> bool;

        /// If this accessory is a bridge, this property is an array of NSUUID objects that,
        /// each of which represents the 'uniqueIdentifier' of the accessory vended by the bridge.
        ///
        ///
        /// Use uniqueIdentifiersForBridgedAccessories to obtain the identifiers for the
        /// bridged accessories.
        #[deprecated = "No longer supported."]
        #[unsafe(method(identifiersForBridgedAccessories))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifiersForBridgedAccessories(&self) -> Option<Retained<NSArray<NSUUID>>>;

        /// If this accessory is a bridge, this property is an array of NSUUID objects that,
        /// each of which represents the 'uniqueIdentifier' of the accessory vended by the bridge.
        ///
        ///
        /// An accessory can be standalone, a bridge, or hosted behind a bridge.
        /// - A standalone accessory would have its 'bridged' property set to FALSE and
        /// its 'uniqueIdentifiersForBridgedAccessories' property set to nil.
        /// - An accessory that is a bridge would have its 'bridged' property set to FALSE,
        /// but have a non-empty 'uniqueIdentifiersForBridgedAccessories' property.
        /// - An accessory behind a bridge would have its 'bridged' property set to TRUE and
        /// its 'uniqueIdentifiersForBridgedAccessories' property set to nil.
        #[unsafe(method(uniqueIdentifiersForBridgedAccessories))]
        #[unsafe(method_family = none)]
        pub unsafe fn uniqueIdentifiersForBridgedAccessories(
            &self,
        ) -> Option<Retained<NSArray<NSUUID>>>;

        #[cfg(feature = "HMAccessoryCategory")]
        /// Category information for the accessory.
        #[unsafe(method(category))]
        #[unsafe(method_family = none)]
        pub unsafe fn category(&self) -> Retained<HMAccessoryCategory>;

        #[cfg(feature = "HMRoom")]
        /// Room containing the accessory.
        #[unsafe(method(room))]
        #[unsafe(method_family = none)]
        pub unsafe fn room(&self) -> Option<Retained<HMRoom>>;

        #[cfg(feature = "HMService")]
        /// Array of HMService objects that represent all the services provided by the accessory.
        #[unsafe(method(services))]
        #[unsafe(method_family = none)]
        pub unsafe fn services(&self) -> Retained<NSArray<HMService>>;

        #[cfg(feature = "HMAccessoryProfile")]
        /// Accessory profiles of the receiver.
        #[unsafe(method(profiles))]
        #[unsafe(method_family = none)]
        pub unsafe fn profiles(&self) -> Retained<NSArray<HMAccessoryProfile>>;

        /// TRUE if the accessory is blocked, FALSE otherwise.
        #[unsafe(method(isBlocked))]
        #[unsafe(method_family = none)]
        pub unsafe fn isBlocked(&self) -> bool;

        /// Model of the accessory.
        #[unsafe(method(model))]
        #[unsafe(method_family = none)]
        pub unsafe fn model(&self) -> Option<Retained<NSString>>;

        /// Manufacturer of the accessory.
        #[unsafe(method(manufacturer))]
        #[unsafe(method_family = none)]
        pub unsafe fn manufacturer(&self) -> Option<Retained<NSString>>;

        /// Accessory's firmware version.
        #[unsafe(method(firmwareVersion))]
        #[unsafe(method_family = none)]
        pub unsafe fn firmwareVersion(&self) -> Option<Retained<NSString>>;

        /// Indicates if the accessory supports the identify action.
        #[unsafe(method(supportsIdentify))]
        #[unsafe(method_family = none)]
        pub unsafe fn supportsIdentify(&self) -> bool;

        /// The node identifier used to identify the device on Apple’s Matter fabric.
        #[unsafe(method(matterNodeID))]
        #[unsafe(method_family = none)]
        pub unsafe fn matterNodeID(&self) -> Option<Retained<NSNumber>>;

        #[cfg(feature = "block2")]
        /// This method is used to change the name of the accessory.
        ///
        ///
        /// Parameter `name`: New name for the accessory.
        ///
        ///
        /// The new name is stored in HomeKit and not on the accessory.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[unsafe(method(updateName:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateName_completionHandler(
            &self,
            name: &NSString,
            completion: &block2::DynBlock<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// This method is used to have an accessory identify itself.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[unsafe(method(identifyWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifyWithCompletionHandler(
            &self,
            completion: &block2::DynBlock<dyn Fn(*mut NSError)>,
        );

        #[deprecated = "HMAccessory objects are created by their parent container objects. Directly creating them is not supported."]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl HMAccessory {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// This defines the protocol for a delegate to receive updates about
    /// different aspects of an accessory
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmaccessorydelegate?language=objc)
    pub unsafe trait HMAccessoryDelegate: NSObjectProtocol {
        /// Informs the delegate when the name of the accessory is modified.
        ///
        ///
        /// Parameter `accessory`: Sender of the message.
        #[optional]
        #[unsafe(method(accessoryDidUpdateName:))]
        #[unsafe(method_family = none)]
        unsafe fn accessoryDidUpdateName(&self, accessory: &HMAccessory);

        #[cfg(feature = "HMService")]
        /// Informs the delegate when the name of a service is modified.
        ///
        ///
        /// Parameter `accessory`: Sender of the message.
        ///
        ///
        /// Parameter `service`: Service whose name was modified.
        #[optional]
        #[unsafe(method(accessory:didUpdateNameForService:))]
        #[unsafe(method_family = none)]
        unsafe fn accessory_didUpdateNameForService(
            &self,
            accessory: &HMAccessory,
            service: &HMService,
        );

        #[cfg(feature = "HMService")]
        /// Informs the delegate when the associated service type of a service is modified.
        ///
        ///
        /// Parameter `accessory`: Sender of the message.
        ///
        ///
        /// Parameter `service`: Service whose associated service type was modified.
        #[optional]
        #[unsafe(method(accessory:didUpdateAssociatedServiceTypeForService:))]
        #[unsafe(method_family = none)]
        unsafe fn accessory_didUpdateAssociatedServiceTypeForService(
            &self,
            accessory: &HMAccessory,
            service: &HMService,
        );

        /// Informs the delegate when the services on the accessory have been dynamically updated.
        /// The services discovered are accessible via the 'services' property of the accessory.
        ///
        ///
        /// Parameter `accessory`: Sender of the message.
        #[optional]
        #[unsafe(method(accessoryDidUpdateServices:))]
        #[unsafe(method_family = none)]
        unsafe fn accessoryDidUpdateServices(&self, accessory: &HMAccessory);

        #[cfg(feature = "HMAccessoryProfile")]
        /// Informs the delegate when a profile is added to an accessory.
        ///
        ///
        /// Parameter `accessory`: Sender of the message.
        ///
        /// Parameter `profile`: The added profile.
        #[optional]
        #[unsafe(method(accessory:didAddProfile:))]
        #[unsafe(method_family = none)]
        unsafe fn accessory_didAddProfile(
            &self,
            accessory: &HMAccessory,
            profile: &HMAccessoryProfile,
        );

        #[cfg(feature = "HMAccessoryProfile")]
        /// Informs the delegate when a profile is removed from an accessory.
        ///
        ///
        /// Parameter `accessory`: Sender of the message.
        ///
        /// Parameter `profile`: The removed profile.
        #[optional]
        #[unsafe(method(accessory:didRemoveProfile:))]
        #[unsafe(method_family = none)]
        unsafe fn accessory_didRemoveProfile(
            &self,
            accessory: &HMAccessory,
            profile: &HMAccessoryProfile,
        );

        /// Informs the delegate when the reachability of the accessory changes.
        ///
        ///
        /// Parameter `accessory`: Sender of the message.
        #[optional]
        #[unsafe(method(accessoryDidUpdateReachability:))]
        #[unsafe(method_family = none)]
        unsafe fn accessoryDidUpdateReachability(&self, accessory: &HMAccessory);

        #[cfg(all(feature = "HMCharacteristic", feature = "HMService"))]
        /// Informs the delegate of a change in value of a characteristic.
        ///
        ///
        /// Parameter `accessory`: Sender of this message
        ///
        ///
        /// Parameter `service`: HMService that contains the characteristic whose value was modified.
        ///
        ///
        /// Parameter `characteristic`: The characteristic whose value was changed.
        #[optional]
        #[unsafe(method(accessory:service:didUpdateValueForCharacteristic:))]
        #[unsafe(method_family = none)]
        unsafe fn accessory_service_didUpdateValueForCharacteristic(
            &self,
            accessory: &HMAccessory,
            service: &HMService,
            characteristic: &HMCharacteristic,
        );

        /// Informs the delegate when firmwareVersion has been changed for an accessory.
        ///
        ///
        /// Parameter `accessory`: Sender of the message.
        ///
        ///
        /// Parameter `firmwareVersion`: The newly updated firmwareVersion.
        #[optional]
        #[unsafe(method(accessory:didUpdateFirmwareVersion:))]
        #[unsafe(method_family = none)]
        unsafe fn accessory_didUpdateFirmwareVersion(
            &self,
            accessory: &HMAccessory,
            firmware_version: &NSString,
        );
    }
);
