//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Used to describe a collection of HMRoom objects
    ///
    ///
    /// This class is used to group a collection of rooms.
    /// This allows for association of a set of rooms into a group.
    /// Eg. "Living Room" and "Kitchen" rooms can be grouped together
    /// in the "Downstairs" zone.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmzone?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HMZone;
);

unsafe impl Send for HMZone {}

unsafe impl Sync for HMZone {}

unsafe impl NSObjectProtocol for HMZone {}

extern_methods!(
    unsafe impl HMZone {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Name of the zone.
        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "HMRoom")]
        /// Array of HMRoom objects that correspond to the rooms contained in this zone.
        #[unsafe(method_family(none))]
        #[method_id(rooms)]
        pub unsafe fn rooms(&self) -> Retained<NSArray<HMRoom>>;

        /// A unique identifier for the zone.
        #[unsafe(method_family(none))]
        #[method_id(uniqueIdentifier)]
        pub unsafe fn uniqueIdentifier(&self) -> Retained<NSUUID>;

        #[cfg(feature = "block2")]
        /// This method is used to change the name of the zone.
        ///
        ///
        /// Parameter `name`: New name for the zone.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[method(updateName:completionHandler:)]
        pub unsafe fn updateName_completionHandler(
            &self,
            name: &NSString,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "HMRoom", feature = "block2"))]
        /// Adds a room to a zone.
        ///
        ///
        /// Both the room and the zone should be part of the home.  A room can be added to multiple
        /// zones, e.g., a room "Kitchen" can be added to "Downstairs" as well as "Outdoor" zones.
        ///
        ///
        /// Parameter `room`: Room to add to this zone.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[method(addRoom:completionHandler:)]
        pub unsafe fn addRoom_completionHandler(
            &self,
            room: &HMRoom,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "HMRoom", feature = "block2"))]
        /// Removes a room from the zone.
        ///
        ///
        /// Parameter `room`: Room to remove from this zone.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[method(removeRoom:completionHandler:)]
        pub unsafe fn removeRoom_completionHandler(
            &self,
            room: &HMRoom,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HMZone {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
