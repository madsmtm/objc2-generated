//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "dispatch2")]
use dispatch2::*;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clliveupdateconfiguration?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLLiveUpdateConfiguration(pub NSInteger);
impl CLLiveUpdateConfiguration {
    #[doc(alias = "CLLiveUpdateConfigurationDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "CLLiveUpdateConfigurationAutomotiveNavigation")]
    pub const AutomotiveNavigation: Self = Self(1);
    #[doc(alias = "CLLiveUpdateConfigurationOtherNavigation")]
    pub const OtherNavigation: Self = Self(2);
    #[doc(alias = "CLLiveUpdateConfigurationFitness")]
    pub const Fitness: Self = Self(3);
    #[doc(alias = "CLLiveUpdateConfigurationAirborne")]
    pub const Airborne: Self = Self(4);
}

unsafe impl Encode for CLLiveUpdateConfiguration {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CLLiveUpdateConfiguration {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clupdate?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLUpdate;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CLUpdate {}
);

impl CLUpdate {
    extern_methods!(
        #[unsafe(method(authorizationDenied))]
        #[unsafe(method_family = none)]
        pub unsafe fn authorizationDenied(&self) -> bool;

        #[unsafe(method(authorizationDeniedGlobally))]
        #[unsafe(method_family = none)]
        pub unsafe fn authorizationDeniedGlobally(&self) -> bool;

        #[unsafe(method(authorizationRestricted))]
        #[unsafe(method_family = none)]
        pub unsafe fn authorizationRestricted(&self) -> bool;

        #[deprecated]
        #[unsafe(method(isStationary))]
        #[unsafe(method_family = none)]
        pub unsafe fn isStationary(&self) -> bool;

        #[unsafe(method(stationary))]
        #[unsafe(method_family = none)]
        pub unsafe fn stationary(&self) -> bool;

        #[unsafe(method(insufficientlyInUse))]
        #[unsafe(method_family = none)]
        pub unsafe fn insufficientlyInUse(&self) -> bool;

        #[unsafe(method(locationUnavailable))]
        #[unsafe(method_family = none)]
        pub unsafe fn locationUnavailable(&self) -> bool;

        #[unsafe(method(accuracyLimited))]
        #[unsafe(method_family = none)]
        pub unsafe fn accuracyLimited(&self) -> bool;

        #[unsafe(method(serviceSessionRequired))]
        #[unsafe(method_family = none)]
        pub unsafe fn serviceSessionRequired(&self) -> bool;

        #[unsafe(method(authorizationRequestInProgress))]
        #[unsafe(method_family = none)]
        pub unsafe fn authorizationRequestInProgress(&self) -> bool;

        #[cfg(feature = "CLLocation")]
        #[unsafe(method(location))]
        #[unsafe(method_family = none)]
        pub unsafe fn location(&self) -> Option<Retained<CLLocation>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl CLUpdate {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationupdater?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLLocationUpdater;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CLLocationUpdater {}
);

impl CLLocationUpdater {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "dispatch2"))]
        #[unsafe(method(liveUpdaterWithQueue:handler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn liveUpdaterWithQueue_handler(
            queue: &DispatchQueue,
            handler: &block2::DynBlock<dyn Fn(*mut CLUpdate)>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "block2", feature = "dispatch2"))]
        #[unsafe(method(liveUpdaterWithConfiguration:queue:handler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn liveUpdaterWithConfiguration_queue_handler(
            configuration: CLLiveUpdateConfiguration,
            queue: &DispatchQueue,
            handler: &block2::DynBlock<dyn Fn(*mut CLUpdate)>,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(resume))]
        #[unsafe(method_family = none)]
        pub unsafe fn resume(&self);

        #[unsafe(method(pause))]
        #[unsafe(method_family = none)]
        pub unsafe fn pause(&self);

        #[unsafe(method(invalidate))]
        #[unsafe(method_family = none)]
        pub unsafe fn invalidate(&self);
    );
}
