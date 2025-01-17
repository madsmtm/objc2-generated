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

unsafe impl NSCoding for NSColorSpace {}

unsafe impl NSObjectProtocol for NSColorSpace {}

unsafe impl NSSecureCoding for NSColorSpace {}

extern_methods!(
    unsafe impl NSColorSpace {
        #[unsafe(method_family(init))]
        #[method_id(initWithICCProfileData:)]
        pub unsafe fn initWithICCProfileData(
            this: Allocated<Self>,
            icc_data: &NSData,
        ) -> Option<Retained<Self>>;

        #[unsafe(method_family(none))]
        #[method_id(ICCProfileData)]
        pub unsafe fn ICCProfileData(&self) -> Option<Retained<NSData>>;

        #[unsafe(method_family(init))]
        #[method_id(initWithColorSyncProfile:)]
        pub unsafe fn initWithColorSyncProfile(
            this: Allocated<Self>,
            prof: NonNull<c_void>,
        ) -> Option<Retained<Self>>;

        #[method(colorSyncProfile)]
        pub unsafe fn colorSyncProfile(&self) -> *mut c_void;

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method_family(init))]
        #[method_id(initWithCGColorSpace:)]
        pub unsafe fn initWithCGColorSpace(
            this: Allocated<Self>,
            cg_color_space: &CGColorSpace,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method_family(none))]
        #[method_id(CGColorSpace)]
        pub unsafe fn CGColorSpace(&self) -> Option<Retained<CGColorSpace>>;

        #[method(numberOfColorComponents)]
        pub unsafe fn numberOfColorComponents(&self) -> NSInteger;

        #[method(colorSpaceModel)]
        pub unsafe fn colorSpaceModel(&self) -> NSColorSpaceModel;

        #[unsafe(method_family(none))]
        #[method_id(localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Retained<NSString>>;

        #[unsafe(method_family(none))]
        #[method_id(sRGBColorSpace)]
        pub unsafe fn sRGBColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(genericGamma22GrayColorSpace)]
        pub unsafe fn genericGamma22GrayColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(extendedSRGBColorSpace)]
        pub unsafe fn extendedSRGBColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(extendedGenericGamma22GrayColorSpace)]
        pub unsafe fn extendedGenericGamma22GrayColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(displayP3ColorSpace)]
        pub unsafe fn displayP3ColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(adobeRGB1998ColorSpace)]
        pub unsafe fn adobeRGB1998ColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(genericRGBColorSpace)]
        pub unsafe fn genericRGBColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(genericGrayColorSpace)]
        pub unsafe fn genericGrayColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(genericCMYKColorSpace)]
        pub unsafe fn genericCMYKColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(deviceRGBColorSpace)]
        pub unsafe fn deviceRGBColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(deviceGrayColorSpace)]
        pub unsafe fn deviceGrayColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(deviceCMYKColorSpace)]
        pub unsafe fn deviceCMYKColorSpace() -> Retained<NSColorSpace>;

        #[unsafe(method_family(none))]
        #[method_id(availableColorSpacesWithModel:)]
        pub unsafe fn availableColorSpacesWithModel(
            model: NSColorSpaceModel,
        ) -> Retained<NSArray<NSColorSpace>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSColorSpace {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

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
