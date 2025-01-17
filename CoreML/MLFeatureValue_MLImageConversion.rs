//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// Options keys passed into the MLFeatureValue construction for image types
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlfeaturevalueimageoption?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type MLFeatureValueImageOption = NSString;

extern "C" {
    /// Key for CGRect describing a crop region of interest of image source in normalized coordinates
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlfeaturevalueimageoptioncroprect?language=objc)
    pub static MLFeatureValueImageOptionCropRect: &'static MLFeatureValueImageOption;
}

extern "C" {
    /// Key for VNImageCropAndScaleOption describing how to crop and scale the image (or region of interest) to the desired size
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlfeaturevalueimageoptioncropandscale?language=objc)
    pub static MLFeatureValueImageOptionCropAndScale: &'static MLFeatureValueImageOption;
}

extern_methods!(
    /// MLImageConversion
    #[cfg(feature = "MLFeatureValue")]
    unsafe impl MLFeatureValue {
        /// Construct image feature value from an image on disk. Orientation is read from Exif if avaiable
        #[unsafe(method_family(none))]
        #[method_id(featureValueWithImageAtURL:pixelsWide:pixelsHigh:pixelFormatType:options:error:_)]
        pub unsafe fn featureValueWithImageAtURL_pixelsWide_pixelsHigh_pixelFormatType_options_error(
            url: &NSURL,
            pixels_wide: NSInteger,
            pixels_high: NSInteger,
            pixel_format_type: OSType,
            options: Option<&NSDictionary<MLFeatureValueImageOption, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "MLImageConstraint")]
        /// Construct image feature value from an image on disk, using a model specified image constraint. Orientation is read from Exif if avaiable
        #[unsafe(method_family(none))]
        #[method_id(featureValueWithImageAtURL:constraint:options:error:_)]
        pub unsafe fn featureValueWithImageAtURL_constraint_options_error(
            url: &NSURL,
            constraint: &MLImageConstraint,
            options: Option<&NSDictionary<MLFeatureValueImageOption, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Construct image feature value from CGImage (orientation is assumed to be kCGImagePropertyOrientationUp)
        #[unsafe(method_family(none))]
        #[method_id(featureValueWithCGImage:pixelsWide:pixelsHigh:pixelFormatType:options:error:_)]
        pub unsafe fn featureValueWithCGImage_pixelsWide_pixelsHigh_pixelFormatType_options_error(
            cg_image: &CGImage,
            pixels_wide: NSInteger,
            pixels_high: NSInteger,
            pixel_format_type: OSType,
            options: Option<&NSDictionary<MLFeatureValueImageOption, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "MLImageConstraint", feature = "objc2-core-graphics"))]
        /// Construct image feature value from CGImage, using the size and type information required by feature description (orientation is assumed to be kCGImagePropertyOrientationUp)
        #[unsafe(method_family(none))]
        #[method_id(featureValueWithCGImage:constraint:options:error:_)]
        pub unsafe fn featureValueWithCGImage_constraint_options_error(
            cg_image: &CGImage,
            constraint: &MLImageConstraint,
            options: Option<&NSDictionary<MLFeatureValueImageOption, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);
