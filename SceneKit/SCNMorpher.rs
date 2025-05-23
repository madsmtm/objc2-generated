//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnmorphercalculationmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNMorpherCalculationMode(pub NSInteger);
impl SCNMorpherCalculationMode {
    #[doc(alias = "SCNMorpherCalculationModeNormalized")]
    pub const Normalized: Self = Self(0);
    #[doc(alias = "SCNMorpherCalculationModeAdditive")]
    pub const Additive: Self = Self(1);
}

unsafe impl Encode for SCNMorpherCalculationMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNMorpherCalculationMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// SCNMorpher controls the deformation of morphed geometries
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnmorpher?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNMorpher;
);

extern_conformance!(
    unsafe impl NSCoding for SCNMorpher {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for SCNMorpher {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for SCNMorpher {}
);

#[cfg(feature = "SCNAnimation")]
extern_conformance!(
    unsafe impl SCNAnimatable for SCNMorpher {}
);

impl SCNMorpher {
    extern_methods!(
        #[cfg(feature = "SCNGeometry")]
        /// Specifies the morph targets as an array of SCNGeometry.
        ///
        /// The target geometries must have the same number of entries in their geometry sources and the same topology as the base geometry.
        #[unsafe(method(targets))]
        #[unsafe(method_family = none)]
        pub unsafe fn targets(&self) -> Retained<NSArray<SCNGeometry>>;

        #[cfg(feature = "SCNGeometry")]
        /// Setter for [`targets`][Self::targets].
        #[unsafe(method(setTargets:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTargets(&self, targets: &NSArray<SCNGeometry>);

        /// Access to all the weights of all the targets.
        #[unsafe(method(weights))]
        #[unsafe(method_family = none)]
        pub unsafe fn weights(&self) -> Retained<NSArray<NSNumber>>;

        /// Setter for [`weights`][Self::weights].
        #[unsafe(method(setWeights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWeights(&self, weights: &NSArray<NSNumber>);

        #[cfg(feature = "objc2-core-foundation")]
        /// Sets the weight for the target at the specified index. Animatable implicitly or explicitly with the keyPath "weights[index]" or "weights["targetName"]" (targetName is the name of the target geometry).
        #[unsafe(method(setWeight:forTargetAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWeight_forTargetAtIndex(&self, weight: CGFloat, target_index: NSUInteger);

        #[cfg(feature = "objc2-core-foundation")]
        /// Retrieves the weight for the target at the specified index.
        #[unsafe(method(weightForTargetAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn weightForTargetAtIndex(&self, target_index: NSUInteger) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Sets the weight for the target with the specified name (targetName is the name of the target geometry).
        #[unsafe(method(setWeight:forTargetNamed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWeight_forTargetNamed(&self, weight: CGFloat, target_name: &NSString);

        #[cfg(feature = "objc2-core-foundation")]
        /// Retrieves the weight for the target with the specified name (targetName is the name of the target geometry).
        #[unsafe(method(weightForTargetNamed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn weightForTargetNamed(&self, target_name: &NSString) -> CGFloat;

        /// Specifies how the morph result is calculated by the receiver. Defaults to SCNMorpherCalculationModeNormalized.
        #[unsafe(method(calculationMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn calculationMode(&self) -> SCNMorpherCalculationMode;

        /// Setter for [`calculationMode`][Self::calculationMode].
        #[unsafe(method(setCalculationMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCalculationMode(&self, calculation_mode: SCNMorpherCalculationMode);

        /// When set to YES the normals are not morphed but are recomputed after morphing the vertex instead. When set to NO, the morpher will morph the normals if the geometry targets have normals. Defaults to NO.
        #[unsafe(method(unifiesNormals))]
        #[unsafe(method_family = none)]
        pub unsafe fn unifiesNormals(&self) -> bool;

        /// Setter for [`unifiesNormals`][Self::unifiesNormals].
        #[unsafe(method(setUnifiesNormals:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUnifiesNormals(&self, unifies_normals: bool);
    );
}

/// Methods declared on superclass `NSObject`.
impl SCNMorpher {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
