//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/laenvironment?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LAEnvironment;
);

unsafe impl NSObjectProtocol for LAEnvironment {}

extern_methods!(
    unsafe impl LAEnvironment {
        /// The clients should use
        /// `currentUser`class property.
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The clients should use
        /// `currentUser`class property.
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Adds observer to monitor changes of the environment.
        ///
        /// The observer will be held weakly so its instance should be kept alive by the caller.
        #[method(addObserver:)]
        pub unsafe fn addObserver(&self, observer: &ProtocolObject<dyn LAEnvironmentObserver>);

        /// Removes the previously registered observer.
        ///
        /// If the observer is deallocated, it will be removed automatically.
        #[method(removeObserver:)]
        pub unsafe fn removeObserver(&self, observer: &ProtocolObject<dyn LAEnvironmentObserver>);

        /// Environment of the current user.
        #[method_id(@__retain_semantics Other currentUser)]
        pub unsafe fn currentUser() -> Retained<LAEnvironment>;

        #[cfg(feature = "LAEnvironmentState")]
        /// The environment state information.
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Retained<LAEnvironmentState>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/localauthentication/laenvironmentobserver?language=objc)
    pub unsafe trait LAEnvironmentObserver: NSObjectProtocol {
        #[cfg(feature = "LAEnvironmentState")]
        /// Called when there has been a change in the environment.
        ///
        /// Invoked on a queue private to LocalAuthentication framework. At the moment of invocation of this method,
        /// `LAEnvironment.state`already contains the new updated state.
        ///
        /// Parameter `oldState`: The old environment state (before update)
        #[optional]
        #[method(environment:stateDidChangeFromOldState:)]
        unsafe fn environment_stateDidChangeFromOldState(
            &self,
            environment: &LAEnvironment,
            old_state: &LAEnvironmentState,
        );
    }
);
