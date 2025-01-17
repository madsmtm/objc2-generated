//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Generic feature found by a CIDetector.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreimage/cifeature?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIFeature;
);

unsafe impl NSObjectProtocol for CIFeature {}

extern_methods!(
    unsafe impl CIFeature {
        /// The type of the feature.
        #[unsafe(method_family(none))]
        #[method_id(type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-core-foundation")]
        /// The bounds of the feature in the image it was detected in.
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIFeature {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// Specifies the type of a feature that is a face.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreimage/cifeaturetypeface?language=objc)
    pub static CIFeatureTypeFace: &'static NSString;
}

extern "C" {
    /// Specifies the type of a feature that is a rectangle.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreimage/cifeaturetyperectangle?language=objc)
    pub static CIFeatureTypeRectangle: &'static NSString;
}

extern "C" {
    /// Specifies the type of a feature that is a QR code.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreimage/cifeaturetypeqrcode?language=objc)
    pub static CIFeatureTypeQRCode: &'static NSString;
}

extern "C" {
    /// Specifies the type of a feature that is a text.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreimage/cifeaturetypetext?language=objc)
    pub static CIFeatureTypeText: &'static NSString;
}

extern_class!(
    /// A face feature found by a CIDetector.
    /// All positions are relative to the original image.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreimage/cifacefeature?language=objc)
    #[unsafe(super(CIFeature, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIFaceFeature;
);

unsafe impl NSObjectProtocol for CIFaceFeature {}

extern_methods!(
    unsafe impl CIFaceFeature {
        #[cfg(feature = "objc2-core-foundation")]
        /// coordinates of various cardinal points within a face.
        ///
        /// Note that the left eye is the eye on the left side of the face
        /// from the observer's perspective. It is not the left eye from
        /// the subject's perspective.
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[method(hasLeftEyePosition)]
        pub unsafe fn hasLeftEyePosition(&self) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(leftEyePosition)]
        pub unsafe fn leftEyePosition(&self) -> CGPoint;

        #[method(hasRightEyePosition)]
        pub unsafe fn hasRightEyePosition(&self) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rightEyePosition)]
        pub unsafe fn rightEyePosition(&self) -> CGPoint;

        #[method(hasMouthPosition)]
        pub unsafe fn hasMouthPosition(&self) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(mouthPosition)]
        pub unsafe fn mouthPosition(&self) -> CGPoint;

        #[method(hasTrackingID)]
        pub unsafe fn hasTrackingID(&self) -> bool;

        #[method(trackingID)]
        pub unsafe fn trackingID(&self) -> c_int;

        #[method(hasTrackingFrameCount)]
        pub unsafe fn hasTrackingFrameCount(&self) -> bool;

        #[method(trackingFrameCount)]
        pub unsafe fn trackingFrameCount(&self) -> c_int;

        #[method(hasFaceAngle)]
        pub unsafe fn hasFaceAngle(&self) -> bool;

        #[method(faceAngle)]
        pub unsafe fn faceAngle(&self) -> c_float;

        #[method(hasSmile)]
        pub unsafe fn hasSmile(&self) -> bool;

        #[method(leftEyeClosed)]
        pub unsafe fn leftEyeClosed(&self) -> bool;

        #[method(rightEyeClosed)]
        pub unsafe fn rightEyeClosed(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIFaceFeature {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A rectangle feature found by a CIDetector
    /// All positions are relative to the original image.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirectanglefeature?language=objc)
    #[unsafe(super(CIFeature, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIRectangleFeature;
);

unsafe impl NSObjectProtocol for CIRectangleFeature {}

extern_methods!(
    unsafe impl CIRectangleFeature {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(topLeft)]
        pub unsafe fn topLeft(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(topRight)]
        pub unsafe fn topRight(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bottomLeft)]
        pub unsafe fn bottomLeft(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bottomRight)]
        pub unsafe fn bottomRight(&self) -> CGPoint;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIRectangleFeature {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciqrcodefeature?language=objc)
    #[unsafe(super(CIFeature, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIQRCodeFeature;
);

unsafe impl NSCoding for CIQRCodeFeature {}

unsafe impl NSCopying for CIQRCodeFeature {}

unsafe impl CopyingHelper for CIQRCodeFeature {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIQRCodeFeature {}

unsafe impl NSSecureCoding for CIQRCodeFeature {}

extern_methods!(
    unsafe impl CIQRCodeFeature {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(topLeft)]
        pub unsafe fn topLeft(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(topRight)]
        pub unsafe fn topRight(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bottomLeft)]
        pub unsafe fn bottomLeft(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bottomRight)]
        pub unsafe fn bottomRight(&self) -> CGPoint;

        #[unsafe(method_family(none))]
        #[method_id(messageString)]
        pub unsafe fn messageString(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "CIBarcodeDescriptor")]
        #[unsafe(method_family(none))]
        #[method_id(symbolDescriptor)]
        pub unsafe fn symbolDescriptor(&self) -> Option<Retained<CIQRCodeDescriptor>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIQRCodeFeature {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/citextfeature?language=objc)
    #[unsafe(super(CIFeature, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CITextFeature;
);

unsafe impl NSObjectProtocol for CITextFeature {}

extern_methods!(
    unsafe impl CITextFeature {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(topLeft)]
        pub unsafe fn topLeft(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(topRight)]
        pub unsafe fn topRight(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bottomLeft)]
        pub unsafe fn bottomLeft(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bottomRight)]
        pub unsafe fn bottomRight(&self) -> CGPoint;

        #[unsafe(method_family(none))]
        #[method_id(subFeatures)]
        pub unsafe fn subFeatures(&self) -> Option<Retained<NSArray>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CITextFeature {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
