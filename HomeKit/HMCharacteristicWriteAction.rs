//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// This class is used to represent an entry in an action set that writes a specific
    /// value to a characteristic.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcharacteristicwriteaction?language=objc)
    #[unsafe(super(HMAction, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HMAction")]
    pub struct HMCharacteristicWriteAction<TargetValueType: ?Sized = AnyObject>;
);

#[cfg(feature = "HMAction")]
unsafe impl<TargetValueType: ?Sized> NSObjectProtocol
    for HMCharacteristicWriteAction<TargetValueType>
{
}

extern_methods!(
    #[cfg(feature = "HMAction")]
    unsafe impl<TargetValueType: Message> HMCharacteristicWriteAction<TargetValueType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "HMCharacteristic")]
        /// Initializer method that ties the action to a particular characteristic.
        ///
        ///
        /// Parameter `characteristic`: The characteristic bound to the action.
        ///
        ///
        /// Parameter `targetValue`: The target value for the characteristic.
        ///
        ///
        /// Returns: Instance object representing the characteristic write action.
        #[method_id(@__retain_semantics Init initWithCharacteristic:targetValue:)]
        pub unsafe fn initWithCharacteristic_targetValue(
            this: Allocated<Self>,
            characteristic: &HMCharacteristic,
            target_value: &TargetValueType,
        ) -> Retained<Self>;

        #[cfg(feature = "HMCharacteristic")]
        /// The characteristic associated with the action.
        #[method_id(@__retain_semantics Other characteristic)]
        pub unsafe fn characteristic(&self) -> Retained<HMCharacteristic>;

        /// The target value for the action.
        #[method_id(@__retain_semantics Other targetValue)]
        pub unsafe fn targetValue(&self) -> Retained<TargetValueType>;

        #[cfg(feature = "block2")]
        /// This method is used to change target value for the characteristic.
        ///
        ///
        /// Parameter `targetValue`: New target value for the characteristic.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[method(updateTargetValue:completionHandler:)]
        pub unsafe fn updateTargetValue_completionHandler(
            &self,
            target_value: &TargetValueType,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);