//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// The category describes the general type of audio that the app plans to use.
///
/// Provides context to the operating system about the type of audio an application intends to use. The system uses this information
/// when arbitrating between Apple products that want to take ownership of Bluetooth audio routes.
///
/// Used for Audio playback.
///
/// Used for recording and playing back audio.
///
/// Appropriate for Voice over IP(VoIP) applications.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioroutingarbitrationcategory?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVAudioRoutingArbitrationCategory(pub NSInteger);
impl AVAudioRoutingArbitrationCategory {
    #[doc(alias = "AVAudioRoutingArbitrationCategoryPlayback")]
    pub const Playback: Self = Self(0);
    #[doc(alias = "AVAudioRoutingArbitrationCategoryPlayAndRecord")]
    pub const PlayAndRecord: Self = Self(1);
    #[doc(alias = "AVAudioRoutingArbitrationCategoryPlayAndRecordVoice")]
    pub const PlayAndRecordVoice: Self = Self(2);
}

unsafe impl Encode for AVAudioRoutingArbitrationCategory {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVAudioRoutingArbitrationCategory {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// The interface to participate in audio routing arbitration.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioroutingarbiter?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioRoutingArbiter;
);

unsafe impl Send for AVAudioRoutingArbiter {}

unsafe impl Sync for AVAudioRoutingArbiter {}

extern_conformance!(
    unsafe impl NSObjectProtocol for AVAudioRoutingArbiter {}
);

impl AVAudioRoutingArbiter {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Returns the singleton AVAudioRoutingArbiter instance.
        #[unsafe(method(sharedRoutingArbiter))]
        #[unsafe(method_family = none)]
        pub unsafe fn sharedRoutingArbiter() -> Retained<AVAudioRoutingArbiter>;

        #[cfg(feature = "block2")]
        /// Begin routing arbitration to take ownership of nearby Bluetooth audio routes.
        ///
        /// When an app wants to participate in automatic audio arbitration for the wireless headphones route, it has to begin arbitration
        /// specifying its arbitration session category. It provides the operating system time to arbitrate with other nearby Apple
        /// devices to obtain ownership of supported Bluetooth audio devices. Then upon completion of arbitration, the operating system
        /// will automatically determine  whether to route audio to the nearby Bluetooth device. Once arbitration completes, the application
        /// is free to start running audio I/O. I/O will be started upon the app request even if the -beginArbitrationWithCategory:completionHandler: fails.
        /// This method should also be used whenever restarting audio I/O in order to allow the system to arbitrate for ownership of a Bluetooth
        /// device that may have been taken by another nearby Apple device during the time that I/O was stopped.
        ///
        /// Parameter `category`: The category describes the general type of audio that the app plans to use.
        ///
        /// Parameter `handler`: A client-supplied block called asynchronously when audio routing arbitration is completed.
        /// This completion handler takes the following parameters:
        /// defaultDeviceChanged
        /// Indicating that the system default audio device has been changed as a result of the arbitration operation.
        /// error
        /// An error object that indicates why the request failed, or nil if the request was successful.
        #[unsafe(method(beginArbitrationWithCategory:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn beginArbitrationWithCategory_completionHandler(
            &self,
            category: AVAudioRoutingArbitrationCategory,
            handler: &block2::DynBlock<dyn Fn(Bool, *mut NSError)>,
        );

        /// Stop participating in audio routing arbitration.
        ///
        /// When an application has stopped using audio for the foreseeable future, it should notify the system. For example,
        /// in Voice over IP (VoIP)  use cases, the application should call -leaveArbitration when the VoIP call has ended.
        /// This allows the system to make a better decision when other participating Apple devices would like to take ownership
        /// of a nearby Bluetooth device. Applications should not call this API in cases where audio is only momentarily paused.
        #[unsafe(method(leaveArbitration))]
        #[unsafe(method_family = none)]
        pub unsafe fn leaveArbitration(&self);
    );
}
