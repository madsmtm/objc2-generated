//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-image")]
#[cfg(not(target_os = "watchos"))]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A SpriteKit node that applies frame buffer effects to the rendered results of its child nodes. This is done continuously on live content and is not a simple snapshot of the rendered result at one instant of time.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/skeffectnode?language=objc)
    #[unsafe(super(SKNode, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    pub struct SKEffectNode;
);

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for SKEffectNode {}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCopying for SKEffectNode {}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl CopyingHelper for SKEffectNode {
    type Result = Self;
}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for SKEffectNode {}

#[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSSecureCoding for SKEffectNode {}

#[cfg(all(
    feature = "SKNode",
    feature = "SKWarpGeometry",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
unsafe impl SKWarpable for SKEffectNode {}

extern_methods!(
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl SKEffectNode {
        #[cfg(feature = "objc2-core-image")]
        /// A CIFilter to be used as an effect
        ///
        /// Any CIFilter that requires only a single "inputImage" and produces an "outputImage" is allowed. The filter is applied to all children of the SKEffectNode. If the filter is nil, the children of this node is flattened before being drawn as long as the SKEffectNode is enabled.
        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Retained<CIFilter>>;

        #[cfg(feature = "objc2-core-image")]
        /// Setter for [`filter`][Self::filter].
        #[method(setFilter:)]
        pub unsafe fn setFilter(&self, filter: Option<&CIFilter>);

        #[method(shouldCenterFilter)]
        pub unsafe fn shouldCenterFilter(&self) -> bool;

        /// Setter for [`shouldCenterFilter`][Self::shouldCenterFilter].
        #[method(setShouldCenterFilter:)]
        pub unsafe fn setShouldCenterFilter(&self, should_center_filter: bool);

        /// Enable the SKEffectNode.
        ///
        /// The SKEffectNode has no effect when appliesEffects is not enabled, this is useful for setting up an effect to use later on. Defaults to YES.
        #[method(shouldEnableEffects)]
        pub unsafe fn shouldEnableEffects(&self) -> bool;

        /// Setter for [`shouldEnableEffects`][Self::shouldEnableEffects].
        #[method(setShouldEnableEffects:)]
        pub unsafe fn setShouldEnableEffects(&self, should_enable_effects: bool);

        /// Enable the rasterization on the SKEffectNode.
        ///
        /// The SKEffectNode's output is rasterized and cached internally. This cache is reused when rendering. When the SKEffectNode's children change, the cache is updated, but changing properties on the CIFilter does *not* cause an update (you must disable rasterization and then re-enable it for the changes to apply). This is more expensive than not rasterizing if the node's children change frequently, only enable this option if you know the children is largely static.
        #[method(shouldRasterize)]
        pub unsafe fn shouldRasterize(&self) -> bool;

        /// Setter for [`shouldRasterize`][Self::shouldRasterize].
        #[method(setShouldRasterize:)]
        pub unsafe fn setShouldRasterize(&self, should_rasterize: bool);

        /// Sets the blend mode to use when composing the effect with the final framebuffer.
        ///
        /// See: SKNode.SKBlendMode
        #[method(blendMode)]
        pub unsafe fn blendMode(&self) -> SKBlendMode;

        /// Setter for [`blendMode`][Self::blendMode].
        #[method(setBlendMode:)]
        pub unsafe fn setBlendMode(&self, blend_mode: SKBlendMode);

        #[cfg(feature = "SKShader")]
        #[method_id(@__retain_semantics Other shader)]
        pub unsafe fn shader(&self) -> Option<Retained<SKShader>>;

        #[cfg(feature = "SKShader")]
        /// Setter for [`shader`][Self::shader].
        #[method(setShader:)]
        pub unsafe fn setShader(&self, shader: Option<&SKShader>);

        #[cfg(feature = "SKAttribute")]
        /// Optional dictionary of SKAttributeValues
        /// Attributes can be used with custom SKShaders.
        #[method_id(@__retain_semantics Other attributeValues)]
        pub unsafe fn attributeValues(&self) -> Retained<NSDictionary<NSString, SKAttributeValue>>;

        #[cfg(feature = "SKAttribute")]
        /// Setter for [`attributeValues`][Self::attributeValues].
        #[method(setAttributeValues:)]
        pub unsafe fn setAttributeValues(
            &self,
            attribute_values: &NSDictionary<NSString, SKAttributeValue>,
        );

        #[cfg(feature = "SKAttribute")]
        #[method_id(@__retain_semantics Other valueForAttributeNamed:)]
        pub unsafe fn valueForAttributeNamed(
            &self,
            key: &NSString,
        ) -> Option<Retained<SKAttributeValue>>;

        #[cfg(feature = "SKAttribute")]
        #[method(setValue:forAttributeNamed:)]
        pub unsafe fn setValue_forAttributeNamed(&self, value: &SKAttributeValue, key: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `SKNode`
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl SKEffectNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Support coding and decoding via NSKeyedArchiver.
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other node)]
        pub unsafe fn node(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other nodeWithFileNamed:)]
        pub unsafe fn nodeWithFileNamed(
            filename: &NSString,
            mtm: MainThreadMarker,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other nodeWithFileNamed:securelyWithClasses:andError:_)]
        pub unsafe fn nodeWithFileNamed_securelyWithClasses_andError(
            filename: &NSString,
            classes: &NSSet<AnyClass>,
            mtm: MainThreadMarker,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl SKEffectNode {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);