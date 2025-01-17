//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechboundary?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechBoundary(pub NSInteger);
impl AVSpeechBoundary {
    #[doc(alias = "AVSpeechBoundaryImmediate")]
    pub const Immediate: Self = Self(0);
    #[doc(alias = "AVSpeechBoundaryWord")]
    pub const Word: Self = Self(1);
}

unsafe impl Encode for AVSpeechBoundary {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechBoundary {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoicequality?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisVoiceQuality(pub NSInteger);
impl AVSpeechSynthesisVoiceQuality {
    #[doc(alias = "AVSpeechSynthesisVoiceQualityDefault")]
    pub const Default: Self = Self(1);
    #[doc(alias = "AVSpeechSynthesisVoiceQualityEnhanced")]
    pub const Enhanced: Self = Self(2);
    #[doc(alias = "AVSpeechSynthesisVoiceQualityPremium")]
    pub const Premium: Self = Self(3);
}

unsafe impl Encode for AVSpeechSynthesisVoiceQuality {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisVoiceQuality {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoicegender?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisVoiceGender(pub NSInteger);
impl AVSpeechSynthesisVoiceGender {
    #[doc(alias = "AVSpeechSynthesisVoiceGenderUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "AVSpeechSynthesisVoiceGenderMale")]
    pub const Male: Self = Self(1);
    #[doc(alias = "AVSpeechSynthesisVoiceGenderFemale")]
    pub const Female: Self = Self(2);
}

unsafe impl Encode for AVSpeechSynthesisVoiceGender {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisVoiceGender {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Markers used in the output event callback. Used for providing metadata on synthesized audio.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesismarkermark?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisMarkerMark(pub NSInteger);
impl AVSpeechSynthesisMarkerMark {
    #[doc(alias = "AVSpeechSynthesisMarkerMarkPhoneme")]
    pub const Phoneme: Self = Self(0);
    #[doc(alias = "AVSpeechSynthesisMarkerMarkWord")]
    pub const Word: Self = Self(1);
    #[doc(alias = "AVSpeechSynthesisMarkerMarkSentence")]
    pub const Sentence: Self = Self(2);
    #[doc(alias = "AVSpeechSynthesisMarkerMarkParagraph")]
    pub const Paragraph: Self = Self(3);
    #[doc(alias = "AVSpeechSynthesisMarkerMarkBookmark")]
    pub const Bookmark: Self = Self(4);
}

unsafe impl Encode for AVSpeechSynthesisMarkerMark {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisMarkerMark {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechutteranceminimumspeechrate?language=objc)
    pub static AVSpeechUtteranceMinimumSpeechRate: c_float;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechutterancemaximumspeechrate?language=objc)
    pub static AVSpeechUtteranceMaximumSpeechRate: c_float;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechutterancedefaultspeechrate?language=objc)
    pub static AVSpeechUtteranceDefaultSpeechRate: c_float;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoiceidentifieralex?language=objc)
    pub static AVSpeechSynthesisVoiceIdentifierAlex: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisipanotationattribute?language=objc)
    pub static AVSpeechSynthesisIPANotationAttribute: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesizerbuffercallback?language=objc)
#[cfg(all(feature = "AVAudioBuffer", feature = "block2"))]
pub type AVSpeechSynthesizerBufferCallback = *mut block2::Block<dyn Fn(NonNull<AVAudioBuffer>)>;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesizermarkercallback?language=objc)
#[cfg(feature = "block2")]
pub type AVSpeechSynthesizerMarkerCallback =
    *mut block2::Block<dyn Fn(NonNull<NSArray<AVSpeechSynthesisMarker>>)>;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesispersonalvoiceauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisPersonalVoiceAuthorizationStatus(pub NSUInteger);
impl AVSpeechSynthesisPersonalVoiceAuthorizationStatus {
    /// The app's authorization status has not yet been determined.
    ///
    /// When your app's status is not determined, calling the requestAuthorization: method prompts the user to grant or deny authorization.
    #[doc(alias = "AVSpeechSynthesisPersonalVoiceAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    /// The user denied your app's request to use personal voices.
    #[doc(alias = "AVSpeechSynthesisPersonalVoiceAuthorizationStatusDenied")]
    pub const Denied: Self = Self(1);
    /// Personal voices are unsupported on this device.
    #[doc(alias = "AVSpeechSynthesisPersonalVoiceAuthorizationStatusUnsupported")]
    pub const Unsupported: Self = Self(2);
    /// The user granted your app's request to use personal voices.
    #[doc(alias = "AVSpeechSynthesisPersonalVoiceAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

unsafe impl Encode for AVSpeechSynthesisPersonalVoiceAuthorizationStatus {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisPersonalVoiceAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoicetraits?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSpeechSynthesisVoiceTraits(pub NSUInteger);
bitflags::bitflags! {
    impl AVSpeechSynthesisVoiceTraits: NSUInteger {
        #[doc(alias = "AVSpeechSynthesisVoiceTraitNone")]
        const None = 0;
/// The voice is generally for novelty purposes, for example a character's voice in a game.
        #[doc(alias = "AVSpeechSynthesisVoiceTraitIsNoveltyVoice")]
        const IsNoveltyVoice = 1<<0;
/// The voice is was generated by, and belongs to the user. Voices with this trait will only be avilable when AVSpeechSynthesizer.personalVoiceAuthorizationStatus is AVSpeechSynthesisPersonalVoiceAuthorizationStatusAuthorized
        #[doc(alias = "AVSpeechSynthesisVoiceTraitIsPersonalVoice")]
        const IsPersonalVoice = 1<<1;
    }
}

unsafe impl Encode for AVSpeechSynthesisVoiceTraits {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVSpeechSynthesisVoiceTraits {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// Posted when available voices for speech synthesis on the system have changed. For example, if new 3rd party voices are available through a downloaded app, or if a new personal voice is available and the app is authorized to access personal voices.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisavailablevoicesdidchangenotification?language=objc)
    pub static AVSpeechSynthesisAvailableVoicesDidChangeNotification: &'static NSNotificationName;
}

extern_class!(
    /// AVSpeechSynthesisVoice encapsulates the attributes of the voice used to synthesize speech on the system.
    ///
    ///
    /// Retrieve a voice by specifying the language code your text should be spoken in, or by using voiceWithIdentifier
    /// for a known voice identifier.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesisvoice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSpeechSynthesisVoice;
);

unsafe impl Send for AVSpeechSynthesisVoice {}

unsafe impl Sync for AVSpeechSynthesisVoice {}

unsafe impl NSCoding for AVSpeechSynthesisVoice {}

unsafe impl NSObjectProtocol for AVSpeechSynthesisVoice {}

unsafe impl NSSecureCoding for AVSpeechSynthesisVoice {}

extern_methods!(
    unsafe impl AVSpeechSynthesisVoice {
        #[unsafe(method_family(none))]
        #[method_id(speechVoices)]
        pub unsafe fn speechVoices() -> Retained<NSArray<AVSpeechSynthesisVoice>>;

        #[unsafe(method_family(none))]
        #[method_id(currentLanguageCode)]
        pub unsafe fn currentLanguageCode() -> Retained<NSString>;

        /// Use a BCP-47 language tag to specify the desired language and region.
        ///
        /// Parameter `languageCode`: Specifies the BCP-47 language tag that represents the voice.
        ///
        /// The default is the system's region and language.
        /// Passing in nil will return the default voice.
        /// Passing in an invalid languageCode will return nil.
        /// Will return enhanced quality voice if available, default quality otherwise.
        /// Examples: en-US (U.S. English), fr-CA (French Canadian)
        #[unsafe(method_family(none))]
        #[method_id(voiceWithLanguage:)]
        pub unsafe fn voiceWithLanguage(
            language_code: Option<&NSString>,
        ) -> Option<Retained<AVSpeechSynthesisVoice>>;

        /// Retrieve a voice by its identifier.
        ///
        /// Parameter `identifier`: A unique identifier for a voice.
        ///
        /// Passing in an invalid identifier will return nil.
        /// Returns nil if the identifier is valid, but the voice is not available on device (i.e. not yet downloaded by the user).
        #[unsafe(method_family(none))]
        #[method_id(voiceWithIdentifier:)]
        pub unsafe fn voiceWithIdentifier(
            identifier: &NSString,
        ) -> Option<Retained<AVSpeechSynthesisVoice>>;

        #[unsafe(method_family(none))]
        #[method_id(language)]
        pub unsafe fn language(&self) -> Retained<NSString>;

        #[unsafe(method_family(none))]
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(quality)]
        pub unsafe fn quality(&self) -> AVSpeechSynthesisVoiceQuality;

        #[method(gender)]
        pub unsafe fn gender(&self) -> AVSpeechSynthesisVoiceGender;

        #[unsafe(method_family(none))]
        #[method_id(audioFileSettings)]
        pub unsafe fn audioFileSettings(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[method(voiceTraits)]
        pub unsafe fn voiceTraits(&self) -> AVSpeechSynthesisVoiceTraits;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVSpeechSynthesisVoice {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// AVSpeechUtterance is the atom of speaking a string or pausing the synthesizer.
    ///
    ///
    /// To start speaking, specify the AVSpeechSynthesisVoice and the string to be spoken, then optionally change the rate, pitch or volume if desired.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechutterance?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSpeechUtterance;
);

unsafe impl NSCoding for AVSpeechUtterance {}

unsafe impl NSCopying for AVSpeechUtterance {}

unsafe impl CopyingHelper for AVSpeechUtterance {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVSpeechUtterance {}

unsafe impl NSSecureCoding for AVSpeechUtterance {}

extern_methods!(
    unsafe impl AVSpeechUtterance {
        #[unsafe(method_family(none))]
        #[method_id(speechUtteranceWithString:)]
        pub unsafe fn speechUtteranceWithString(string: &NSString) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(speechUtteranceWithAttributedString:)]
        pub unsafe fn speechUtteranceWithAttributedString(
            string: &NSAttributedString,
        ) -> Retained<Self>;

        /// A speech utterance that expects markup written using the Speech Synthesis Markup Language (SSML) standard.
        /// Returns nil if invalid SSML is passed in.
        #[unsafe(method_family(none))]
        #[method_id(speechUtteranceWithSSMLRepresentation:)]
        pub unsafe fn speechUtteranceWithSSMLRepresentation(
            string: &NSString,
        ) -> Option<Retained<Self>>;

        #[unsafe(method_family(init))]
        #[method_id(initWithString:)]
        pub unsafe fn initWithString(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            this: Allocated<Self>,
            string: &NSAttributedString,
        ) -> Retained<Self>;

        /// A speech utterance that expects markup written using the Speech Synthesis Markup Language (SSML)  standard.
        ///
        ///
        /// Uses SSML markup to add attributes. If using SSML to request voices that fall under certain attributes, a single
        /// utterance may be split into multiple parts, each sent to the appropriate synthesizer. If no voice matches the properties,
        /// the voice in the
        /// `voice`property of the utterance will be used. If no
        /// `voice`is specified, the system's default
        /// will be used.
        /// `AVSpeechUtterance`properties that affect the prosidy of a voice such as
        /// `rate,``pitchMultiplier,``pitchMultiplier`will not apply to an utterance that uses an SSML representation.
        ///
        /// Returns nil if invalid SSML is passed in.
        #[unsafe(method_family(init))]
        #[method_id(initWithSSMLRepresentation:)]
        pub unsafe fn initWithSSMLRepresentation(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Option<Retained<Self>>;

        #[unsafe(method_family(none))]
        #[method_id(voice)]
        pub unsafe fn voice(&self) -> Option<Retained<AVSpeechSynthesisVoice>>;

        /// Setter for [`voice`][Self::voice].
        #[method(setVoice:)]
        pub unsafe fn setVoice(&self, voice: Option<&AVSpeechSynthesisVoice>);

        #[unsafe(method_family(none))]
        #[method_id(speechString)]
        pub unsafe fn speechString(&self) -> Retained<NSString>;

        #[unsafe(method_family(none))]
        #[method_id(attributedSpeechString)]
        pub unsafe fn attributedSpeechString(&self) -> Retained<NSAttributedString>;

        #[method(rate)]
        pub unsafe fn rate(&self) -> c_float;

        /// Setter for [`rate`][Self::rate].
        #[method(setRate:)]
        pub unsafe fn setRate(&self, rate: c_float);

        #[method(pitchMultiplier)]
        pub unsafe fn pitchMultiplier(&self) -> c_float;

        /// Setter for [`pitchMultiplier`][Self::pitchMultiplier].
        #[method(setPitchMultiplier:)]
        pub unsafe fn setPitchMultiplier(&self, pitch_multiplier: c_float);

        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        /// Setter for [`volume`][Self::volume].
        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[method(prefersAssistiveTechnologySettings)]
        pub unsafe fn prefersAssistiveTechnologySettings(&self) -> bool;

        /// Setter for [`prefersAssistiveTechnologySettings`][Self::prefersAssistiveTechnologySettings].
        #[method(setPrefersAssistiveTechnologySettings:)]
        pub unsafe fn setPrefersAssistiveTechnologySettings(
            &self,
            prefers_assistive_technology_settings: bool,
        );

        #[method(preUtteranceDelay)]
        pub unsafe fn preUtteranceDelay(&self) -> NSTimeInterval;

        /// Setter for [`preUtteranceDelay`][Self::preUtteranceDelay].
        #[method(setPreUtteranceDelay:)]
        pub unsafe fn setPreUtteranceDelay(&self, pre_utterance_delay: NSTimeInterval);

        #[method(postUtteranceDelay)]
        pub unsafe fn postUtteranceDelay(&self) -> NSTimeInterval;

        /// Setter for [`postUtteranceDelay`][Self::postUtteranceDelay].
        #[method(setPostUtteranceDelay:)]
        pub unsafe fn setPostUtteranceDelay(&self, post_utterance_delay: NSTimeInterval);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVSpeechUtterance {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// AVSpeechSynthesizer allows speaking of speech utterances with a basic queuing mechanism.
    ///
    ///
    /// Create an instance of AVSpeechSynthesizer to start generating synthesized speech by using AVSpeechUtterance objects.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSpeechSynthesizer;
);

unsafe impl NSObjectProtocol for AVSpeechSynthesizer {}

extern_methods!(
    unsafe impl AVSpeechSynthesizer {
        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVSpeechSynthesizerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AVSpeechSynthesizerDelegate>>,
        );

        #[method(isSpeaking)]
        pub unsafe fn isSpeaking(&self) -> bool;

        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        #[method(speakUtterance:)]
        pub unsafe fn speakUtterance(&self, utterance: &AVSpeechUtterance);

        #[cfg(all(feature = "AVAudioBuffer", feature = "block2"))]
        #[method(writeUtterance:toBufferCallback:)]
        pub unsafe fn writeUtterance_toBufferCallback(
            &self,
            utterance: &AVSpeechUtterance,
            buffer_callback: AVSpeechSynthesizerBufferCallback,
        );

        #[cfg(all(feature = "AVAudioBuffer", feature = "block2"))]
        /// Use this method to receive audio buffers and associated metadata that can be used to store or further process synthesized speech.
        /// The dictionary provided by -[AVSpeechSynthesisVoice audioFileSettings] can be used to create an AVAudioFile.
        #[method(writeUtterance:toBufferCallback:toMarkerCallback:)]
        pub unsafe fn writeUtterance_toBufferCallback_toMarkerCallback(
            &self,
            utterance: &AVSpeechUtterance,
            buffer_callback: AVSpeechSynthesizerBufferCallback,
            marker_callback: AVSpeechSynthesizerMarkerCallback,
        );

        #[method(stopSpeakingAtBoundary:)]
        pub unsafe fn stopSpeakingAtBoundary(&self, boundary: AVSpeechBoundary) -> bool;

        #[method(pauseSpeakingAtBoundary:)]
        pub unsafe fn pauseSpeakingAtBoundary(&self, boundary: AVSpeechBoundary) -> bool;

        #[method(continueSpeaking)]
        pub unsafe fn continueSpeaking(&self) -> bool;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[unsafe(method_family(none))]
        #[method_id(outputChannels)]
        pub unsafe fn outputChannels(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionChannelDescription>>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        /// Setter for [`outputChannels`][Self::outputChannels].
        #[method(setOutputChannels:)]
        pub unsafe fn setOutputChannels(
            &self,
            output_channels: Option<&NSArray<AVAudioSessionChannelDescription>>,
        );

        #[method(usesApplicationAudioSession)]
        pub unsafe fn usesApplicationAudioSession(&self) -> bool;

        /// Setter for [`usesApplicationAudioSession`][Self::usesApplicationAudioSession].
        #[method(setUsesApplicationAudioSession:)]
        pub unsafe fn setUsesApplicationAudioSession(&self, uses_application_audio_session: bool);

        #[method(mixToTelephonyUplink)]
        pub unsafe fn mixToTelephonyUplink(&self) -> bool;

        /// Setter for [`mixToTelephonyUplink`][Self::mixToTelephonyUplink].
        #[method(setMixToTelephonyUplink:)]
        pub unsafe fn setMixToTelephonyUplink(&self, mix_to_telephony_uplink: bool);

        #[cfg(feature = "block2")]
        /// Asks the user to allow your app to use personal voices for speech synthesis
        ///
        /// Call this method before performing any other tasks associated with speech synthesis using personal voices. This method executes asynchronously, returning shortly after you call it. At some point later, the system calls the provided handler block with the results.
        ///
        /// When your app's authorization status is PersonalVoiceAuthorizationStatus.notDetermined, this method causes the system to prompt the user to grant or deny permission for your app to use personal voices. The user's response is saved so that future calls to this method do not prompt the user again.
        #[method(requestPersonalVoiceAuthorizationWithCompletionHandler:)]
        pub unsafe fn requestPersonalVoiceAuthorizationWithCompletionHandler(
            handler: &block2::Block<dyn Fn(AVSpeechSynthesisPersonalVoiceAuthorizationStatus)>,
        );

        /// Returns your app's current authorization to use personal voices.
        ///
        ///
        /// The user can reject your app's request to use personal voices, but your request can also be denied if personal voices are not supported on the device. The app can also change your app's authorization status at any time from the Settings app.
        ///
        ///
        /// Returns: The app's current authorization status value. For a list of values, see AVSpeechSynthesisPersonalVoiceAuthorizationStatus.
        #[method(personalVoiceAuthorizationStatus)]
        pub unsafe fn personalVoiceAuthorizationStatus(
        ) -> AVSpeechSynthesisPersonalVoiceAuthorizationStatus;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVSpeechSynthesizer {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// Defines an interface for delegates of AVSpeechSynthesizer to receive notifications of important speech utterance events.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesizerdelegate?language=objc)
    pub unsafe trait AVSpeechSynthesizerDelegate: NSObjectProtocol {
        #[optional]
        #[method(speechSynthesizer:didStartSpeechUtterance:)]
        unsafe fn speechSynthesizer_didStartSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:didFinishSpeechUtterance:)]
        unsafe fn speechSynthesizer_didFinishSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:didPauseSpeechUtterance:)]
        unsafe fn speechSynthesizer_didPauseSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:didContinueSpeechUtterance:)]
        unsafe fn speechSynthesizer_didContinueSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:didCancelSpeechUtterance:)]
        unsafe fn speechSynthesizer_didCancelSpeechUtterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:willSpeakRangeOfSpeechString:utterance:)]
        unsafe fn speechSynthesizer_willSpeakRangeOfSpeechString_utterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            character_range: NSRange,
            utterance: &AVSpeechUtterance,
        );

        #[optional]
        #[method(speechSynthesizer:willSpeakMarker:utterance:)]
        unsafe fn speechSynthesizer_willSpeakMarker_utterance(
            &self,
            synthesizer: &AVSpeechSynthesizer,
            marker: &AVSpeechSynthesisMarker,
            utterance: &AVSpeechUtterance,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avspeechsynthesismarker?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSpeechSynthesisMarker;
);

unsafe impl Send for AVSpeechSynthesisMarker {}

unsafe impl Sync for AVSpeechSynthesisMarker {}

unsafe impl NSCoding for AVSpeechSynthesisMarker {}

unsafe impl NSCopying for AVSpeechSynthesisMarker {}

unsafe impl CopyingHelper for AVSpeechSynthesisMarker {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVSpeechSynthesisMarker {}

unsafe impl NSSecureCoding for AVSpeechSynthesisMarker {}

extern_methods!(
    unsafe impl AVSpeechSynthesisMarker {
        #[method(mark)]
        pub unsafe fn mark(&self) -> AVSpeechSynthesisMarkerMark;

        /// Setter for [`mark`][Self::mark].
        #[method(setMark:)]
        pub unsafe fn setMark(&self, mark: AVSpeechSynthesisMarkerMark);

        /// Byte offset into the associated audio buffer
        #[method(byteSampleOffset)]
        pub unsafe fn byteSampleOffset(&self) -> NSUInteger;

        /// Setter for [`byteSampleOffset`][Self::byteSampleOffset].
        #[method(setByteSampleOffset:)]
        pub unsafe fn setByteSampleOffset(&self, byte_sample_offset: NSUInteger);

        /// The location and length of the pertaining speech request's SSML text. This marker applies to the range of characters represented by the NSString.
        #[method(textRange)]
        pub unsafe fn textRange(&self) -> NSRange;

        /// Setter for [`textRange`][Self::textRange].
        #[method(setTextRange:)]
        pub unsafe fn setTextRange(&self, text_range: NSRange);

        #[unsafe(method_family(none))]
        #[method_id(bookmarkName)]
        pub unsafe fn bookmarkName(&self) -> Retained<NSString>;

        /// Setter for [`bookmarkName`][Self::bookmarkName].
        #[method(setBookmarkName:)]
        pub unsafe fn setBookmarkName(&self, bookmark_name: &NSString);

        #[unsafe(method_family(none))]
        #[method_id(phoneme)]
        pub unsafe fn phoneme(&self) -> Retained<NSString>;

        /// Setter for [`phoneme`][Self::phoneme].
        #[method(setPhoneme:)]
        pub unsafe fn setPhoneme(&self, phoneme: &NSString);

        #[unsafe(method_family(init))]
        #[method_id(initWithMarkerType:forTextRange:atByteSampleOffset:)]
        pub unsafe fn initWithMarkerType_forTextRange_atByteSampleOffset(
            this: Allocated<Self>,
            r#type: AVSpeechSynthesisMarkerMark,
            range: NSRange,
            byte_sample_offset: NSUInteger,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithWordRange:atByteSampleOffset:)]
        pub unsafe fn initWithWordRange_atByteSampleOffset(
            this: Allocated<Self>,
            range: NSRange,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithSentenceRange:atByteSampleOffset:)]
        pub unsafe fn initWithSentenceRange_atByteSampleOffset(
            this: Allocated<Self>,
            range: NSRange,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithParagraphRange:atByteSampleOffset:)]
        pub unsafe fn initWithParagraphRange_atByteSampleOffset(
            this: Allocated<Self>,
            range: NSRange,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithPhonemeString:atByteSampleOffset:)]
        pub unsafe fn initWithPhonemeString_atByteSampleOffset(
            this: Allocated<Self>,
            phoneme: &NSString,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithBookmarkName:atByteSampleOffset:)]
        pub unsafe fn initWithBookmarkName_atByteSampleOffset(
            this: Allocated<Self>,
            mark: &NSString,
            byte_sample_offset: NSInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVSpeechSynthesisMarker {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
