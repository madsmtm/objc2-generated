//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// This class can be used to launch system UI that will allow the user to go through the process
    /// of adding one or more accessories to a particular home and follow up with additional setup.
    /// These APIs do not require that the current app has home data authorization
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmaccessorysetupmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HMAccessorySetupManager;
);

unsafe impl Send for HMAccessorySetupManager {}

unsafe impl Sync for HMAccessorySetupManager {}

unsafe impl NSObjectProtocol for HMAccessorySetupManager {}

extern_methods!(
    unsafe impl HMAccessorySetupManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "HMAccessorySetupRequest",
            feature = "HMAccessorySetupResult",
            feature = "block2"
        ))]
        /// Launch system UI to perform the process of setting up accessories with the given request.
        /// During this process, each of the accessories is added to a home, assigned to a room and
        /// further configured based on its services
        ///
        ///
        /// Parameter `request`: A request object describing information about how to set up the accessory
        ///
        /// Parameter `completion`: A block that is invoked once the setup process finishes. On failure, the result will be
        /// nil and the error will provide additional information
        #[method(performAccessorySetupUsingRequest:completionHandler:)]
        pub unsafe fn performAccessorySetupUsingRequest_completionHandler(
            &self,
            request: &HMAccessorySetupRequest,
            completion: &block2::Block<dyn Fn(*mut HMAccessorySetupResult, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HMAccessorySetupManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);