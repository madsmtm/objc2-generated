//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
#[cfg(target_vendor = "apple")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorspacemodel?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSColorSpaceModel(pub NSInteger);
impl NSColorSpaceModel {
    #[doc(alias = "NSColorSpaceModelUnknown")]
    pub const Unknown: Self = Self(-1);
    #[doc(alias = "NSColorSpaceModelGray")]
    pub const Gray: Self = Self(0);
    #[doc(alias = "NSColorSpaceModelRGB")]
    pub const RGB: Self = Self(1);
    #[doc(alias = "NSColorSpaceModelCMYK")]
    pub const CMYK: Self = Self(2);
    #[doc(alias = "NSColorSpaceModelLAB")]
    pub const LAB: Self = Self(3);
    #[doc(alias = "NSColorSpaceModelDeviceN")]
    pub const DeviceN: Self = Self(4);
    #[doc(alias = "NSColorSpaceModelIndexed")]
    pub const Indexed: Self = Self(5);
    #[doc(alias = "NSColorSpaceModelPatterned")]
    pub const Patterned: Self = Self(6);
}

unsafe impl Encode for NSColorSpaceModel {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSColorSpaceModel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorspace?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSColorSpace;
);

unsafe impl Send for NSColorSpace {}

unsafe impl Sync for NSColorSpace {}

extern_conformance!(
    unsafe impl NSCoding for NSColorSpace {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSColorSpace {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for NSColorSpace {}
);

impl NSColorSpace {
    extern_methods!(
        #[unsafe(method(initWithICCProfileData:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithICCProfileData(
            this: Allocated<Self>,
            icc_data: &NSData,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(ICCProfileData))]
        #[unsafe(method_family = none)]
        pub unsafe fn ICCProfileData(&self) -> Option<Retained<NSData>>;

        #[unsafe(method(initWithColorSyncProfile:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithColorSyncProfile(
            this: Allocated<Self>,
            prof: NonNull<c_void>,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(colorSyncProfile))]
        #[unsafe(method_family = none)]
        pub unsafe fn colorSyncProfile(&self) -> *mut c_void;

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(initWithCGColorSpace:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCGColorSpace(
            this: Allocated<Self>,
            cg_color_space: &CGColorSpace,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method(CGColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn CGColorSpace(&self) -> Option<Retained<CGColorSpace>>;

        #[unsafe(method(numberOfColorComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn numberOfColorComponents(&self) -> NSInteger;

        #[unsafe(method(colorSpaceModel))]
        #[unsafe(method_family = none)]
        pub unsafe fn colorSpaceModel(&self) -> NSColorSpaceModel;

        #[unsafe(method(localizedName))]
        #[unsafe(method_family = none)]
        pub unsafe fn localizedName(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(sRGBColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn sRGBColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(genericGamma22GrayColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn genericGamma22GrayColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(extendedSRGBColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn extendedSRGBColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(extendedGenericGamma22GrayColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn extendedGenericGamma22GrayColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(displayP3ColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn displayP3ColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(adobeRGB1998ColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn adobeRGB1998ColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(genericRGBColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn genericRGBColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(genericGrayColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn genericGrayColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(genericCMYKColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn genericCMYKColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(deviceRGBColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn deviceRGBColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(deviceGrayColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn deviceGrayColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(deviceCMYKColorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn deviceCMYKColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method(availableColorSpacesWithModel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn availableColorSpacesWithModel(
            model: NSColorSpaceModel,
        ) -> Retained<NSArray<NSColorSpace>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSColorSpace {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsunknowncolorspacemodel?language=objc)
pub static NSUnknownColorSpaceModel: NSColorSpaceModel =
    NSColorSpaceModel(NSColorSpaceModel::Unknown.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgraycolorspacemodel?language=objc)
pub static NSGrayColorSpaceModel: NSColorSpaceModel = NSColorSpaceModel(NSColorSpaceModel::Gray.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrgbcolorspacemodel?language=objc)
pub static NSRGBColorSpaceModel: NSColorSpaceModel = NSColorSpaceModel(NSColorSpaceModel::RGB.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscmykcolorspacemodel?language=objc)
pub static NSCMYKColorSpaceModel: NSColorSpaceModel = NSColorSpaceModel(NSColorSpaceModel::CMYK.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nslabcolorspacemodel?language=objc)
pub static NSLABColorSpaceModel: NSColorSpaceModel = NSColorSpaceModel(NSColorSpaceModel::LAB.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdevicencolorspacemodel?language=objc)
pub static NSDeviceNColorSpaceModel: NSColorSpaceModel =
    NSColorSpaceModel(NSColorSpaceModel::DeviceN.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsindexedcolorspacemodel?language=objc)
pub static NSIndexedColorSpaceModel: NSColorSpaceModel =
    NSColorSpaceModel(NSColorSpaceModel::Indexed.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspatterncolorspacemodel?language=objc)
pub static NSPatternColorSpaceModel: NSColorSpaceModel =
    NSColorSpaceModel(NSColorSpaceModel::Patterned.0);
