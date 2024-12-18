//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audioconverterref?language=objc)
pub type AudioConverterRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audioconverterpropertyid?language=objc)
pub type AudioConverterPropertyID = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertyminimuminputbuffersize?language=objc)
pub const kAudioConverterPropertyMinimumInputBufferSize: AudioConverterPropertyID = 0x6d696273;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertyminimumoutputbuffersize?language=objc)
pub const kAudioConverterPropertyMinimumOutputBufferSize: AudioConverterPropertyID = 0x6d6f6273;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertymaximuminputpacketsize?language=objc)
pub const kAudioConverterPropertyMaximumInputPacketSize: AudioConverterPropertyID = 0x78697073;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertymaximumoutputpacketsize?language=objc)
pub const kAudioConverterPropertyMaximumOutputPacketSize: AudioConverterPropertyID = 0x786f7073;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertycalculateinputbuffersize?language=objc)
pub const kAudioConverterPropertyCalculateInputBufferSize: AudioConverterPropertyID = 0x63696273;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertycalculateoutputbuffersize?language=objc)
pub const kAudioConverterPropertyCalculateOutputBufferSize: AudioConverterPropertyID = 0x636f6273;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertyinputcodecparameters?language=objc)
pub const kAudioConverterPropertyInputCodecParameters: AudioConverterPropertyID = 0x69636470;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertyoutputcodecparameters?language=objc)
pub const kAudioConverterPropertyOutputCodecParameters: AudioConverterPropertyID = 0x6f636470;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertersamplerateconvertercomplexity?language=objc)
pub const kAudioConverterSampleRateConverterComplexity: AudioConverterPropertyID = 0x73726361;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertersamplerateconverterquality?language=objc)
pub const kAudioConverterSampleRateConverterQuality: AudioConverterPropertyID = 0x73726371;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertersamplerateconverterinitialphase?language=objc)
pub const kAudioConverterSampleRateConverterInitialPhase: AudioConverterPropertyID = 0x73726370;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertercodecquality?language=objc)
pub const kAudioConverterCodecQuality: AudioConverterPropertyID = 0x63647175;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterprimemethod?language=objc)
pub const kAudioConverterPrimeMethod: AudioConverterPropertyID = 0x70726d6d;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterprimeinfo?language=objc)
pub const kAudioConverterPrimeInfo: AudioConverterPropertyID = 0x7072696d;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterchannelmap?language=objc)
pub const kAudioConverterChannelMap: AudioConverterPropertyID = 0x63686d70;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterdecompressionmagiccookie?language=objc)
pub const kAudioConverterDecompressionMagicCookie: AudioConverterPropertyID = 0x646d6763;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertercompressionmagiccookie?language=objc)
pub const kAudioConverterCompressionMagicCookie: AudioConverterPropertyID = 0x636d6763;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterencodebitrate?language=objc)
pub const kAudioConverterEncodeBitRate: AudioConverterPropertyID = 0x62726174;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterencodeadjustablesamplerate?language=objc)
pub const kAudioConverterEncodeAdjustableSampleRate: AudioConverterPropertyID = 0x616a7372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterinputchannellayout?language=objc)
pub const kAudioConverterInputChannelLayout: AudioConverterPropertyID = 0x69636c20;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverteroutputchannellayout?language=objc)
pub const kAudioConverterOutputChannelLayout: AudioConverterPropertyID = 0x6f636c20;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterapplicableencodebitrates?language=objc)
pub const kAudioConverterApplicableEncodeBitRates: AudioConverterPropertyID = 0x61656272;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverteravailableencodebitrates?language=objc)
pub const kAudioConverterAvailableEncodeBitRates: AudioConverterPropertyID = 0x76656272;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterapplicableencodesamplerates?language=objc)
pub const kAudioConverterApplicableEncodeSampleRates: AudioConverterPropertyID = 0x61657372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverteravailableencodesamplerates?language=objc)
pub const kAudioConverterAvailableEncodeSampleRates: AudioConverterPropertyID = 0x76657372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverteravailableencodechannellayouttags?language=objc)
pub const kAudioConverterAvailableEncodeChannelLayoutTags: AudioConverterPropertyID = 0x6165636c;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertercurrentoutputstreamdescription?language=objc)
pub const kAudioConverterCurrentOutputStreamDescription: AudioConverterPropertyID = 0x61636f64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertercurrentinputstreamdescription?language=objc)
pub const kAudioConverterCurrentInputStreamDescription: AudioConverterPropertyID = 0x61636964;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertysettings?language=objc)
pub const kAudioConverterPropertySettings: AudioConverterPropertyID = 0x61637073;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertybitdepthhint?language=objc)
pub const kAudioConverterPropertyBitDepthHint: AudioConverterPropertyID = 0x61636264;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertyformatlist?language=objc)
pub const kAudioConverterPropertyFormatList: AudioConverterPropertyID = 0x666c7374;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertydithering?language=objc)
pub const kAudioConverterPropertyDithering: AudioConverterPropertyID = 0x64697468;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertyditherbitdepth?language=objc)
pub const kAudioConverterPropertyDitherBitDepth: AudioConverterPropertyID = 0x64626974;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kditheralgorithm_tpdf?language=objc)
pub const kDitherAlgorithm_TPDF: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kditheralgorithm_noiseshaping?language=objc)
pub const kDitherAlgorithm_NoiseShaping: u32 = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterquality_max?language=objc)
pub const kAudioConverterQuality_Max: u32 = 0x7F;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterquality_high?language=objc)
pub const kAudioConverterQuality_High: u32 = 0x60;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterquality_medium?language=objc)
pub const kAudioConverterQuality_Medium: u32 = 0x40;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterquality_low?language=objc)
pub const kAudioConverterQuality_Low: u32 = 0x20;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterquality_min?language=objc)
pub const kAudioConverterQuality_Min: u32 = 0;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertersamplerateconvertercomplexity_linear?language=objc)
pub const kAudioConverterSampleRateConverterComplexity_Linear: u32 = 0x6c696e65;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertersamplerateconvertercomplexity_normal?language=objc)
pub const kAudioConverterSampleRateConverterComplexity_Normal: u32 = 0x6e6f726d;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertersamplerateconvertercomplexity_mastering?language=objc)
pub const kAudioConverterSampleRateConverterComplexity_Mastering: u32 = 0x62617473;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertersamplerateconvertercomplexity_minimumphase?language=objc)
pub const kAudioConverterSampleRateConverterComplexity_MinimumPhase: u32 = 0x6d696e70;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kconverterprimemethod_pre?language=objc)
pub const kConverterPrimeMethod_Pre: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kconverterprimemethod_normal?language=objc)
pub const kConverterPrimeMethod_Normal: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kconverterprimemethod_none?language=objc)
pub const kConverterPrimeMethod_None: u32 = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audioconverterprimeinfo?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioConverterPrimeInfo {
    pub leadingFrames: u32,
    pub trailingFrames: u32,
}

unsafe impl Encode for AudioConverterPrimeInfo {
    const ENCODING: Encoding = Encoding::Struct(
        "AudioConverterPrimeInfo",
        &[<u32>::ENCODING, <u32>::ENCODING],
    );
}

unsafe impl RefEncode for AudioConverterPrimeInfo {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audioconverteroptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AudioConverterOptions(pub u32);
bitflags::bitflags! {
    impl AudioConverterOptions: u32 {
        const kAudioConverterOption_Unbuffered = 1<<16;
    }
}

unsafe impl Encode for AudioConverterOptions {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for AudioConverterOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_formatnotsupported?language=objc)
pub const kAudioConverterErr_FormatNotSupported: OSStatus = 0x666d743f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_operationnotsupported?language=objc)
pub const kAudioConverterErr_OperationNotSupported: OSStatus = 0x6F703F3F;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_propertynotsupported?language=objc)
pub const kAudioConverterErr_PropertyNotSupported: OSStatus = 0x70726f70;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_invalidinputsize?language=objc)
pub const kAudioConverterErr_InvalidInputSize: OSStatus = 0x696e737a;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_invalidoutputsize?language=objc)
pub const kAudioConverterErr_InvalidOutputSize: OSStatus = 0x6f74737a;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_unspecifiederror?language=objc)
pub const kAudioConverterErr_UnspecifiedError: OSStatus = 0x77686174;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_badpropertysizeerror?language=objc)
pub const kAudioConverterErr_BadPropertySizeError: OSStatus = 0x2173697a;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_requirespacketdescriptionserror?language=objc)
pub const kAudioConverterErr_RequiresPacketDescriptionsError: OSStatus = 0x21706b64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_inputsamplerateoutofrange?language=objc)
pub const kAudioConverterErr_InputSampleRateOutOfRange: OSStatus = 0x21697372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertererr_outputsamplerateoutofrange?language=objc)
pub const kAudioConverterErr_OutputSampleRateOutOfRange: OSStatus = 0x216f7372;

extern "C-unwind" {
    #[cfg(feature = "block2")]
    pub fn AudioConverterPrepare(
        in_flags: u32,
        io_reserved: *mut c_void,
        in_completion_block: Option<&block2::Block<dyn Fn(OSStatus)>>,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-audio-types")]
    pub fn AudioConverterNew(
        in_source_format: NonNull<AudioStreamBasicDescription>,
        in_destination_format: NonNull<AudioStreamBasicDescription>,
        out_audio_converter: NonNull<AudioConverterRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-audio-types")]
    pub fn AudioConverterNewSpecific(
        in_source_format: NonNull<AudioStreamBasicDescription>,
        in_destination_format: NonNull<AudioStreamBasicDescription>,
        in_number_class_descriptions: u32,
        in_class_descriptions: NonNull<AudioClassDescription>,
        out_audio_converter: NonNull<AudioConverterRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-audio-types")]
    pub fn AudioConverterNewWithOptions(
        in_source_format: NonNull<AudioStreamBasicDescription>,
        in_destination_format: NonNull<AudioStreamBasicDescription>,
        in_options: AudioConverterOptions,
        out_audio_converter: NonNull<AudioConverterRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioConverterDispose(in_audio_converter: AudioConverterRef) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioConverterReset(in_audio_converter: AudioConverterRef) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioConverterGetPropertyInfo(
        in_audio_converter: AudioConverterRef,
        in_property_id: AudioConverterPropertyID,
        out_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioConverterGetProperty(
        in_audio_converter: AudioConverterRef,
        in_property_id: AudioConverterPropertyID,
        io_property_data_size: NonNull<u32>,
        out_property_data: NonNull<c_void>,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioConverterSetProperty(
        in_audio_converter: AudioConverterRef,
        in_property_id: AudioConverterPropertyID,
        in_property_data_size: u32,
        in_property_data: NonNull<c_void>,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioConverterConvertBuffer(
        in_audio_converter: AudioConverterRef,
        in_input_data_size: u32,
        in_input_data: NonNull<c_void>,
        io_output_data_size: NonNull<u32>,
        out_output_data: NonNull<c_void>,
    ) -> OSStatus;
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audioconvertercomplexinputdataproc?language=objc)
#[cfg(feature = "objc2-core-audio-types")]
pub type AudioConverterComplexInputDataProc = Option<
    unsafe extern "C-unwind" fn(
        AudioConverterRef,
        NonNull<u32>,
        NonNull<AudioBufferList>,
        *mut *mut AudioStreamPacketDescription,
        *mut c_void,
    ) -> OSStatus,
>;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-audio-types")]
    pub fn AudioConverterFillComplexBuffer(
        in_audio_converter: AudioConverterRef,
        in_input_data_proc: AudioConverterComplexInputDataProc,
        in_input_data_proc_user_data: *mut c_void,
        io_output_data_packet_size: NonNull<u32>,
        out_output_data: NonNull<AudioBufferList>,
        out_packet_description: *mut AudioStreamPacketDescription,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-audio-types")]
    pub fn AudioConverterConvertComplexBuffer(
        in_audio_converter: AudioConverterRef,
        in_number_pcm_frames: u32,
        in_input_data: NonNull<AudioBufferList>,
        out_output_data: NonNull<AudioBufferList>,
    ) -> OSStatus;
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertymaximuminputbuffersize?language=objc)
pub const kAudioConverterPropertyMaximumInputBufferSize: AudioConverterPropertyID = 0x78696273;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioconvertersamplerateconverteralgorithm?language=objc)
pub const kAudioConverterSampleRateConverterAlgorithm: AudioConverterPropertyID = 0x73726369;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audioconverterinputdataproc?language=objc)
pub type AudioConverterInputDataProc = Option<
    unsafe extern "C-unwind" fn(
        AudioConverterRef,
        NonNull<u32>,
        NonNull<NonNull<c_void>>,
        *mut c_void,
    ) -> OSStatus,
>;

extern "C-unwind" {
    #[deprecated = "no longer supported"]
    pub fn AudioConverterFillBuffer(
        in_audio_converter: AudioConverterRef,
        in_input_data_proc: AudioConverterInputDataProc,
        in_input_data_proc_user_data: *mut c_void,
        io_output_data_size: NonNull<u32>,
        out_output_data: NonNull<c_void>,
    ) -> OSStatus;
}
