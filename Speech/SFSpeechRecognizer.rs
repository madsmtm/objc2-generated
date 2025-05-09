//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/speech/sfspeechrecognizerauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SFSpeechRecognizerAuthorizationStatus(pub NSInteger);
impl SFSpeechRecognizerAuthorizationStatus {
    #[doc(alias = "SFSpeechRecognizerAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "SFSpeechRecognizerAuthorizationStatusDenied")]
    pub const Denied: Self = Self(1);
    #[doc(alias = "SFSpeechRecognizerAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(2);
    #[doc(alias = "SFSpeechRecognizerAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

unsafe impl Encode for SFSpeechRecognizerAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SFSpeechRecognizerAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/speech/sfspeechrecognizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSpeechRecognizer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for SFSpeechRecognizer {}
);

impl SFSpeechRecognizer {
    extern_methods!(
        #[unsafe(method(supportedLocales))]
        #[unsafe(method_family = none)]
        pub unsafe fn supportedLocales() -> Retained<NSSet<NSLocale>>;

        #[unsafe(method(authorizationStatus))]
        #[unsafe(method_family = none)]
        pub unsafe fn authorizationStatus() -> SFSpeechRecognizerAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[unsafe(method(requestAuthorization:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestAuthorization(
            handler: &block2::DynBlock<dyn Fn(SFSpeechRecognizerAuthorizationStatus)>,
        );

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Option<Retained<Self>>;

        #[unsafe(method(initWithLocale:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLocale(
            this: Allocated<Self>,
            locale: &NSLocale,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(isAvailable))]
        #[unsafe(method_family = none)]
        pub unsafe fn isAvailable(&self) -> bool;

        #[unsafe(method(locale))]
        #[unsafe(method_family = none)]
        pub unsafe fn locale(&self) -> Retained<NSLocale>;

        #[unsafe(method(supportsOnDeviceRecognition))]
        #[unsafe(method_family = none)]
        pub unsafe fn supportsOnDeviceRecognition(&self) -> bool;

        /// Setter for [`supportsOnDeviceRecognition`][Self::supportsOnDeviceRecognition].
        #[unsafe(method(setSupportsOnDeviceRecognition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSupportsOnDeviceRecognition(&self, supports_on_device_recognition: bool);

        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn SFSpeechRecognizerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn SFSpeechRecognizerDelegate>>,
        );

        #[cfg(feature = "SFSpeechRecognitionTaskHint")]
        #[unsafe(method(defaultTaskHint))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultTaskHint(&self) -> SFSpeechRecognitionTaskHint;

        #[cfg(feature = "SFSpeechRecognitionTaskHint")]
        /// Setter for [`defaultTaskHint`][Self::defaultTaskHint].
        #[unsafe(method(setDefaultTaskHint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDefaultTaskHint(&self, default_task_hint: SFSpeechRecognitionTaskHint);

        #[cfg(all(
            feature = "SFSpeechRecognitionRequest",
            feature = "SFSpeechRecognitionResult",
            feature = "SFSpeechRecognitionTask",
            feature = "block2"
        ))]
        #[unsafe(method(recognitionTaskWithRequest:resultHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recognitionTaskWithRequest_resultHandler(
            &self,
            request: &SFSpeechRecognitionRequest,
            result_handler: &block2::DynBlock<dyn Fn(*mut SFSpeechRecognitionResult, *mut NSError)>,
        ) -> Retained<SFSpeechRecognitionTask>;

        #[cfg(all(
            feature = "SFSpeechRecognitionRequest",
            feature = "SFSpeechRecognitionTask"
        ))]
        #[unsafe(method(recognitionTaskWithRequest:delegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recognitionTaskWithRequest_delegate(
            &self,
            request: &SFSpeechRecognitionRequest,
            delegate: &ProtocolObject<dyn SFSpeechRecognitionTaskDelegate>,
        ) -> Retained<SFSpeechRecognitionTask>;

        #[unsafe(method(queue))]
        #[unsafe(method_family = none)]
        pub unsafe fn queue(&self) -> Retained<NSOperationQueue>;

        /// Setter for [`queue`][Self::queue].
        #[unsafe(method(setQueue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setQueue(&self, queue: &NSOperationQueue);
    );
}

/// Methods declared on superclass `NSObject`.
impl SFSpeechRecognizer {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/speech/sfspeechrecognizerdelegate?language=objc)
    pub unsafe trait SFSpeechRecognizerDelegate: NSObjectProtocol {
        #[optional]
        #[unsafe(method(speechRecognizer:availabilityDidChange:))]
        #[unsafe(method_family = none)]
        unsafe fn speechRecognizer_availabilityDidChange(
            &self,
            speech_recognizer: &SFSpeechRecognizer,
            available: bool,
        );
    }
);
