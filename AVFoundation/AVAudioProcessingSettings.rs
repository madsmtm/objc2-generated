//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avaudiotimepitchalgorithm?language=objc)
// NS_TYPED_ENUM
pub type AVAudioTimePitchAlgorithm = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avaudiotimepitchalgorithmlowqualityzerolatency?language=objc)
    pub static AVAudioTimePitchAlgorithmLowQualityZeroLatency:
        Option<&'static AVAudioTimePitchAlgorithm>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avaudiotimepitchalgorithmtimedomain?language=objc)
    pub static AVAudioTimePitchAlgorithmTimeDomain: Option<&'static AVAudioTimePitchAlgorithm>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avaudiotimepitchalgorithmspectral?language=objc)
    pub static AVAudioTimePitchAlgorithmSpectral: Option<&'static AVAudioTimePitchAlgorithm>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avaudiotimepitchalgorithmvarispeed?language=objc)
    pub static AVAudioTimePitchAlgorithmVarispeed: Option<&'static AVAudioTimePitchAlgorithm>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avaudiospatializationformats?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVAudioSpatializationFormats(pub NSUInteger);
bitflags::bitflags! {
    impl AVAudioSpatializationFormats: NSUInteger {
        const AVAudioSpatializationFormatNone = 0;
        const AVAudioSpatializationFormatMonoAndStereo = 0x3;
        const AVAudioSpatializationFormatMultichannel = 0x4;
        const AVAudioSpatializationFormatMonoStereoAndMultichannel = 0x7;
    }
}

unsafe impl Encode for AVAudioSpatializationFormats {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVAudioSpatializationFormats {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
