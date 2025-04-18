//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// This class represents an event that is evaluated based on the value of a characteristic
    /// set to a particular value.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcharacteristicevent?language=objc)
    #[unsafe(super(HMEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HMEvent")]
    pub struct HMCharacteristicEvent<TriggerValueType: ?Sized = AnyObject>;
);

#[cfg(feature = "HMEvent")]
extern_conformance!(
    unsafe impl<TriggerValueType: ?Sized> NSCopying for HMCharacteristicEvent<TriggerValueType> {}
);

#[cfg(feature = "HMEvent")]
unsafe impl<TriggerValueType: ?Sized + Message> CopyingHelper
    for HMCharacteristicEvent<TriggerValueType>
{
    type Result = Self;
}

#[cfg(feature = "HMEvent")]
extern_conformance!(
    unsafe impl<TriggerValueType: ?Sized> NSMutableCopying for HMCharacteristicEvent<TriggerValueType> {}
);

#[cfg(feature = "HMEvent")]
unsafe impl<TriggerValueType: ?Sized + Message> MutableCopyingHelper
    for HMCharacteristicEvent<TriggerValueType>
{
    type Result = HMMutableCharacteristicEvent<TriggerValueType>;
}

#[cfg(feature = "HMEvent")]
extern_conformance!(
    unsafe impl<TriggerValueType: ?Sized> NSObjectProtocol for HMCharacteristicEvent<TriggerValueType> {}
);

#[cfg(feature = "HMEvent")]
impl<TriggerValueType: Message> HMCharacteristicEvent<TriggerValueType> {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "HMCharacteristic")]
        /// Initializes a new characteristic event object
        ///
        ///
        /// Parameter `characteristic`: The characteristic bound to the event. The characteristic must
        /// support notification. An exception will be thrown otherwise.
        ///
        ///
        /// Parameter `triggerValue`: The value of the characteristic that triggers the event.
        /// Specifying nil as the trigger value corresponds to any change in the value of the
        /// characteristic.
        ///
        ///
        /// Returns: Instance object representing the characteristic event.
        #[unsafe(method(initWithCharacteristic:triggerValue:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCharacteristic_triggerValue(
            this: Allocated<Self>,
            characteristic: &HMCharacteristic,
            trigger_value: Option<&TriggerValueType>,
        ) -> Retained<Self>;

        #[cfg(feature = "HMCharacteristic")]
        /// The characteristic associated with the event.
        #[unsafe(method(characteristic))]
        #[unsafe(method_family = none)]
        pub unsafe fn characteristic(&self) -> Retained<HMCharacteristic>;

        /// The value of the characteristic that triggers the event.
        /// A value of nil corresponds to any change in the value of the characteristic.
        #[unsafe(method(triggerValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn triggerValue(&self) -> Option<Retained<TriggerValueType>>;

        #[cfg(feature = "block2")]
        /// This method is used to change trigger value for the characteristic.
        ///
        ///
        /// Parameter `triggerValue`: New trigger value for the characteristic.
        /// Specifying nil as the trigger value corresponds to any change in the value of the
        /// characteristic.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[deprecated = "No longer supported."]
        #[unsafe(method(updateTriggerValue:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateTriggerValue_completionHandler(
            &self,
            trigger_value: Option<&TriggerValueType>,
            completion: &block2::DynBlock<dyn Fn(*mut NSError)>,
        );
    );
}

/// Methods declared on superclass `HMEvent`.
#[cfg(feature = "HMEvent")]
impl<TriggerValueType: Message> HMCharacteristicEvent<TriggerValueType> {
    extern_methods!(
        #[deprecated = "HMEvent is abstract"]
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// This class represents an event that is evaluated based on the value of a characteristic
    /// set to a particular value.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmmutablecharacteristicevent?language=objc)
    #[unsafe(super(HMCharacteristicEvent, HMEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HMEvent")]
    pub struct HMMutableCharacteristicEvent<TriggerValueType: ?Sized = AnyObject>;
);

#[cfg(feature = "HMEvent")]
extern_conformance!(
    unsafe impl<TriggerValueType: ?Sized> NSCopying for HMMutableCharacteristicEvent<TriggerValueType> {}
);

#[cfg(feature = "HMEvent")]
unsafe impl<TriggerValueType: ?Sized + Message> CopyingHelper
    for HMMutableCharacteristicEvent<TriggerValueType>
{
    type Result = HMCharacteristicEvent<TriggerValueType>;
}

#[cfg(feature = "HMEvent")]
extern_conformance!(
    unsafe impl<TriggerValueType: ?Sized> NSMutableCopying
        for HMMutableCharacteristicEvent<TriggerValueType>
    {
    }
);

#[cfg(feature = "HMEvent")]
unsafe impl<TriggerValueType: ?Sized + Message> MutableCopyingHelper
    for HMMutableCharacteristicEvent<TriggerValueType>
{
    type Result = Self;
}

#[cfg(feature = "HMEvent")]
extern_conformance!(
    unsafe impl<TriggerValueType: ?Sized> NSObjectProtocol
        for HMMutableCharacteristicEvent<TriggerValueType>
    {
    }
);

#[cfg(feature = "HMEvent")]
impl<TriggerValueType: Message> HMMutableCharacteristicEvent<TriggerValueType> {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "HMCharacteristic")]
        /// The characteristic associated with the event.
        #[unsafe(method(characteristic))]
        #[unsafe(method_family = none)]
        pub unsafe fn characteristic(&self) -> Retained<HMCharacteristic>;

        #[cfg(feature = "HMCharacteristic")]
        /// Setter for [`characteristic`][Self::characteristic].
        #[unsafe(method(setCharacteristic:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCharacteristic(&self, characteristic: &HMCharacteristic);

        /// The value of the characteristic that triggers the event.
        /// A value of nil corresponds to any change in the value of the characteristic.
        #[unsafe(method(triggerValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn triggerValue(&self) -> Option<Retained<TriggerValueType>>;

        /// Setter for [`triggerValue`][Self::triggerValue].
        #[unsafe(method(setTriggerValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTriggerValue(&self, trigger_value: Option<&TriggerValueType>);
    );
}

/// Methods declared on superclass `HMCharacteristicEvent`.
#[cfg(feature = "HMEvent")]
impl<TriggerValueType: Message> HMMutableCharacteristicEvent<TriggerValueType> {
    extern_methods!(
        #[cfg(feature = "HMCharacteristic")]
        /// Initializes a new characteristic event object
        ///
        ///
        /// Parameter `characteristic`: The characteristic bound to the event. The characteristic must
        /// support notification. An exception will be thrown otherwise.
        ///
        ///
        /// Parameter `triggerValue`: The value of the characteristic that triggers the event.
        /// Specifying nil as the trigger value corresponds to any change in the value of the
        /// characteristic.
        ///
        ///
        /// Returns: Instance object representing the characteristic event.
        #[unsafe(method(initWithCharacteristic:triggerValue:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCharacteristic_triggerValue(
            this: Allocated<Self>,
            characteristic: &HMCharacteristic,
            trigger_value: Option<&TriggerValueType>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `HMEvent`.
#[cfg(feature = "HMEvent")]
impl<TriggerValueType: Message> HMMutableCharacteristicEvent<TriggerValueType> {
    extern_methods!(
        #[deprecated = "HMEvent is abstract"]
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
