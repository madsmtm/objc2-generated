//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "dispatch2")]
use dispatch2::*;
use objc2::__framework_prelude::*;

use crate::*;

/// A CMIOObjectPropertySelector is a four char code that identifies, along with the CMIOObjectPropertyScope and CMIOObjectPropertyElement, a specific piece of
/// information about a CMIOObject.
///
/// The property selector specifies the general classification of the property such as volume, streamID format, latency, etc. Note that each class has a different set of
/// selectors. A subclass inherits it's super class's set of selectors, although it may not implement them all.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/cmioobjectpropertyselector?language=objc)
pub type CMIOObjectPropertySelector = u32;

/// A CMIOObjectPropertyScope is a four char code that identifies, along with the CMIOObjectPropertySelector and CMIOObjectPropertyElement, a specific piece of
/// information about a CMIOObject.
///
/// The scope specifies the section of the object in which to look for the property, such as input, output, global, etc. Note that each class has a different set of scopes. A
/// subclass inherits it's superclass's set of scopes.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/cmioobjectpropertyscope?language=objc)
pub type CMIOObjectPropertyScope = u32;

/// A CMIOObjectPropertyElement is an integer that identifies, along with the CMIOObjectPropertySelector and CMIOObjectPropertyScope, a specific piece of information
/// about a CMIOObject.
///
/// The element selects one of possibly many items in the section of the object in which to look for the property. Elements are numbered sequentially where 0 represents the
/// main element. Elements are particular to an instance of a class, meaning that two instances can have different numbers of elements in the same scope. There is no
/// inheritance of elements.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/cmioobjectpropertyelement?language=objc)
pub type CMIOObjectPropertyElement = u32;

/// A CMIOObjectPropertyAddress collects the three parts that identify a specific property together in a struct for easy transmission.
/// Field: mSelector
/// The CMIOObjectPropertySelector for the property.
/// Field: mScope
/// The CMIOObjectPropertyScope for the property.
/// Field: mElement
/// The CMIOObjectPropertyElement for the property.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/cmioobjectpropertyaddress?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMIOObjectPropertyAddress {
    pub mSelector: CMIOObjectPropertySelector,
    pub mScope: CMIOObjectPropertyScope,
    pub mElement: CMIOObjectPropertyElement,
}

unsafe impl Encode for CMIOObjectPropertyAddress {
    const ENCODING: Encoding = Encoding::Struct(
        "CMIOObjectPropertyAddress",
        &[
            <CMIOObjectPropertySelector>::ENCODING,
            <CMIOObjectPropertyScope>::ENCODING,
            <CMIOObjectPropertyElement>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CMIOObjectPropertyAddress {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyselectorwildcard?language=objc)
pub const kCMIOObjectPropertySelectorWildcard: c_uint = 0x2a2a2a2a;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyscopewildcard?language=objc)
pub const kCMIOObjectPropertyScopeWildcard: c_uint = 0x2a2a2a2a;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyelementwildcard?language=objc)
pub const kCMIOObjectPropertyElementWildcard: c_uint = 0xFFFFFFFF;

/// CMIOClassIDs are used to identify the class of a CMIOObject.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/cmioclassid?language=objc)
pub type CMIOClassID = u32;

/// CMIOObject is the base class for all the objects in the DAL.
///
/// CMIOObjects have properties and can contain other CMIOObjects.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/cmioobjectid?language=objc)
pub type CMIOObjectID = u32;

/// Clients register a CMIOObjectPropertyListenerProc with a CMIOObject in order to receive notifications when the properties of the object change.
///
/// Listeners will be called when possibly many properties have changed. Consequently, the implementation of a listener must go through the array of addresses to see what
/// exactly has changed. Note that the array of addresses will always have at least one address in it for which the listener is signed up to receive notifications about but
/// may contain addresses for properties for which the listener is not signed up to receive notifications.
///
/// Parameter `objectID`: The CMIOObject whose properties have changed.
///
/// Parameter `numberAddresses`: The number of elements in the addresses array.
///
/// Parameter `addresses`: An array of CMIOObjectPropertyAddresses indicating which properties changed.
///
/// Parameter `clientData`: A pointer to client data established when the listener proc was registered with the CMIOObject.
///
/// Returns: The return value is currently unused and should always be 0.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/cmioobjectpropertylistenerproc?language=objc)
pub type CMIOObjectPropertyListenerProc = Option<
    unsafe extern "C-unwind" fn(
        CMIOObjectID,
        u32,
        *mut CMIOObjectPropertyAddress,
        *mut c_void,
    ) -> OSStatus,
>;

/// Clients register an CMIOObjectPropertyListenerBlock with an CMIOObject in order to receive notifications when the properties of the object change.
///
/// Listeners will be called when possibly many properties have changed. Consequently, the implementation of a listener must go through the array of addresses to see what
/// exactly has changed. Note that the array of addresses will always have at least one address in it for which the listener is signed up to receive notifications about but
/// may contain addresses for properties for which the listener is not signed up to receive notifications.
///
/// Parameter `numberAddresses`: The number of elements in the addresses array.
///
/// Parameter `addresses`: An array of CMIOObjectPropertyAddresses indicating which properties changed.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/cmioobjectpropertylistenerblock?language=objc)
#[cfg(feature = "block2")]
pub type CMIOObjectPropertyListenerBlock =
    *mut block2::DynBlock<dyn Fn(u32, *mut CMIOObjectPropertyAddress)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyscopeglobal?language=objc)
pub const kCMIOObjectPropertyScopeGlobal: c_uint = 0x676c6f62;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyelementmain?language=objc)
pub const kCMIOObjectPropertyElementMain: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyelementmaster?language=objc)
#[deprecated]
pub const kCMIOObjectPropertyElementMaster: c_uint = kCMIOObjectPropertyElementMain;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectclassid?language=objc)
pub const kCMIOObjectClassID: c_uint = 0x616f626a;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectclassidwildcard?language=objc)
pub const kCMIOObjectClassIDWildcard: c_uint = 0x2a2a2a2a;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectunknown?language=objc)
pub const kCMIOObjectUnknown: c_uint = 0;

/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyclass?language=objc)
pub const kCMIOObjectPropertyClass: c_uint = 0x636c6173;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyowner?language=objc)
pub const kCMIOObjectPropertyOwner: c_uint = 0x73746476;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertycreator?language=objc)
pub const kCMIOObjectPropertyCreator: c_uint = 0x6f706c67;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyname?language=objc)
pub const kCMIOObjectPropertyName: c_uint = 0x6c6e616d;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertymanufacturer?language=objc)
pub const kCMIOObjectPropertyManufacturer: c_uint = 0x6c6d616b;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyelementname?language=objc)
pub const kCMIOObjectPropertyElementName: c_uint = 0x6c63686e;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyelementcategoryname?language=objc)
pub const kCMIOObjectPropertyElementCategoryName: c_uint = 0x6c63636e;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyelementnumbername?language=objc)
pub const kCMIOObjectPropertyElementNumberName: c_uint = 0x6c636e6e;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertyownedobjects?language=objc)
pub const kCMIOObjectPropertyOwnedObjects: c_uint = 0x6f776e64;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertylisteneradded?language=objc)
pub const kCMIOObjectPropertyListenerAdded: c_uint = 0x6c697361;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioobjectpropertylistenerremoved?language=objc)
pub const kCMIOObjectPropertyListenerRemoved: c_uint = 0x6c697372;

extern "C-unwind" {
    /// Prints to standard out a textural description of the CMIOObject.
    ///
    /// Parameter `objectID`: The CMIOObject to show.
    pub fn CMIOObjectShow(object_id: CMIOObjectID);
}

/// Queries a CMIOObject about whether or not it has the given property.
///
/// Parameter `objectID`: The CMIOObject to query.
///
/// Parameter `address`: A CMIOObjectPropertyAddress indicating which property is being queried.
///
/// Returns: A Boolean indicating whether or not the CMIOObject has the given property.
#[inline]
pub unsafe extern "C-unwind" fn CMIOObjectHasProperty(
    object_id: CMIOObjectID,
    address: *const CMIOObjectPropertyAddress,
) -> bool {
    extern "C-unwind" {
        fn CMIOObjectHasProperty(
            object_id: CMIOObjectID,
            address: *const CMIOObjectPropertyAddress,
        ) -> Boolean;
    }
    let ret = unsafe { CMIOObjectHasProperty(object_id, address) };
    ret != 0
}

extern "C-unwind" {
    /// Queries a CMIOObject about whether or not the given property can be set using CMIOObjectSetPropertyData.
    ///
    /// Parameter `objectID`: The CMIOObject to query.
    ///
    /// Parameter `address`: A CMIOObjectPropertyAddress indicating which property is being queried.
    ///
    /// Parameter `isSettable`: A Boolean indicating whether or not the property can be set.
    ///
    /// Returns: An OSStatus indicating success or failure.
    pub fn CMIOObjectIsPropertySettable(
        object_id: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        is_settable: *mut Boolean,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Queries a CMIOObject to find the size of the data for the given property.
    ///
    /// Parameter `objectID`: The CMIOObject to query.
    ///
    /// Parameter `address`: A CMIOObjectPropertyAddress indicating which property is being queried.
    ///
    /// Parameter `qualifierDataSize`: A UInt32 indicating the size of the buffer pointed to by qualifierData. Note that not all properties require qualification, in which case this value will be 0.
    ///
    /// Parameter `qualifierData`: A buffer of data to be used in determining the data of the property being queried. Note that not all properties require qualification, in which case this value will be
    /// NULL.
    ///
    /// Parameter `dataSize`: A UInt32 indicating how many bytes the data for the given property occupies.
    ///
    /// Returns: An OSStatus indicating success or failure.
    pub fn CMIOObjectGetPropertyDataSize(
        object_id: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: *mut u32,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Queries a CMIOObject to get the data of the given property and places it in the provided buffer.
    ///
    /// Parameter `objectID`: The CMIOObject to query.
    ///
    /// Parameter `address`: A CMIOObjectPropertyAddress indicating which property is being queried.
    ///
    /// Parameter `qualifierDataSize`: A UInt32 indicating the size of the buffer pointed to by qualifierData. Note that not all properties require qualification, in which case this value will be 0.
    ///
    /// Parameter `qualifierData`: A buffer of data to be used in determining the data of the property being queried. Note that not all properties require qualification, in which case this value will be
    /// NULL.
    ///
    /// Parameter `dataSize`: A UInt32 which indicates the size (in bytes) of the buffer pointed to by data.
    ///
    /// Parameter `dataUsed`: A UInt32 which indicates how much (in bytes) of the buffer was used.
    ///
    /// Parameter `data`: The buffer into which the CMIOObject will put the data for the given property.
    ///
    /// Returns: An OSStatus indicating success or failure.
    pub fn CMIOObjectGetPropertyData(
        object_id: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data_used: *mut u32,
        data: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Tells a CMIOObject to change the value of the given property using the provided data.
    ///
    /// Note that the value of the property should not be considered changed until the DAL has called the listeners as many properties values are changed asynchronously.
    ///
    /// Parameter `objectID`: The CMIOObject to change.
    ///
    /// Parameter `address`: A CMIOObjectPropertyAddress indicating which property is being changed.
    ///
    /// Parameter `qualifierDataSize`: A UInt32 indicating the size of the buffer pointed to by qualifierData. Note that not all properties require qualification, in which case this value will be 0.
    ///
    /// Parameter `qualifierData`: A buffer of data to be used in determining the data of the property being queried. Note that not all properties require qualification, in which case this value will be
    /// NULL.
    ///
    /// Parameter `dataSize`: A UInt32 indicating the size of the buffer pointed to by data.
    ///
    /// Parameter `data`: The buffer containing the data to be used to change the property's value.
    ///
    /// Returns: An OSStatus indicating success or failure.
    pub fn CMIOObjectSetPropertyData(
        object_id: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Registers the given CMIOObjectPropertyListenerProc to receive notifications when the given properties change.
    ///
    /// Parameter `objectID`: The CMIOObject to register the listener with.
    ///
    /// Parameter `address`: The CMIOObjectPropertyAddresses indicating which property the listener should be notified about.
    ///
    /// Parameter `listener`: The CMIOObjectPropertyListenerProc to call.
    ///
    /// Parameter `clientData`: A pointer to client data that is passed to the listener when it is called.
    ///
    /// Returns: An OSStatus indicating success or failure.
    pub fn CMIOObjectAddPropertyListener(
        object_id: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        listener: CMIOObjectPropertyListenerProc,
        client_data: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Unregisters the given CMIOObjectPropertyListenerProc from receiving notifications when the given properties change.
    ///
    /// Parameter `objectID`: The CMIOObject to unregister the listener from.
    ///
    /// Parameter `address`: The CMIOObjectPropertyAddress indicating which property the listener should be removed from.
    ///
    /// Parameter `listener`: The CMIOObjectPropertyListenerProc being removed.
    ///
    /// Parameter `clientData`: A pointer to client data that is passed to the listener when it is called.
    ///
    /// Returns: An OSStatus indicating success or failure.
    pub fn CMIOObjectRemovePropertyListener(
        object_id: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        listener: CMIOObjectPropertyListenerProc,
        client_data: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Registers the given CMIOObjectPropertyListenerBlock to receive notifications when the given properties change.
    ///
    /// Parameter `objectID`: The CMIOObject to register the listener with.
    ///
    /// Parameter `address`: The CMIOObjectPropertyAddresses indicating which property the listener should be notified about.
    ///
    /// Parameter `dispatchQueue`: The dispatch queue on which the listener block will be dispatched. All listener blocks will be dispatched asynchronously save for those dispatched from the IO context
    /// (of which kCMIODevicePropertyDeviceIsRunning and kCMIODeviceProcessorOverload are the only examples) which will be dispatched synchronously. Note that this dispatch
    /// queue will be retained until a matching call to CMIOObjectRemovePropertyListenerBlock is made. If this value is NULL, then the block will be directly invoked.
    ///
    /// Parameter `listener`: The CMIOObjectPropertyListenerBlock to call. Note that this block will be Block_copy'd and the reference maintained until a matching call to
    /// CMIOObjectRemovePropertyListenerBlock is made.
    ///
    /// Returns: An OSStatus indicating success or failure.
    #[cfg(all(feature = "block2", feature = "dispatch2"))]
    pub fn CMIOObjectAddPropertyListenerBlock(
        object_id: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        dispatch_queue: Option<&DispatchQueue>,
        listener: CMIOObjectPropertyListenerBlock,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Unregisters the given CMIOObjectPropertyListenerBlock from receiving notifications when the given properties change.
    ///
    /// Parameter `objectID`: The CMIOObject to unregister the listener from.
    ///
    /// Parameter `numberAddresses`: The number of elements in the addresses array.
    ///
    /// Parameter `addresses`: The CMIOObjectPropertyAddress indicating which property the listener should be removed from.
    ///
    /// Parameter `dispatchQueue`: The dispatch queue on which the listener block was being dispatched to.
    ///
    /// Parameter `listener`: The CMIOObjectPropertyListenerBlock being removed.
    ///
    /// Returns: An OSStatus indicating success or failure.
    #[cfg(all(feature = "block2", feature = "dispatch2"))]
    pub fn CMIOObjectRemovePropertyListenerBlock(
        object_id: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        dispatch_queue: Option<&DispatchQueue>,
        listener: CMIOObjectPropertyListenerBlock,
    ) -> OSStatus;
}
