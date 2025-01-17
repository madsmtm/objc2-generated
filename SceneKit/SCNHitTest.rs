//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// hit test modes
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestsearchmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNHitTestSearchMode(pub NSInteger);
impl SCNHitTestSearchMode {
    #[doc(alias = "SCNHitTestSearchModeClosest")]
    pub const Closest: Self = Self(0);
    #[doc(alias = "SCNHitTestSearchModeAll")]
    pub const All: Self = Self(1);
    #[doc(alias = "SCNHitTestSearchModeAny")]
    pub const Any: Self = Self(2);
}

unsafe impl Encode for SCNHitTestSearchMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNHitTestSearchMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Hit-test options
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestoption?language=objc)
// NS_TYPED_ENUM
pub type SCNHitTestOption = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestcliptozrangekey?language=objc)
    pub static SCNHitTestClipToZRangeKey: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestbackfacecullingkey?language=objc)
    pub static SCNHitTestBackFaceCullingKey: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestboundingboxonlykey?language=objc)
    pub static SCNHitTestBoundingBoxOnlyKey: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestignorechildnodeskey?language=objc)
    pub static SCNHitTestIgnoreChildNodesKey: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestrootnodekey?language=objc)
    pub static SCNHitTestRootNodeKey: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestignorehiddennodeskey?language=objc)
    pub static SCNHitTestIgnoreHiddenNodesKey: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestoptioncategorybitmask?language=objc)
    pub static SCNHitTestOptionCategoryBitMask: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestoptionsearchmode?language=objc)
    pub static SCNHitTestOptionSearchMode: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestoptionignorelightarea?language=objc)
    pub static SCNHitTestOptionIgnoreLightArea: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestfirstfoundonlykey?language=objc)
    pub static SCNHitTestFirstFoundOnlyKey: &'static SCNHitTestOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestsortresultskey?language=objc)
    pub static SCNHitTestSortResultsKey: &'static SCNHitTestOption;
}

extern_class!(
    /// Results returned by the hit-test methods.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnhittestresult?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNHitTestResult;
);

unsafe impl NSObjectProtocol for SCNHitTestResult {}

extern_methods!(
    unsafe impl SCNHitTestResult {
        #[cfg(feature = "SCNNode")]
        /// The hit node.
        #[unsafe(method_family(none))]
        #[method_id(node)]
        pub unsafe fn node(&self) -> Retained<SCNNode>;

        /// Index of the hit geometry element.
        #[method(geometryIndex)]
        pub unsafe fn geometryIndex(&self) -> NSInteger;

        /// Index of the hit primitive of the geometry element.
        #[method(faceIndex)]
        pub unsafe fn faceIndex(&self) -> NSInteger;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Intersection point in the node's local coordinate system.
        #[method(localCoordinates)]
        pub unsafe fn localCoordinates(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Intersection point in the world coordinate system.
        #[method(worldCoordinates)]
        pub unsafe fn worldCoordinates(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Intersection normal in the node's local coordinate system.
        #[method(localNormal)]
        pub unsafe fn localNormal(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Intersection normal in the world coordinate system.
        #[method(worldNormal)]
        pub unsafe fn worldNormal(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-quartz-core"))]
        #[cfg(not(target_os = "watchos"))]
        /// World transform of the hit node.
        #[method(modelTransform)]
        pub unsafe fn modelTransform(&self) -> SCNMatrix4;

        #[cfg(feature = "SCNNode")]
        /// The hit bone. Only available if the node hit has a SCNSkinner attached.
        #[unsafe(method_family(none))]
        #[method_id(boneNode)]
        pub unsafe fn boneNode(&self) -> Option<Retained<SCNNode>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Returns the texture coordinates at the point of intersection, for a given mapping channel.
        ///
        /// Parameter `channel`: The texture coordinates source index of the geometry to use. The channel must exists on the geometry otherwise {0,0} will be returned.
        #[method(textureCoordinatesWithMappingChannel:)]
        pub unsafe fn textureCoordinatesWithMappingChannel(&self, channel: NSInteger) -> CGPoint;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNHitTestResult {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// SIMD
    unsafe impl SCNHitTestResult {}
);
