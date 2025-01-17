//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirawdecoderversion?language=objc)
// NS_TYPED_ENUM
pub type CIRAWDecoderVersion = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirawdecoderversionnone?language=objc)
    pub static CIRAWDecoderVersionNone: &'static CIRAWDecoderVersion;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirawdecoderversion8?language=objc)
    pub static CIRAWDecoderVersion8: &'static CIRAWDecoderVersion;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirawdecoderversion8dng?language=objc)
    pub static CIRAWDecoderVersion8DNG: &'static CIRAWDecoderVersion;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirawdecoderversion7?language=objc)
    pub static CIRAWDecoderVersion7: &'static CIRAWDecoderVersion;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirawdecoderversion7dng?language=objc)
    pub static CIRAWDecoderVersion7DNG: &'static CIRAWDecoderVersion;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirawdecoderversion6?language=objc)
    pub static CIRAWDecoderVersion6: &'static CIRAWDecoderVersion;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirawdecoderversion6dng?language=objc)
    pub static CIRAWDecoderVersion6DNG: &'static CIRAWDecoderVersion;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirawfilter?language=objc)
    #[unsafe(super(CIFilter, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CIFilter")]
    pub struct CIRAWFilter;
);

#[cfg(feature = "CIFilter")]
unsafe impl NSCoding for CIRAWFilter {}

#[cfg(feature = "CIFilter")]
unsafe impl NSCopying for CIRAWFilter {}

#[cfg(feature = "CIFilter")]
unsafe impl CopyingHelper for CIRAWFilter {
    type Result = Self;
}

#[cfg(feature = "CIFilter")]
unsafe impl NSObjectProtocol for CIRAWFilter {}

#[cfg(feature = "CIFilter")]
unsafe impl NSSecureCoding for CIRAWFilter {}

extern_methods!(
    #[cfg(feature = "CIFilter")]
    unsafe impl CIRAWFilter {
        #[unsafe(method_family(none))]
        #[method_id(supportedCameraModels)]
        pub unsafe fn supportedCameraModels() -> Retained<NSArray<NSString>>;

        #[unsafe(method_family(none))]
        #[method_id(supportedDecoderVersions)]
        pub unsafe fn supportedDecoderVersions(&self) -> Retained<NSArray<CIRAWDecoderVersion>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(nativeSize)]
        pub unsafe fn nativeSize(&self) -> CGSize;

        #[unsafe(method_family(none))]
        #[method_id(properties)]
        pub unsafe fn properties(&self) -> Retained<NSDictionary>;

        #[method(isDraftModeEnabled)]
        pub unsafe fn isDraftModeEnabled(&self) -> bool;

        /// Setter for [`isDraftModeEnabled`][Self::isDraftModeEnabled].
        #[method(setDraftModeEnabled:)]
        pub unsafe fn setDraftModeEnabled(&self, draft_mode_enabled: bool);

        #[unsafe(method_family(none))]
        #[method_id(decoderVersion)]
        pub unsafe fn decoderVersion(&self) -> Retained<CIRAWDecoderVersion>;

        /// Setter for [`decoderVersion`][Self::decoderVersion].
        #[method(setDecoderVersion:)]
        pub unsafe fn setDecoderVersion(&self, decoder_version: &CIRAWDecoderVersion);

        #[method(scaleFactor)]
        pub unsafe fn scaleFactor(&self) -> c_float;

        /// Setter for [`scaleFactor`][Self::scaleFactor].
        #[method(setScaleFactor:)]
        pub unsafe fn setScaleFactor(&self, scale_factor: c_float);

        #[method(exposure)]
        pub unsafe fn exposure(&self) -> c_float;

        /// Setter for [`exposure`][Self::exposure].
        #[method(setExposure:)]
        pub unsafe fn setExposure(&self, exposure: c_float);

        #[method(baselineExposure)]
        pub unsafe fn baselineExposure(&self) -> c_float;

        /// Setter for [`baselineExposure`][Self::baselineExposure].
        #[method(setBaselineExposure:)]
        pub unsafe fn setBaselineExposure(&self, baseline_exposure: c_float);

        #[method(shadowBias)]
        pub unsafe fn shadowBias(&self) -> c_float;

        /// Setter for [`shadowBias`][Self::shadowBias].
        #[method(setShadowBias:)]
        pub unsafe fn setShadowBias(&self, shadow_bias: c_float);

        #[method(boostAmount)]
        pub unsafe fn boostAmount(&self) -> c_float;

        /// Setter for [`boostAmount`][Self::boostAmount].
        #[method(setBoostAmount:)]
        pub unsafe fn setBoostAmount(&self, boost_amount: c_float);

        #[method(boostShadowAmount)]
        pub unsafe fn boostShadowAmount(&self) -> c_float;

        /// Setter for [`boostShadowAmount`][Self::boostShadowAmount].
        #[method(setBoostShadowAmount:)]
        pub unsafe fn setBoostShadowAmount(&self, boost_shadow_amount: c_float);

        #[method(isGamutMappingEnabled)]
        pub unsafe fn isGamutMappingEnabled(&self) -> bool;

        /// Setter for [`isGamutMappingEnabled`][Self::isGamutMappingEnabled].
        #[method(setGamutMappingEnabled:)]
        pub unsafe fn setGamutMappingEnabled(&self, gamut_mapping_enabled: bool);

        #[method(isLensCorrectionSupported)]
        pub unsafe fn isLensCorrectionSupported(&self) -> bool;

        #[method(isLensCorrectionEnabled)]
        pub unsafe fn isLensCorrectionEnabled(&self) -> bool;

        /// Setter for [`isLensCorrectionEnabled`][Self::isLensCorrectionEnabled].
        #[method(setLensCorrectionEnabled:)]
        pub unsafe fn setLensCorrectionEnabled(&self, lens_correction_enabled: bool);

        #[method(isLuminanceNoiseReductionSupported)]
        pub unsafe fn isLuminanceNoiseReductionSupported(&self) -> bool;

        #[method(luminanceNoiseReductionAmount)]
        pub unsafe fn luminanceNoiseReductionAmount(&self) -> c_float;

        /// Setter for [`luminanceNoiseReductionAmount`][Self::luminanceNoiseReductionAmount].
        #[method(setLuminanceNoiseReductionAmount:)]
        pub unsafe fn setLuminanceNoiseReductionAmount(
            &self,
            luminance_noise_reduction_amount: c_float,
        );

        #[method(isColorNoiseReductionSupported)]
        pub unsafe fn isColorNoiseReductionSupported(&self) -> bool;

        #[method(colorNoiseReductionAmount)]
        pub unsafe fn colorNoiseReductionAmount(&self) -> c_float;

        /// Setter for [`colorNoiseReductionAmount`][Self::colorNoiseReductionAmount].
        #[method(setColorNoiseReductionAmount:)]
        pub unsafe fn setColorNoiseReductionAmount(&self, color_noise_reduction_amount: c_float);

        #[method(isSharpnessSupported)]
        pub unsafe fn isSharpnessSupported(&self) -> bool;

        #[method(sharpnessAmount)]
        pub unsafe fn sharpnessAmount(&self) -> c_float;

        /// Setter for [`sharpnessAmount`][Self::sharpnessAmount].
        #[method(setSharpnessAmount:)]
        pub unsafe fn setSharpnessAmount(&self, sharpness_amount: c_float);

        #[method(isContrastSupported)]
        pub unsafe fn isContrastSupported(&self) -> bool;

        #[method(contrastAmount)]
        pub unsafe fn contrastAmount(&self) -> c_float;

        /// Setter for [`contrastAmount`][Self::contrastAmount].
        #[method(setContrastAmount:)]
        pub unsafe fn setContrastAmount(&self, contrast_amount: c_float);

        #[method(isDetailSupported)]
        pub unsafe fn isDetailSupported(&self) -> bool;

        #[method(detailAmount)]
        pub unsafe fn detailAmount(&self) -> c_float;

        /// Setter for [`detailAmount`][Self::detailAmount].
        #[method(setDetailAmount:)]
        pub unsafe fn setDetailAmount(&self, detail_amount: c_float);

        #[method(isMoireReductionSupported)]
        pub unsafe fn isMoireReductionSupported(&self) -> bool;

        #[method(moireReductionAmount)]
        pub unsafe fn moireReductionAmount(&self) -> c_float;

        /// Setter for [`moireReductionAmount`][Self::moireReductionAmount].
        #[method(setMoireReductionAmount:)]
        pub unsafe fn setMoireReductionAmount(&self, moire_reduction_amount: c_float);

        #[method(isLocalToneMapSupported)]
        pub unsafe fn isLocalToneMapSupported(&self) -> bool;

        #[method(localToneMapAmount)]
        pub unsafe fn localToneMapAmount(&self) -> c_float;

        /// Setter for [`localToneMapAmount`][Self::localToneMapAmount].
        #[method(setLocalToneMapAmount:)]
        pub unsafe fn setLocalToneMapAmount(&self, local_tone_map_amount: c_float);

        #[method(extendedDynamicRangeAmount)]
        pub unsafe fn extendedDynamicRangeAmount(&self) -> c_float;

        /// Setter for [`extendedDynamicRangeAmount`][Self::extendedDynamicRangeAmount].
        #[method(setExtendedDynamicRangeAmount:)]
        pub unsafe fn setExtendedDynamicRangeAmount(&self, extended_dynamic_range_amount: c_float);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(neutralChromaticity)]
        pub unsafe fn neutralChromaticity(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`neutralChromaticity`][Self::neutralChromaticity].
        #[method(setNeutralChromaticity:)]
        pub unsafe fn setNeutralChromaticity(&self, neutral_chromaticity: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(neutralLocation)]
        pub unsafe fn neutralLocation(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`neutralLocation`][Self::neutralLocation].
        #[method(setNeutralLocation:)]
        pub unsafe fn setNeutralLocation(&self, neutral_location: CGPoint);

        #[method(neutralTemperature)]
        pub unsafe fn neutralTemperature(&self) -> c_float;

        /// Setter for [`neutralTemperature`][Self::neutralTemperature].
        #[method(setNeutralTemperature:)]
        pub unsafe fn setNeutralTemperature(&self, neutral_temperature: c_float);

        #[method(neutralTint)]
        pub unsafe fn neutralTint(&self) -> c_float;

        /// Setter for [`neutralTint`][Self::neutralTint].
        #[method(setNeutralTint:)]
        pub unsafe fn setNeutralTint(&self, neutral_tint: c_float);

        #[unsafe(method_family(none))]
        #[method_id(linearSpaceFilter)]
        pub unsafe fn linearSpaceFilter(&self) -> Option<Retained<CIFilter>>;

        /// Setter for [`linearSpaceFilter`][Self::linearSpaceFilter].
        #[method(setLinearSpaceFilter:)]
        pub unsafe fn setLinearSpaceFilter(&self, linear_space_filter: Option<&CIFilter>);

        #[cfg(feature = "CIImage")]
        #[unsafe(method_family(none))]
        #[method_id(previewImage)]
        pub unsafe fn previewImage(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[unsafe(method_family(none))]
        #[method_id(portraitEffectsMatte)]
        pub unsafe fn portraitEffectsMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[unsafe(method_family(none))]
        #[method_id(semanticSegmentationSkinMatte)]
        pub unsafe fn semanticSegmentationSkinMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[unsafe(method_family(none))]
        #[method_id(semanticSegmentationHairMatte)]
        pub unsafe fn semanticSegmentationHairMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[unsafe(method_family(none))]
        #[method_id(semanticSegmentationGlassesMatte)]
        pub unsafe fn semanticSegmentationGlassesMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[unsafe(method_family(none))]
        #[method_id(semanticSegmentationSkyMatte)]
        pub unsafe fn semanticSegmentationSkyMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[unsafe(method_family(none))]
        #[method_id(semanticSegmentationTeethMatte)]
        pub unsafe fn semanticSegmentationTeethMatte(&self) -> Option<Retained<CIImage>>;

        #[unsafe(method_family(none))]
        #[method_id(filterWithImageURL:)]
        pub unsafe fn filterWithImageURL(url: &NSURL) -> Option<Retained<Self>>;

        #[unsafe(method_family(none))]
        #[method_id(filterWithImageData:identifierHint:)]
        pub unsafe fn filterWithImageData_identifierHint(
            data: &NSData,
            identifier_hint: Option<&NSString>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-video")]
        #[unsafe(method_family(none))]
        #[method_id(filterWithCVPixelBuffer:properties:)]
        pub unsafe fn filterWithCVPixelBuffer_properties(
            buffer: &CVPixelBuffer,
            properties: &NSDictionary,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CIFilter")]
    unsafe impl CIRAWFilter {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
