//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caemitterlayeremittershape?language=objc)
// NS_TYPED_ENUM
pub type CAEmitterLayerEmitterShape = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caemitterlayeremittermode?language=objc)
// NS_TYPED_ENUM
pub type CAEmitterLayerEmitterMode = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caemitterlayerrendermode?language=objc)
// NS_TYPED_ENUM
pub type CAEmitterLayerRenderMode = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caemitterlayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CAEmitterLayer;
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
extern_conformance!(
    unsafe impl CAMediaTiming for CAEmitterLayer {}
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl NSCoding for CAEmitterLayer {}
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CAEmitterLayer {}
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl NSSecureCoding for CAEmitterLayer {}
);

#[cfg(feature = "CALayer")]
impl CAEmitterLayer {
    extern_methods!(
        #[cfg(feature = "CAEmitterCell")]
        #[unsafe(method(emitterCells))]
        #[unsafe(method_family = none)]
        pub unsafe fn emitterCells(&self) -> Option<Retained<NSArray<CAEmitterCell>>>;

        #[cfg(feature = "CAEmitterCell")]
        /// Setter for [`emitterCells`][Self::emitterCells].
        #[unsafe(method(setEmitterCells:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEmitterCells(&self, emitter_cells: Option<&NSArray<CAEmitterCell>>);

        #[unsafe(method(birthRate))]
        #[unsafe(method_family = none)]
        pub unsafe fn birthRate(&self) -> c_float;

        /// Setter for [`birthRate`][Self::birthRate].
        #[unsafe(method(setBirthRate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBirthRate(&self, birth_rate: c_float);

        #[unsafe(method(lifetime))]
        #[unsafe(method_family = none)]
        pub unsafe fn lifetime(&self) -> c_float;

        /// Setter for [`lifetime`][Self::lifetime].
        #[unsafe(method(setLifetime:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLifetime(&self, lifetime: c_float);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(emitterPosition))]
        #[unsafe(method_family = none)]
        pub unsafe fn emitterPosition(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`emitterPosition`][Self::emitterPosition].
        #[unsafe(method(setEmitterPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEmitterPosition(&self, emitter_position: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(emitterZPosition))]
        #[unsafe(method_family = none)]
        pub unsafe fn emitterZPosition(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`emitterZPosition`][Self::emitterZPosition].
        #[unsafe(method(setEmitterZPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEmitterZPosition(&self, emitter_z_position: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(emitterSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn emitterSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`emitterSize`][Self::emitterSize].
        #[unsafe(method(setEmitterSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEmitterSize(&self, emitter_size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(emitterDepth))]
        #[unsafe(method_family = none)]
        pub unsafe fn emitterDepth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`emitterDepth`][Self::emitterDepth].
        #[unsafe(method(setEmitterDepth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEmitterDepth(&self, emitter_depth: CGFloat);

        #[unsafe(method(emitterShape))]
        #[unsafe(method_family = none)]
        pub unsafe fn emitterShape(&self) -> Retained<CAEmitterLayerEmitterShape>;

        /// Setter for [`emitterShape`][Self::emitterShape].
        #[unsafe(method(setEmitterShape:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEmitterShape(&self, emitter_shape: &CAEmitterLayerEmitterShape);

        #[unsafe(method(emitterMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn emitterMode(&self) -> Retained<CAEmitterLayerEmitterMode>;

        /// Setter for [`emitterMode`][Self::emitterMode].
        #[unsafe(method(setEmitterMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEmitterMode(&self, emitter_mode: &CAEmitterLayerEmitterMode);

        #[unsafe(method(renderMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn renderMode(&self) -> Retained<CAEmitterLayerRenderMode>;

        /// Setter for [`renderMode`][Self::renderMode].
        #[unsafe(method(setRenderMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRenderMode(&self, render_mode: &CAEmitterLayerRenderMode);

        #[unsafe(method(preservesDepth))]
        #[unsafe(method_family = none)]
        pub unsafe fn preservesDepth(&self) -> bool;

        /// Setter for [`preservesDepth`][Self::preservesDepth].
        #[unsafe(method(setPreservesDepth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPreservesDepth(&self, preserves_depth: bool);

        #[unsafe(method(velocity))]
        #[unsafe(method_family = none)]
        pub unsafe fn velocity(&self) -> c_float;

        /// Setter for [`velocity`][Self::velocity].
        #[unsafe(method(setVelocity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVelocity(&self, velocity: c_float);

        #[unsafe(method(scale))]
        #[unsafe(method_family = none)]
        pub unsafe fn scale(&self) -> c_float;

        /// Setter for [`scale`][Self::scale].
        #[unsafe(method(setScale:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setScale(&self, scale: c_float);

        #[unsafe(method(spin))]
        #[unsafe(method_family = none)]
        pub unsafe fn spin(&self) -> c_float;

        /// Setter for [`spin`][Self::spin].
        #[unsafe(method(setSpin:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSpin(&self, spin: c_float);

        #[unsafe(method(seed))]
        #[unsafe(method_family = none)]
        pub unsafe fn seed(&self) -> c_uint;

        /// Setter for [`seed`][Self::seed].
        #[unsafe(method(setSeed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSeed(&self, seed: c_uint);
    );
}

/// Methods declared on superclass `CALayer`.
#[cfg(feature = "CALayer")]
impl CAEmitterLayer {
    extern_methods!(
        /// Layer creation and initialization. *
        #[unsafe(method(layer))]
        #[unsafe(method_family = none)]
        pub unsafe fn layer() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithLayer:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "CALayer")]
impl CAEmitterLayer {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern "C" {
    /// `emitterShape' values. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerpoint?language=objc)
    pub static kCAEmitterLayerPoint: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerline?language=objc)
    pub static kCAEmitterLayerLine: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerrectangle?language=objc)
    pub static kCAEmitterLayerRectangle: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayercuboid?language=objc)
    pub static kCAEmitterLayerCuboid: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayercircle?language=objc)
    pub static kCAEmitterLayerCircle: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayersphere?language=objc)
    pub static kCAEmitterLayerSphere: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// `emitterMode' values. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerpoints?language=objc)
    pub static kCAEmitterLayerPoints: &'static CAEmitterLayerEmitterMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayeroutline?language=objc)
    pub static kCAEmitterLayerOutline: &'static CAEmitterLayerEmitterMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayersurface?language=objc)
    pub static kCAEmitterLayerSurface: &'static CAEmitterLayerEmitterMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayervolume?language=objc)
    pub static kCAEmitterLayerVolume: &'static CAEmitterLayerEmitterMode;
}

extern "C" {
    /// `renderMode' values. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerunordered?language=objc)
    pub static kCAEmitterLayerUnordered: &'static CAEmitterLayerRenderMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayeroldestfirst?language=objc)
    pub static kCAEmitterLayerOldestFirst: &'static CAEmitterLayerRenderMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayeroldestlast?language=objc)
    pub static kCAEmitterLayerOldestLast: &'static CAEmitterLayerRenderMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerbacktofront?language=objc)
    pub static kCAEmitterLayerBackToFront: &'static CAEmitterLayerRenderMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayeradditive?language=objc)
    pub static kCAEmitterLayerAdditive: &'static CAEmitterLayerRenderMode;
}
