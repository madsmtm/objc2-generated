//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdllighttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MDLLightType(pub NSUInteger);
impl MDLLightType {
    #[doc(alias = "MDLLightTypeUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "MDLLightTypeAmbient")]
    pub const Ambient: Self = Self(1);
    #[doc(alias = "MDLLightTypeDirectional")]
    pub const Directional: Self = Self(2);
    #[doc(alias = "MDLLightTypeSpot")]
    pub const Spot: Self = Self(3);
    #[doc(alias = "MDLLightTypePoint")]
    pub const Point: Self = Self(4);
    #[doc(alias = "MDLLightTypeLinear")]
    pub const Linear: Self = Self(5);
    #[doc(alias = "MDLLightTypeDiscArea")]
    pub const DiscArea: Self = Self(6);
    #[doc(alias = "MDLLightTypeRectangularArea")]
    pub const RectangularArea: Self = Self(7);
    #[doc(alias = "MDLLightTypeSuperElliptical")]
    pub const SuperElliptical: Self = Self(8);
    #[doc(alias = "MDLLightTypePhotometric")]
    pub const Photometric: Self = Self(9);
    #[doc(alias = "MDLLightTypeProbe")]
    pub const Probe: Self = Self(10);
    #[doc(alias = "MDLLightTypeEnvironment")]
    pub const Environment: Self = Self(11);
}

unsafe impl Encode for MDLLightType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MDLLightType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdllight?language=objc)
    #[unsafe(super(MDLObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MDLObject")]
    pub struct MDLLight;
);

#[cfg(all(feature = "MDLObject", feature = "MDLTypes"))]
extern_conformance!(
    unsafe impl MDLNamed for MDLLight {}
);

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MDLLight {}
);

#[cfg(feature = "MDLObject")]
impl MDLLight {
    extern_methods!(
        #[unsafe(method(lightType))]
        #[unsafe(method_family = none)]
        pub unsafe fn lightType(&self) -> MDLLightType;

        /// Setter for [`lightType`][Self::lightType].
        #[unsafe(method(setLightType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLightType(&self, light_type: MDLLightType);

        #[unsafe(method(colorSpace))]
        #[unsafe(method_family = none)]
        pub unsafe fn colorSpace(&self) -> Retained<NSString>;

        /// Setter for [`colorSpace`][Self::colorSpace].
        #[unsafe(method(setColorSpace:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setColorSpace(&self, color_space: &NSString);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MDLObject")]
impl MDLLight {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// A light with characteristics representing plausible real world lights
    ///
    ///
    ///
    ///
    ///
    /// quadratically attenuates to zero.
    ///
    /// light is maximally bright.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlphysicallyplausiblelight?language=objc)
    #[unsafe(super(MDLLight, MDLObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MDLObject")]
    pub struct MDLPhysicallyPlausibleLight;
);

#[cfg(all(feature = "MDLObject", feature = "MDLTypes"))]
extern_conformance!(
    unsafe impl MDLNamed for MDLPhysicallyPlausibleLight {}
);

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MDLPhysicallyPlausibleLight {}
);

#[cfg(feature = "MDLObject")]
impl MDLPhysicallyPlausibleLight {
    extern_methods!(
        /// Light color specified by color temperature, in degrees Kelvin
        ///
        /// default color is 6500K, cool daylight.
        #[unsafe(method(setColorByTemperature:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setColorByTemperature(&self, temperature: c_float);

        #[cfg(feature = "objc2-core-graphics")]
        #[unsafe(method(color))]
        #[unsafe(method_family = none)]
        pub unsafe fn color(&self) -> Option<Retained<CGColor>>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`color`][Self::color].
        #[unsafe(method(setColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setColor(&self, color: Option<&CGColor>);

        #[unsafe(method(lumens))]
        #[unsafe(method_family = none)]
        pub unsafe fn lumens(&self) -> c_float;

        /// Setter for [`lumens`][Self::lumens].
        #[unsafe(method(setLumens:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLumens(&self, lumens: c_float);

        #[unsafe(method(innerConeAngle))]
        #[unsafe(method_family = none)]
        pub unsafe fn innerConeAngle(&self) -> c_float;

        /// Setter for [`innerConeAngle`][Self::innerConeAngle].
        #[unsafe(method(setInnerConeAngle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInnerConeAngle(&self, inner_cone_angle: c_float);

        #[unsafe(method(outerConeAngle))]
        #[unsafe(method_family = none)]
        pub unsafe fn outerConeAngle(&self) -> c_float;

        /// Setter for [`outerConeAngle`][Self::outerConeAngle].
        #[unsafe(method(setOuterConeAngle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOuterConeAngle(&self, outer_cone_angle: c_float);

        #[unsafe(method(attenuationStartDistance))]
        #[unsafe(method_family = none)]
        pub unsafe fn attenuationStartDistance(&self) -> c_float;

        /// Setter for [`attenuationStartDistance`][Self::attenuationStartDistance].
        #[unsafe(method(setAttenuationStartDistance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttenuationStartDistance(&self, attenuation_start_distance: c_float);

        #[unsafe(method(attenuationEndDistance))]
        #[unsafe(method_family = none)]
        pub unsafe fn attenuationEndDistance(&self) -> c_float;

        /// Setter for [`attenuationEndDistance`][Self::attenuationEndDistance].
        #[unsafe(method(setAttenuationEndDistance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttenuationEndDistance(&self, attenuation_end_distance: c_float);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MDLObject")]
impl MDLPhysicallyPlausibleLight {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlarealight?language=objc)
    #[unsafe(super(MDLPhysicallyPlausibleLight, MDLLight, MDLObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MDLObject")]
    pub struct MDLAreaLight;
);

#[cfg(all(feature = "MDLObject", feature = "MDLTypes"))]
extern_conformance!(
    unsafe impl MDLNamed for MDLAreaLight {}
);

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MDLAreaLight {}
);

#[cfg(feature = "MDLObject")]
impl MDLAreaLight {
    extern_methods!(
        #[unsafe(method(areaRadius))]
        #[unsafe(method_family = none)]
        pub unsafe fn areaRadius(&self) -> c_float;

        /// Setter for [`areaRadius`][Self::areaRadius].
        #[unsafe(method(setAreaRadius:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAreaRadius(&self, area_radius: c_float);

        #[unsafe(method(aspect))]
        #[unsafe(method_family = none)]
        pub unsafe fn aspect(&self) -> c_float;

        /// Setter for [`aspect`][Self::aspect].
        #[unsafe(method(setAspect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAspect(&self, aspect: c_float);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MDLObject")]
impl MDLAreaLight {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// A light created from measurements at various angles.
    ///
    ///
    /// learn the intensity of the light in that direction.
    ///
    /// used to calculate the spherical harmonics coefficients
    ///
    /// calculated by generateSphericalHarmonicsFromLight:
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlphotometriclight?language=objc)
    #[unsafe(super(MDLPhysicallyPlausibleLight, MDLLight, MDLObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MDLObject")]
    pub struct MDLPhotometricLight;
);

#[cfg(all(feature = "MDLObject", feature = "MDLTypes"))]
extern_conformance!(
    unsafe impl MDLNamed for MDLPhotometricLight {}
);

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MDLPhotometricLight {}
);

#[cfg(feature = "MDLObject")]
impl MDLPhotometricLight {
    extern_methods!(
        #[unsafe(method(initWithIESProfile:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIESProfile(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(generateSphericalHarmonicsFromLight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn generateSphericalHarmonicsFromLight(
            &self,
            spherical_harmonics_level: NSUInteger,
        );

        #[unsafe(method(generateCubemapFromLight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn generateCubemapFromLight(&self, texture_size: NSUInteger);

        #[cfg(feature = "MDLTexture")]
        /// Generate an IES compliant MDLTexture
        /// 1D when the number of horizontal angles is one and the innerConeAngle is
        /// <
        /// 180
        /// 2D when the previous statement fails and innerConeAngle
        /// <
        /// 89
        /// 3D in all other cases
        /// the parameter textureSize is the size in pixels of the texture image. For a size of N,
        /// 1D generates an Nx1 image, 2D generates an NxN image, 3D generates an Nx(N*6) image (i.e. cubemap).
        #[unsafe(method(generateTexture:))]
        #[unsafe(method_family = none)]
        pub unsafe fn generateTexture(&self, texture_size: NSUInteger) -> Retained<MDLTexture>;

        #[cfg(feature = "MDLTexture")]
        #[unsafe(method(lightCubeMap))]
        #[unsafe(method_family = none)]
        pub unsafe fn lightCubeMap(&self) -> Option<Retained<MDLTexture>>;

        #[unsafe(method(sphericalHarmonicsLevel))]
        #[unsafe(method_family = none)]
        pub unsafe fn sphericalHarmonicsLevel(&self) -> NSUInteger;

        #[unsafe(method(sphericalHarmonicsCoefficients))]
        #[unsafe(method_family = none)]
        pub unsafe fn sphericalHarmonicsCoefficients(&self) -> Option<Retained<NSData>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MDLObject")]
impl MDLPhotometricLight {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdllightprobe?language=objc)
    #[unsafe(super(MDLLight, MDLObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MDLObject")]
    pub struct MDLLightProbe;
);

#[cfg(all(feature = "MDLObject", feature = "MDLTypes"))]
extern_conformance!(
    unsafe impl MDLNamed for MDLLightProbe {}
);

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MDLLightProbe {}
);

#[cfg(feature = "MDLObject")]
impl MDLLightProbe {
    extern_methods!(
        #[cfg(feature = "MDLTexture")]
        #[unsafe(method(initWithReflectiveTexture:irradianceTexture:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithReflectiveTexture_irradianceTexture(
            this: Allocated<Self>,
            reflective_texture: Option<&MDLTexture>,
            irradiance_texture: Option<&MDLTexture>,
        ) -> Retained<Self>;

        #[unsafe(method(generateSphericalHarmonicsFromIrradiance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn generateSphericalHarmonicsFromIrradiance(
            &self,
            spherical_harmonics_level: NSUInteger,
        );

        #[cfg(feature = "MDLTexture")]
        #[unsafe(method(reflectiveTexture))]
        #[unsafe(method_family = none)]
        pub unsafe fn reflectiveTexture(&self) -> Option<Retained<MDLTexture>>;

        #[cfg(feature = "MDLTexture")]
        #[unsafe(method(irradianceTexture))]
        #[unsafe(method_family = none)]
        pub unsafe fn irradianceTexture(&self) -> Option<Retained<MDLTexture>>;

        #[unsafe(method(sphericalHarmonicsLevel))]
        #[unsafe(method_family = none)]
        pub unsafe fn sphericalHarmonicsLevel(&self) -> NSUInteger;

        #[unsafe(method(sphericalHarmonicsCoefficients))]
        #[unsafe(method_family = none)]
        pub unsafe fn sphericalHarmonicsCoefficients(&self) -> Option<Retained<NSData>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MDLObject")]
impl MDLLightProbe {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// MDLLightBaking.
#[cfg(feature = "MDLObject")]
impl MDLLightProbe {
    extern_methods!(
        #[cfg(all(feature = "MDLTexture", feature = "MDLTransform"))]
        #[unsafe(method(lightProbeWithTextureSize:forLocation:lightsToConsider:objectsToConsider:reflectiveCubemap:irradianceCubemap:))]
        #[unsafe(method_family = none)]
        pub unsafe fn lightProbeWithTextureSize_forLocation_lightsToConsider_objectsToConsider_reflectiveCubemap_irradianceCubemap(
            texture_size: NSInteger,
            transform: &MDLTransform,
            lights_to_consider: &NSArray<MDLLight>,
            objects_to_consider: &NSArray<MDLObject>,
            reflective_cubemap: Option<&MDLTexture>,
            irradiance_cubemap: Option<&MDLTexture>,
        ) -> Option<Retained<MDLLightProbe>>;
    );
}
