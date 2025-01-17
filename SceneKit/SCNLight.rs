//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// Light Types
///
/// Describes the various light types available.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlighttype?language=objc)
// NS_TYPED_ENUM
pub type SCNLightType = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlighttypeambient?language=objc)
    pub static SCNLightTypeAmbient: &'static SCNLightType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlighttypeomni?language=objc)
    pub static SCNLightTypeOmni: &'static SCNLightType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlighttypedirectional?language=objc)
    pub static SCNLightTypeDirectional: &'static SCNLightType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlighttypespot?language=objc)
    pub static SCNLightTypeSpot: &'static SCNLightType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlighttypeies?language=objc)
    pub static SCNLightTypeIES: &'static SCNLightType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlighttypeprobe?language=objc)
    pub static SCNLightTypeProbe: &'static SCNLightType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlighttypearea?language=objc)
    pub static SCNLightTypeArea: &'static SCNLightType;
}

/// The different modes available to compute shadows.
///
/// When the shadow mode is set to SCNShadowModeForward, shadows are computed while computing the lighting. In this mode only the alpha component of the shadow color is used to alter the lighting contribution.
/// When the shadow mode is set to SCNShadowModeDeferred shadows are applied as a post process. Shadows are blend over the final image and can therefor be of any arbitrary color. However it is most of the time less efficient than SCNShadowModeForward, except when a scene has a lot of overdraw.
/// When the shadow mode is set to SCNShadowModeModulated the light doesn't illuminate the scene anymore, it only casts shadows. Therefore setting the light color has no effect. In this mode gobos act as a shadow projector: the gobo image is modulated with the shadow receiver's fragments. The typical usage is to use an image of a radial gradient (black to white) that is projected under a character (and use the categoryBitMask of the light and nodes to exclude the character from the shadow receiver).
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnshadowmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNShadowMode(pub NSInteger);
impl SCNShadowMode {
    #[doc(alias = "SCNShadowModeForward")]
    pub const Forward: Self = Self(0);
    #[doc(alias = "SCNShadowModeDeferred")]
    pub const Deferred: Self = Self(1);
    #[doc(alias = "SCNShadowModeModulated")]
    pub const Modulated: Self = Self(2);
}

unsafe impl Encode for SCNShadowMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNShadowMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightprobetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNLightProbeType(pub NSInteger);
impl SCNLightProbeType {
    #[doc(alias = "SCNLightProbeTypeIrradiance")]
    pub const Irradiance: Self = Self(0);
    #[doc(alias = "SCNLightProbeTypeRadiance")]
    pub const Radiance: Self = Self(1);
}

unsafe impl Encode for SCNLightProbeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNLightProbeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightprobeupdatetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNLightProbeUpdateType(pub NSInteger);
impl SCNLightProbeUpdateType {
    #[doc(alias = "SCNLightProbeUpdateTypeNever")]
    pub const Never: Self = Self(0);
    #[doc(alias = "SCNLightProbeUpdateTypeRealtime")]
    pub const Realtime: Self = Self(1);
}

unsafe impl Encode for SCNLightProbeUpdateType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNLightProbeUpdateType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlightareatype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNLightAreaType(pub NSInteger);
impl SCNLightAreaType {
    #[doc(alias = "SCNLightAreaTypeRectangle")]
    pub const Rectangle: Self = Self(1);
    #[doc(alias = "SCNLightAreaTypePolygon")]
    pub const Polygon: Self = Self(4);
}

unsafe impl Encode for SCNLightAreaType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNLightAreaType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// SCNLight represents a light that can be attached to a SCNNode.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlight?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNLight;
);

unsafe impl NSCoding for SCNLight {}

unsafe impl NSCopying for SCNLight {}

unsafe impl CopyingHelper for SCNLight {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNLight {}

unsafe impl NSSecureCoding for SCNLight {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNLight {}

extern_methods!(
    unsafe impl SCNLight {
        /// Creates and returns a light instance.
        #[unsafe(method_family(none))]
        #[method_id(light)]
        pub unsafe fn light() -> Retained<Self>;

        /// Specifies the receiver's type.
        ///
        /// Defaults to SCNLightTypeOmni on iOS 8 and later, and on macOS 10.10 and later (otherwise defaults to SCNLightTypeAmbient).
        #[unsafe(method_family(none))]
        #[method_id(type)]
        pub unsafe fn r#type(&self) -> Retained<SCNLightType>;

        /// Setter for [`type`][Self::type].
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: &SCNLightType);

        /// Specifies the receiver's color (NSColor or CGColorRef). Animatable. Defaults to white.
        ///
        /// The initial value is a NSColor. The renderer multiplies the light's color is by the color derived from the light's temperature.
        #[unsafe(method_family(none))]
        #[method_id(color)]
        pub unsafe fn color(&self) -> Retained<AnyObject>;

        /// Setter for [`color`][Self::color].
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &AnyObject);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the receiver's temperature.
        ///
        /// This specifies the temperature of the light in Kelvin. The renderer multiplies the light's color by the color derived from the light's temperature. Defaults to 6500 (pure white). Animatable.
        #[method(temperature)]
        pub unsafe fn temperature(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`temperature`][Self::temperature].
        #[method(setTemperature:)]
        pub unsafe fn setTemperature(&self, temperature: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the receiver's intensity.
        ///
        /// This intensity is used to modulate the light color. When used with a physically-based material, this corresponds to the luminous flux of the light, expressed in lumens (lm). Defaults to 1000. Animatable.
        #[method(intensity)]
        pub unsafe fn intensity(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`intensity`][Self::intensity].
        #[method(setIntensity:)]
        pub unsafe fn setIntensity(&self, intensity: CGFloat);

        /// Determines the name of the receiver.
        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        /// Setter for [`name`][Self::name].
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        /// Determines whether the receiver casts a shadow. Defaults to NO.
        ///
        /// Shadows are only supported by spot and directional lights.
        #[method(castsShadow)]
        pub unsafe fn castsShadow(&self) -> bool;

        /// Setter for [`castsShadow`][Self::castsShadow].
        #[method(setCastsShadow:)]
        pub unsafe fn setCastsShadow(&self, casts_shadow: bool);

        /// Specifies the color (CGColorRef or NSColor) of the shadow casted by the receiver. Defaults to black. Animatable.
        ///
        /// On iOS 9 or earlier and macOS 10.11 or earlier, this defaults to black 50% transparent.
        #[unsafe(method_family(none))]
        #[method_id(shadowColor)]
        pub unsafe fn shadowColor(&self) -> Retained<AnyObject>;

        /// Setter for [`shadowColor`][Self::shadowColor].
        #[method(setShadowColor:)]
        pub unsafe fn setShadowColor(&self, shadow_color: &AnyObject);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the sample radius used to render the receiver’s shadow. Default value is 3.0. Animatable.
        #[method(shadowRadius)]
        pub unsafe fn shadowRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`shadowRadius`][Self::shadowRadius].
        #[method(setShadowRadius:)]
        pub unsafe fn setShadowRadius(&self, shadow_radius: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the size of the shadow map.
        ///
        /// The larger the shadow map is the more precise the shadows are but the slower the computation is. If set to {0,0} the size of the shadow map is automatically chosen. Defaults to {0,0}.
        #[method(shadowMapSize)]
        pub unsafe fn shadowMapSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`shadowMapSize`][Self::shadowMapSize].
        #[method(setShadowMapSize:)]
        pub unsafe fn setShadowMapSize(&self, shadow_map_size: CGSize);

        /// Specifies the number of sample per fragment to compute the shadow map. Defaults to 0.
        ///
        /// On macOS 10.11 or earlier, the shadowSampleCount defaults to 16. On iOS 9 or earlier it defaults to 1.0.
        /// On macOS 10.12, iOS 10 and greater, when the shadowSampleCount is set to 0, a default sample count is chosen depending on the platform.
        #[method(shadowSampleCount)]
        pub unsafe fn shadowSampleCount(&self) -> NSUInteger;

        /// Setter for [`shadowSampleCount`][Self::shadowSampleCount].
        #[method(setShadowSampleCount:)]
        pub unsafe fn setShadowSampleCount(&self, shadow_sample_count: NSUInteger);

        /// Specified the mode to use to cast shadows. See above for the available modes and their description. Defaults to SCNShadowModeDefered on 10.9 and before, defaults to SCNShadowModeForward otherwise.
        #[method(shadowMode)]
        pub unsafe fn shadowMode(&self) -> SCNShadowMode;

        /// Setter for [`shadowMode`][Self::shadowMode].
        #[method(setShadowMode:)]
        pub unsafe fn setShadowMode(&self, shadow_mode: SCNShadowMode);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the correction to apply to the shadow map to correct acne artefacts. It is multiplied by an implementation-specific value to create a constant depth offset. Defaults to 1.0
        #[method(shadowBias)]
        pub unsafe fn shadowBias(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`shadowBias`][Self::shadowBias].
        #[method(setShadowBias:)]
        pub unsafe fn setShadowBias(&self, shadow_bias: CGFloat);

        /// Specifies if the shadow map projection should be done automatically or manually by the user. Defaults to YES.
        #[method(automaticallyAdjustsShadowProjection)]
        pub unsafe fn automaticallyAdjustsShadowProjection(&self) -> bool;

        /// Setter for [`automaticallyAdjustsShadowProjection`][Self::automaticallyAdjustsShadowProjection].
        #[method(setAutomaticallyAdjustsShadowProjection:)]
        pub unsafe fn setAutomaticallyAdjustsShadowProjection(
            &self,
            automatically_adjusts_shadow_projection: bool,
        );

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the maximum distance from the viewpoint from which the shadows for the receiver light won't be computed. Defaults to 100.0.
        #[method(maximumShadowDistance)]
        pub unsafe fn maximumShadowDistance(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maximumShadowDistance`][Self::maximumShadowDistance].
        #[method(setMaximumShadowDistance:)]
        pub unsafe fn setMaximumShadowDistance(&self, maximum_shadow_distance: CGFloat);

        /// Render only back faces of the shadow caster when enabled. Defaults to NO.
        /// This is a behavior change from previous releases.
        #[method(forcesBackFaceCasters)]
        pub unsafe fn forcesBackFaceCasters(&self) -> bool;

        /// Setter for [`forcesBackFaceCasters`][Self::forcesBackFaceCasters].
        #[method(setForcesBackFaceCasters:)]
        pub unsafe fn setForcesBackFaceCasters(&self, forces_back_face_casters: bool);

        /// Use the sample distribution of the main rendering to better fit the shadow frusta. Defaults to NO.
        #[method(sampleDistributedShadowMaps)]
        pub unsafe fn sampleDistributedShadowMaps(&self) -> bool;

        /// Setter for [`sampleDistributedShadowMaps`][Self::sampleDistributedShadowMaps].
        #[method(setSampleDistributedShadowMaps:)]
        pub unsafe fn setSampleDistributedShadowMaps(&self, sample_distributed_shadow_maps: bool);

        /// Specifies the number of distinct shadow maps that will be computed for the receiver light. Defaults to 1. Maximum is 4.
        #[method(shadowCascadeCount)]
        pub unsafe fn shadowCascadeCount(&self) -> NSUInteger;

        /// Setter for [`shadowCascadeCount`][Self::shadowCascadeCount].
        #[method(setShadowCascadeCount:)]
        pub unsafe fn setShadowCascadeCount(&self, shadow_cascade_count: NSUInteger);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies a factor to interpolate between linear splitting (0) and logarithmic splitting (1). Defaults to 0.15.
        #[method(shadowCascadeSplittingFactor)]
        pub unsafe fn shadowCascadeSplittingFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`shadowCascadeSplittingFactor`][Self::shadowCascadeSplittingFactor].
        #[method(setShadowCascadeSplittingFactor:)]
        pub unsafe fn setShadowCascadeSplittingFactor(
            &self,
            shadow_cascade_splitting_factor: CGFloat,
        );

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the orthographic scale used to render from the directional light into the shadow map. Defaults to 1.
        ///
        /// This is only applicable for directional lights.
        #[method(orthographicScale)]
        pub unsafe fn orthographicScale(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`orthographicScale`][Self::orthographicScale].
        #[method(setOrthographicScale:)]
        pub unsafe fn setOrthographicScale(&self, orthographic_scale: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the minimal distance between the light and the surface to cast shadow on. If a surface is closer to the light than this minimal distance, then the surface won't be shadowed. The near value must be different than zero. Animatable. Defaults to 1.
        #[method(zNear)]
        pub unsafe fn zNear(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`zNear`][Self::zNear].
        #[method(setZNear:)]
        pub unsafe fn setZNear(&self, z_near: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the maximal distance between the light and a visible surface to cast shadow on. If a surface is further from the light than this maximal distance, then the surface won't be shadowed. Animatable. Defaults to 100.
        #[method(zFar)]
        pub unsafe fn zFar(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`zFar`][Self::zFar].
        #[method(setZFar:)]
        pub unsafe fn setZFar(&self, z_far: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The distance at which the attenuation starts (Omni or Spot light types only). Animatable. Defaults to 0.
        #[method(attenuationStartDistance)]
        pub unsafe fn attenuationStartDistance(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`attenuationStartDistance`][Self::attenuationStartDistance].
        #[method(setAttenuationStartDistance:)]
        pub unsafe fn setAttenuationStartDistance(&self, attenuation_start_distance: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The distance at which the attenuation ends (Omni or Spot light types only). Animatable. Defaults to 0.
        #[method(attenuationEndDistance)]
        pub unsafe fn attenuationEndDistance(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`attenuationEndDistance`][Self::attenuationEndDistance].
        #[method(setAttenuationEndDistance:)]
        pub unsafe fn setAttenuationEndDistance(&self, attenuation_end_distance: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the attenuation between the start and end attenuation distances. 0 means a constant attenuation, 1 a linear attenuation and 2 a quadratic attenuation, but any positive value will work (Omni or Spot light types only). Animatable. Defaults to 2.
        #[method(attenuationFalloffExponent)]
        pub unsafe fn attenuationFalloffExponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`attenuationFalloffExponent`][Self::attenuationFalloffExponent].
        #[method(setAttenuationFalloffExponent:)]
        pub unsafe fn setAttenuationFalloffExponent(&self, attenuation_falloff_exponent: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The angle in degrees between the spot direction and the lit element below which the lighting is at full strength. Animatable. Defaults to 0.
        #[method(spotInnerAngle)]
        pub unsafe fn spotInnerAngle(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`spotInnerAngle`][Self::spotInnerAngle].
        #[method(setSpotInnerAngle:)]
        pub unsafe fn setSpotInnerAngle(&self, spot_inner_angle: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The angle in degrees between the spot direction and the lit element after which the lighting is at zero strength. Animatable. Defaults to 45 degrees.
        #[method(spotOuterAngle)]
        pub unsafe fn spotOuterAngle(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`spotOuterAngle`][Self::spotOuterAngle].
        #[method(setSpotOuterAngle:)]
        pub unsafe fn setSpotOuterAngle(&self, spot_outer_angle: CGFloat);

        /// Specifies the IES file from which the shape, direction, and intensity of illumination is determined. Defaults to nil.
        #[unsafe(method_family(none))]
        #[method_id(IESProfileURL)]
        pub unsafe fn IESProfileURL(&self) -> Option<Retained<NSURL>>;

        /// Setter for [`IESProfileURL`][Self::IESProfileURL].
        #[method(setIESProfileURL:)]
        pub unsafe fn setIESProfileURL(&self, ies_profile_url: Option<&NSURL>);

        /// The receiver's spherical harmonics coefficients.
        ///
        /// Currently spherical harmonics are only supported by light probes (SCNLightTypeProbe). The data is an array of 27 32-bit floating-point values, containing three non-interleaved data sets corresponding to the red, green, and blue sets of coefficients.
        #[unsafe(method_family(none))]
        #[method_id(sphericalHarmonicsCoefficients)]
        pub unsafe fn sphericalHarmonicsCoefficients(&self) -> Retained<NSData>;

        #[method(probeType)]
        pub unsafe fn probeType(&self) -> SCNLightProbeType;

        /// Setter for [`probeType`][Self::probeType].
        #[method(setProbeType:)]
        pub unsafe fn setProbeType(&self, probe_type: SCNLightProbeType);

        #[method(probeUpdateType)]
        pub unsafe fn probeUpdateType(&self) -> SCNLightProbeUpdateType;

        /// Setter for [`probeUpdateType`][Self::probeUpdateType].
        #[method(setProbeUpdateType:)]
        pub unsafe fn setProbeUpdateType(&self, probe_update_type: SCNLightProbeUpdateType);

        #[method(parallaxCorrectionEnabled)]
        pub unsafe fn parallaxCorrectionEnabled(&self) -> bool;

        /// Setter for [`parallaxCorrectionEnabled`][Self::parallaxCorrectionEnabled].
        #[method(setParallaxCorrectionEnabled:)]
        pub unsafe fn setParallaxCorrectionEnabled(&self, parallax_correction_enabled: bool);

        #[cfg(feature = "SCNMaterialProperty")]
        #[unsafe(method_family(none))]
        #[method_id(probeEnvironment)]
        pub unsafe fn probeEnvironment(&self) -> Option<Retained<SCNMaterialProperty>>;

        /// Determines the shape of a light of type SCNLightTypeArea. Defaults to SCNLightAreaTypeRectangle.
        #[method(areaType)]
        pub unsafe fn areaType(&self) -> SCNLightAreaType;

        /// Setter for [`areaType`][Self::areaType].
        #[method(setAreaType:)]
        pub unsafe fn setAreaType(&self, area_type: SCNLightAreaType);

        /// Determines the shape of light of an area light of type SCNLightAreaTypePolygon. Defaults nil.
        ///
        /// An array of CGPoint values corresponding to the coordinates of the polygon's vertices in the XY plane.
        #[unsafe(method_family(none))]
        #[method_id(areaPolygonVertices)]
        pub unsafe fn areaPolygonVertices(&self) -> Option<Retained<NSArray<NSValue>>>;

        /// Setter for [`areaPolygonVertices`][Self::areaPolygonVertices].
        #[method(setAreaPolygonVertices:)]
        pub unsafe fn setAreaPolygonVertices(
            &self,
            area_polygon_vertices: Option<&NSArray<NSValue>>,
        );

        /// Determines whether the shape of a light of type SCNLightTypeArea is drawn in the scene. Defaults to YES.
        #[method(drawsArea)]
        pub unsafe fn drawsArea(&self) -> bool;

        /// Setter for [`drawsArea`][Self::drawsArea].
        #[method(setDrawsArea:)]
        pub unsafe fn setDrawsArea(&self, draws_area: bool);

        /// Determines whether a light of type SCNLightTypeArea is double-sided. Defaults NO.
        ///
        /// Area lights of type SCNLightAreaTypeRectangle or SCNLightAreaTypePolygon emit light along the -Z axis. When set to YES, they also emit light along the +Z axis.
        #[method(doubleSided)]
        pub unsafe fn doubleSided(&self) -> bool;

        /// Setter for [`doubleSided`][Self::doubleSided].
        #[method(setDoubleSided:)]
        pub unsafe fn setDoubleSided(&self, double_sided: bool);

        #[cfg(feature = "SCNMaterialProperty")]
        /// Specifies the gobo (or "cookie") of the light, used to control the shape of emitted light.
        ///
        /// Gobos are only supported by spot lights.
        #[unsafe(method_family(none))]
        #[method_id(gobo)]
        pub unsafe fn gobo(&self) -> Option<Retained<SCNMaterialProperty>>;

        /// Determines the node categories that will be lit by the receiver. Defaults to all bit set.
        #[method(categoryBitMask)]
        pub unsafe fn categoryBitMask(&self) -> NSUInteger;

        /// Setter for [`categoryBitMask`][Self::categoryBitMask].
        #[method(setCategoryBitMask:)]
        pub unsafe fn setCategoryBitMask(&self, category_bit_mask: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNLight {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
