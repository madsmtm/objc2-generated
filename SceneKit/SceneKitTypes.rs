//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

/// The modes that an action can use to adjust the apparent timing of the action.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnactiontimingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNActionTimingMode(pub NSInteger);
impl SCNActionTimingMode {
    #[doc(alias = "SCNActionTimingModeLinear")]
    pub const Linear: Self = Self(0);
    #[doc(alias = "SCNActionTimingModeEaseIn")]
    pub const EaseIn: Self = Self(1);
    #[doc(alias = "SCNActionTimingModeEaseOut")]
    pub const EaseOut: Self = Self(2);
    #[doc(alias = "SCNActionTimingModeEaseInEaseOut")]
    pub const EaseInEaseOut: Self = Self(3);
}

unsafe impl Encode for SCNActionTimingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNActionTimingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Color components
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scncolormask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNColorMask(pub NSInteger);
bitflags::bitflags! {
    impl SCNColorMask: NSInteger {
        #[doc(alias = "SCNColorMaskNone")]
        const None = 0;
        #[doc(alias = "SCNColorMaskRed")]
        const Red = 0x1<<3;
        #[doc(alias = "SCNColorMaskGreen")]
        const Green = 0x1<<2;
        #[doc(alias = "SCNColorMaskBlue")]
        const Blue = 0x1<<1;
        #[doc(alias = "SCNColorMaskAlpha")]
        const Alpha = 0x1<<0;
        #[doc(alias = "SCNColorMaskAll")]
        const All = 0xf;
    }
}

unsafe impl Encode for SCNColorMask {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNColorMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnvector3?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCNVector3 {
    pub x: CGFloat,
    pub y: CGFloat,
    pub z: CGFloat,
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for SCNVector3 {
    const ENCODING: Encoding = Encoding::Struct(
        "SCNVector3",
        &[
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for SCNVector3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnvector4?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCNVector4 {
    pub x: CGFloat,
    pub y: CGFloat,
    pub z: CGFloat,
    pub w: CGFloat,
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for SCNVector4 {
    const ENCODING: Encoding = Encoding::Struct(
        "SCNVector4",
        &[
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for SCNVector4 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnvector3zero?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static SCNVector3Zero: SCNVector3;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnvector4zero?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static SCNVector4Zero: SCNVector4;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn SCNVector3EqualToVector3(a: SCNVector3, b: SCNVector3) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn SCNVector4EqualToVector4(a: SCNVector4, b: SCNVector4) -> bool;
}

// TODO: pub fn SCNVector3Make(x: CGFloat,y: CGFloat,z: CGFloat,) -> SCNVector3;

// TODO: pub fn SCNVector4Make(x: CGFloat,y: CGFloat,z: CGFloat,w: CGFloat,) -> SCNVector4;

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnquaternion?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type SCNQuaternion = SCNVector4;

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnmatrix4?language=objc)
#[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
#[cfg(not(target_os = "watchos"))]
pub type SCNMatrix4 = CATransform3D;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnmatrix4identity?language=objc)
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
    #[cfg(not(target_os = "watchos"))]
    pub static SCNMatrix4Identity: SCNMatrix4;
}

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
    #[cfg(not(target_os = "watchos"))]
    pub fn SCNMatrix4IsIdentity(m: SCNMatrix4) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
    #[cfg(not(target_os = "watchos"))]
    pub fn SCNMatrix4EqualToMatrix4(a: SCNMatrix4, b: SCNMatrix4) -> bool;
}

// TODO: pub fn SCNMatrix4MakeTranslation(tx: CGFloat,ty: CGFloat,tz: CGFloat,) -> SCNMatrix4;

// TODO: pub fn SCNMatrix4MakeScale(sx: CGFloat,sy: CGFloat,sz: CGFloat,) -> SCNMatrix4;

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
    #[cfg(not(target_os = "watchos"))]
    pub fn SCNMatrix4MakeRotation(angle: CGFloat, x: CGFloat, y: CGFloat, z: CGFloat)
        -> SCNMatrix4;
}

// TODO: pub fn SCNMatrix4Translate(m: SCNMatrix4,tx: CGFloat,ty: CGFloat,tz: CGFloat,) -> SCNMatrix4;

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
    #[cfg(not(target_os = "watchos"))]
    pub fn SCNMatrix4Scale(m: SCNMatrix4, sx: CGFloat, sy: CGFloat, sz: CGFloat) -> SCNMatrix4;
}

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
    #[cfg(not(target_os = "watchos"))]
    pub fn SCNMatrix4Rotate(
        m: SCNMatrix4,
        angle: CGFloat,
        x: CGFloat,
        y: CGFloat,
        z: CGFloat,
    ) -> SCNMatrix4;
}

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
    #[cfg(not(target_os = "watchos"))]
    pub fn SCNMatrix4Invert(m: SCNMatrix4) -> SCNMatrix4;
}

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
    #[cfg(not(target_os = "watchos"))]
    pub fn SCNMatrix4Mult(a: SCNMatrix4, b: SCNMatrix4) -> SCNMatrix4;
}

extern_category!(
    /// Category "SceneKitAdditions" on [`NSValue`].
    #[doc(alias = "SceneKitAdditions")]
    /// Adds methods to wrap vectors in NSValue objects.
    pub unsafe trait NSValueSceneKitAdditions {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other valueWithSCNVector3:)]
        unsafe fn valueWithSCNVector3(v: SCNVector3) -> Retained<NSValue>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other valueWithSCNVector4:)]
        unsafe fn valueWithSCNVector4(v: SCNVector4) -> Retained<NSValue>;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other valueWithSCNMatrix4:)]
        unsafe fn valueWithSCNMatrix4(v: SCNMatrix4) -> Retained<NSValue>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(SCNVector3Value)]
        unsafe fn SCNVector3Value(&self) -> SCNVector3;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(SCNVector4Value)]
        unsafe fn SCNVector4Value(&self) -> SCNVector4;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
        #[cfg(not(target_os = "watchos"))]
        #[method(SCNMatrix4Value)]
        unsafe fn SCNMatrix4Value(&self) -> SCNMatrix4;
    }

    unsafe impl NSValueSceneKitAdditions for NSValue {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnerrordomain?language=objc)
    pub static SCNErrorDomain: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnprogramcompilationerror?language=objc)
pub const SCNProgramCompilationError: c_uint = 1;