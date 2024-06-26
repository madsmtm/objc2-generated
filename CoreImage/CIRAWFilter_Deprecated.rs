//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type CIRAWFilterOption = NSString;

extern "C" {
    pub static kCIInputAllowDraftModeKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputDecoderVersionKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCISupportedDecoderVersionsKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputBaselineExposureKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputBoostKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputBoostShadowAmountKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputDisableGamutMapKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputNeutralChromaticityXKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputNeutralChromaticityYKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputNeutralTemperatureKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputNeutralTintKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputNeutralLocationKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputScaleFactorKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputIgnoreImageOrientationKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputImageOrientationKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputEnableSharpeningKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputEnableChromaticNoiseTrackingKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputNoiseReductionAmountKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputMoireAmountKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputEnableVendorLensCorrectionKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputLuminanceNoiseReductionAmountKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputColorNoiseReductionAmountKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputNoiseReductionSharpnessAmountKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputNoiseReductionContrastAmountKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputNoiseReductionDetailAmountKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputEnableEDRModeKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputLocalToneMapAmountKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIInputLinearSpaceFilter: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIOutputNativeSizeKey: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIActiveKeys: Option<&'static CIRAWFilterOption>;
}

extern "C" {
    pub static kCIPropertiesKey: Option<&'static CIRAWFilterOption>;
}

extern_methods!(
    /// CIRAWFilter
    #[cfg(feature = "CIFilter")]
    unsafe impl CIFilter {
        #[deprecated = "Use new CIRAWFilter class instead."]
        #[method_id(@__retain_semantics Other filterWithImageURL:options:)]
        pub unsafe fn filterWithImageURL_options(
            url: Option<&NSURL>,
            options: Option<&NSDictionary<CIRAWFilterOption, AnyObject>>,
        ) -> Option<Retained<CIFilter>>;

        #[deprecated = "Use new CIRAWFilter class instead."]
        #[method_id(@__retain_semantics Other filterWithImageData:options:)]
        pub unsafe fn filterWithImageData_options(
            data: Option<&NSData>,
            options: Option<&NSDictionary<CIRAWFilterOption, AnyObject>>,
        ) -> Option<Retained<CIFilter>>;

        #[deprecated = "Use new CIRAWFilter class instead."]
        #[method_id(@__retain_semantics Other supportedRawCameraModels)]
        pub unsafe fn supportedRawCameraModels() -> Option<Retained<NSArray<NSString>>>;
    }
);
