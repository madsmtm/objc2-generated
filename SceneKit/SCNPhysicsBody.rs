//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnphysicsbodytype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNPhysicsBodyType(pub NSInteger);
impl SCNPhysicsBodyType {
    #[doc(alias = "SCNPhysicsBodyTypeStatic")]
    pub const Static: Self = Self(0);
    #[doc(alias = "SCNPhysicsBodyTypeDynamic")]
    pub const Dynamic: Self = Self(1);
    #[doc(alias = "SCNPhysicsBodyTypeKinematic")]
    pub const Kinematic: Self = Self(2);
}

unsafe impl Encode for SCNPhysicsBodyType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNPhysicsBodyType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnphysicscollisioncategory?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNPhysicsCollisionCategory(pub NSUInteger);
bitflags::bitflags! {
    impl SCNPhysicsCollisionCategory: NSUInteger {
        #[doc(alias = "SCNPhysicsCollisionCategoryDefault")]
        const Default = 1<<0;
        #[doc(alias = "SCNPhysicsCollisionCategoryStatic")]
        const Static = 1<<1;
        #[doc(alias = "SCNPhysicsCollisionCategoryAll")]
        const All = !0;
    }
}

unsafe impl Encode for SCNPhysicsCollisionCategory {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for SCNPhysicsCollisionCategory {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// The SCNPhysicsBody class describes the physics properties (such as mass, friction...) of a node.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnphysicsbody?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNPhysicsBody;
);

unsafe impl NSCoding for SCNPhysicsBody {}

unsafe impl NSCopying for SCNPhysicsBody {}

unsafe impl CopyingHelper for SCNPhysicsBody {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNPhysicsBody {}

unsafe impl NSSecureCoding for SCNPhysicsBody {}

extern_methods!(
    unsafe impl SCNPhysicsBody {
        #[method_id(@__retain_semantics Other staticBody)]
        pub unsafe fn staticBody() -> Retained<Self>;

        #[method_id(@__retain_semantics Other dynamicBody)]
        pub unsafe fn dynamicBody() -> Retained<Self>;

        #[method_id(@__retain_semantics Other kinematicBody)]
        pub unsafe fn kinematicBody() -> Retained<Self>;

        #[cfg(feature = "SCNPhysicsShape")]
        #[method_id(@__retain_semantics Other bodyWithType:shape:)]
        pub unsafe fn bodyWithType_shape(
            r#type: SCNPhysicsBodyType,
            shape: Option<&SCNPhysicsShape>,
        ) -> Retained<Self>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> SCNPhysicsBodyType;

        /// Setter for [`type`][Self::type].
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: SCNPhysicsBodyType);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(mass)]
        pub unsafe fn mass(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`mass`][Self::mass].
        #[method(setMass:)]
        pub unsafe fn setMass(&self, mass: CGFloat);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        #[method(momentOfInertia)]
        pub unsafe fn momentOfInertia(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`momentOfInertia`][Self::momentOfInertia].
        #[method(setMomentOfInertia:)]
        pub unsafe fn setMomentOfInertia(&self, moment_of_inertia: SCNVector3);

        #[method(usesDefaultMomentOfInertia)]
        pub unsafe fn usesDefaultMomentOfInertia(&self) -> bool;

        /// Setter for [`usesDefaultMomentOfInertia`][Self::usesDefaultMomentOfInertia].
        #[method(setUsesDefaultMomentOfInertia:)]
        pub unsafe fn setUsesDefaultMomentOfInertia(&self, uses_default_moment_of_inertia: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(charge)]
        pub unsafe fn charge(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`charge`][Self::charge].
        #[method(setCharge:)]
        pub unsafe fn setCharge(&self, charge: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(friction)]
        pub unsafe fn friction(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`friction`][Self::friction].
        #[method(setFriction:)]
        pub unsafe fn setFriction(&self, friction: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(restitution)]
        pub unsafe fn restitution(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`restitution`][Self::restitution].
        #[method(setRestitution:)]
        pub unsafe fn setRestitution(&self, restitution: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rollingFriction)]
        pub unsafe fn rollingFriction(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`rollingFriction`][Self::rollingFriction].
        #[method(setRollingFriction:)]
        pub unsafe fn setRollingFriction(&self, rolling_friction: CGFloat);

        #[cfg(feature = "SCNPhysicsShape")]
        #[method_id(@__retain_semantics Other physicsShape)]
        pub unsafe fn physicsShape(&self) -> Option<Retained<SCNPhysicsShape>>;

        #[cfg(feature = "SCNPhysicsShape")]
        /// Setter for [`physicsShape`][Self::physicsShape].
        #[method(setPhysicsShape:)]
        pub unsafe fn setPhysicsShape(&self, physics_shape: Option<&SCNPhysicsShape>);

        #[method(isResting)]
        pub unsafe fn isResting(&self) -> bool;

        #[method(allowsResting)]
        pub unsafe fn allowsResting(&self) -> bool;

        /// Setter for [`allowsResting`][Self::allowsResting].
        #[method(setAllowsResting:)]
        pub unsafe fn setAllowsResting(&self, allows_resting: bool);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        #[method(velocity)]
        pub unsafe fn velocity(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`velocity`][Self::velocity].
        #[method(setVelocity:)]
        pub unsafe fn setVelocity(&self, velocity: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        #[method(angularVelocity)]
        pub unsafe fn angularVelocity(&self) -> SCNVector4;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`angularVelocity`][Self::angularVelocity].
        #[method(setAngularVelocity:)]
        pub unsafe fn setAngularVelocity(&self, angular_velocity: SCNVector4);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(damping)]
        pub unsafe fn damping(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`damping`][Self::damping].
        #[method(setDamping:)]
        pub unsafe fn setDamping(&self, damping: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(angularDamping)]
        pub unsafe fn angularDamping(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`angularDamping`][Self::angularDamping].
        #[method(setAngularDamping:)]
        pub unsafe fn setAngularDamping(&self, angular_damping: CGFloat);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        #[method(velocityFactor)]
        pub unsafe fn velocityFactor(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`velocityFactor`][Self::velocityFactor].
        #[method(setVelocityFactor:)]
        pub unsafe fn setVelocityFactor(&self, velocity_factor: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        #[method(angularVelocityFactor)]
        pub unsafe fn angularVelocityFactor(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`angularVelocityFactor`][Self::angularVelocityFactor].
        #[method(setAngularVelocityFactor:)]
        pub unsafe fn setAngularVelocityFactor(&self, angular_velocity_factor: SCNVector3);

        #[method(categoryBitMask)]
        pub unsafe fn categoryBitMask(&self) -> NSUInteger;

        /// Setter for [`categoryBitMask`][Self::categoryBitMask].
        #[method(setCategoryBitMask:)]
        pub unsafe fn setCategoryBitMask(&self, category_bit_mask: NSUInteger);

        #[method(collisionBitMask)]
        pub unsafe fn collisionBitMask(&self) -> NSUInteger;

        /// Setter for [`collisionBitMask`][Self::collisionBitMask].
        #[method(setCollisionBitMask:)]
        pub unsafe fn setCollisionBitMask(&self, collision_bit_mask: NSUInteger);

        #[method(contactTestBitMask)]
        pub unsafe fn contactTestBitMask(&self) -> NSUInteger;

        /// Setter for [`contactTestBitMask`][Self::contactTestBitMask].
        #[method(setContactTestBitMask:)]
        pub unsafe fn setContactTestBitMask(&self, contact_test_bit_mask: NSUInteger);

        #[method(isAffectedByGravity)]
        pub unsafe fn isAffectedByGravity(&self) -> bool;

        /// Setter for [`isAffectedByGravity`][Self::isAffectedByGravity].
        #[method(setAffectedByGravity:)]
        pub unsafe fn setAffectedByGravity(&self, affected_by_gravity: bool);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        #[method(applyForce:impulse:)]
        pub unsafe fn applyForce_impulse(&self, direction: SCNVector3, impulse: bool);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        #[method(applyForce:atPosition:impulse:)]
        pub unsafe fn applyForce_atPosition_impulse(
            &self,
            direction: SCNVector3,
            position: SCNVector3,
            impulse: bool,
        );

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        #[method(applyTorque:impulse:)]
        pub unsafe fn applyTorque_impulse(&self, torque: SCNVector4, impulse: bool);

        #[method(clearAllForces)]
        pub unsafe fn clearAllForces(&self);

        #[method(resetTransform)]
        pub unsafe fn resetTransform(&self);

        #[method(setResting:)]
        pub unsafe fn setResting(&self, resting: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(continuousCollisionDetectionThreshold)]
        pub unsafe fn continuousCollisionDetectionThreshold(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`continuousCollisionDetectionThreshold`][Self::continuousCollisionDetectionThreshold].
        #[method(setContinuousCollisionDetectionThreshold:)]
        pub unsafe fn setContinuousCollisionDetectionThreshold(
            &self,
            continuous_collision_detection_threshold: CGFloat,
        );

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        #[method(centerOfMassOffset)]
        pub unsafe fn centerOfMassOffset(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`centerOfMassOffset`][Self::centerOfMassOffset].
        #[method(setCenterOfMassOffset:)]
        pub unsafe fn setCenterOfMassOffset(&self, center_of_mass_offset: SCNVector3);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(linearRestingThreshold)]
        pub unsafe fn linearRestingThreshold(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`linearRestingThreshold`][Self::linearRestingThreshold].
        #[method(setLinearRestingThreshold:)]
        pub unsafe fn setLinearRestingThreshold(&self, linear_resting_threshold: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(angularRestingThreshold)]
        pub unsafe fn angularRestingThreshold(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`angularRestingThreshold`][Self::angularRestingThreshold].
        #[method(setAngularRestingThreshold:)]
        pub unsafe fn setAngularRestingThreshold(&self, angular_resting_threshold: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNPhysicsBody {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);