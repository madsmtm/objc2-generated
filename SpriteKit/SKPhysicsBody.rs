//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A SpriteKit physics body. These are the physical representations of your nodes. These specify the area and mass and any collision masking needed.
    ///
    /// All bodies have zero, one or more shapes that define its area. A body with no shapes is ethereal and does not collide with other bodies.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/skphysicsbody?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKPhysicsBody;
);

unsafe impl NSCoding for SKPhysicsBody {}

unsafe impl NSCopying for SKPhysicsBody {}

unsafe impl CopyingHelper for SKPhysicsBody {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SKPhysicsBody {}

unsafe impl NSSecureCoding for SKPhysicsBody {}

extern_methods!(
    unsafe impl SKPhysicsBody {
        #[cfg(feature = "objc2-core-foundation")]
        /// Creates a circle of radius r centered at the node's origin.
        ///
        /// Parameter `r`: the radius in points
        #[method_id(@__retain_semantics Other bodyWithCircleOfRadius:)]
        pub unsafe fn bodyWithCircleOfRadius(r: CGFloat) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Creates a circle of radius r centered at a point in the node's coordinate space.
        ///
        /// Parameter `r`: the radius in points
        #[method_id(@__retain_semantics Other bodyWithCircleOfRadius:center:)]
        pub unsafe fn bodyWithCircleOfRadius_center(
            r: CGFloat,
            center: CGPoint,
        ) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Creates a rectangle of the specified size centered at the node's origin.
        ///
        /// Parameter `s`: the size in points
        #[method_id(@__retain_semantics Other bodyWithRectangleOfSize:)]
        pub unsafe fn bodyWithRectangleOfSize(s: CGSize) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Creates a rectangle of the specified size centered at a point in the node's coordinate space.
        ///
        /// Parameter `s`: the size in points
        #[method_id(@__retain_semantics Other bodyWithRectangleOfSize:center:)]
        pub unsafe fn bodyWithRectangleOfSize_center(
            s: CGSize,
            center: CGPoint,
        ) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "objc2-core-graphics")]
        /// The path must represent a convex or concave polygon with counter clockwise winding and no self intersection. Positions are relative to the node's origin.
        ///
        /// Parameter `path`: the path to use
        #[method_id(@__retain_semantics Other bodyWithPolygonFromPath:)]
        pub unsafe fn bodyWithPolygonFromPath(path: &CGPath) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Creates an edge from p1 to p2. Edges have no volume and are intended to be used to create static environments. Edges can collide with bodies of volume, but not with each other.
        ///
        /// Parameter `p1`: start point
        ///
        /// Parameter `p2`: end point
        #[method_id(@__retain_semantics Other bodyWithEdgeFromPoint:toPoint:)]
        pub unsafe fn bodyWithEdgeFromPoint_toPoint(
            p1: CGPoint,
            p2: CGPoint,
        ) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Creates an edge chain from a path. The path must have no self intersection. Edges have no volume and are intended to be used to create static environments. Edges can collide with bodies of volume, but not with each other.
        ///
        /// Parameter `path`: the path to use
        #[method_id(@__retain_semantics Other bodyWithEdgeChainFromPath:)]
        pub unsafe fn bodyWithEdgeChainFromPath(path: &CGPath) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Creates an edge loop from a path. A loop is automatically created by joining the last point to the first. The path must have no self intersection. Edges have no volume and are intended to be used to create static environments. Edges can collide with body's of volume, but not with each other.
        ///
        /// Parameter `path`: the path to use
        #[method_id(@__retain_semantics Other bodyWithEdgeLoopFromPath:)]
        pub unsafe fn bodyWithEdgeLoopFromPath(path: &CGPath) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Creates an edge loop from a CGRect. Edges have no volume and are intended to be used to create static environments. Edges can collide with body's of volume, but not with each other.
        ///
        /// Parameter `rect`: the CGRect to use
        #[method_id(@__retain_semantics Other bodyWithEdgeLoopFromRect:)]
        pub unsafe fn bodyWithEdgeLoopFromRect(rect: CGRect) -> Retained<SKPhysicsBody>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Creates a body from the alpha values in the supplied texture.
        ///
        /// Parameter `texture`: the texture to be interpreted
        ///
        /// Parameter `size`: of the generated physics body
        #[method_id(@__retain_semantics Other bodyWithTexture:size:)]
        pub unsafe fn bodyWithTexture_size(
            texture: &SKTexture,
            size: CGSize,
        ) -> Retained<SKPhysicsBody>;

        #[cfg(all(feature = "SKTexture", feature = "objc2-core-foundation"))]
        /// Creates a body from the alpha values in the supplied texture.
        ///
        /// Parameter `texture`: the texture to be interpreted
        ///
        /// Parameter `alphaThreshold`: the alpha value above which a pixel is interpreted as opaque
        ///
        /// Parameter `size`: of the generated physics body
        #[method_id(@__retain_semantics Other bodyWithTexture:alphaThreshold:size:)]
        pub unsafe fn bodyWithTexture_alphaThreshold_size(
            texture: &SKTexture,
            alpha_threshold: c_float,
            size: CGSize,
        ) -> Retained<SKPhysicsBody>;

        /// Creates an compound body that is the union of the bodies used to create it.
        #[method_id(@__retain_semantics Other bodyWithBodies:)]
        pub unsafe fn bodyWithBodies(bodies: &NSArray<SKPhysicsBody>) -> Retained<SKPhysicsBody>;

        #[method(isDynamic)]
        pub unsafe fn isDynamic(&self) -> bool;

        /// Setter for [`isDynamic`][Self::isDynamic].
        #[method(setDynamic:)]
        pub unsafe fn setDynamic(&self, dynamic: bool);

        #[method(usesPreciseCollisionDetection)]
        pub unsafe fn usesPreciseCollisionDetection(&self) -> bool;

        /// Setter for [`usesPreciseCollisionDetection`][Self::usesPreciseCollisionDetection].
        #[method(setUsesPreciseCollisionDetection:)]
        pub unsafe fn setUsesPreciseCollisionDetection(
            &self,
            uses_precise_collision_detection: bool,
        );

        #[method(allowsRotation)]
        pub unsafe fn allowsRotation(&self) -> bool;

        /// Setter for [`allowsRotation`][Self::allowsRotation].
        #[method(setAllowsRotation:)]
        pub unsafe fn setAllowsRotation(&self, allows_rotation: bool);

        #[method(pinned)]
        pub unsafe fn pinned(&self) -> bool;

        /// Setter for [`pinned`][Self::pinned].
        #[method(setPinned:)]
        pub unsafe fn setPinned(&self, pinned: bool);

        /// If the physics simulation has determined that this body is at rest it may set the resting property to YES. Resting bodies do not participate
        /// in the simulation until some collision with a non-resting  object, or an impulse is applied, that unrests it. If all bodies in the world are resting
        /// then the simulation as a whole is "at rest".
        #[method(isResting)]
        pub unsafe fn isResting(&self) -> bool;

        /// Setter for [`isResting`][Self::isResting].
        #[method(setResting:)]
        pub unsafe fn setResting(&self, resting: bool);

        #[cfg(feature = "objc2-core-foundation")]
        /// Determines the 'roughness' for the surface of the physics body (0.0 - 1.0). Defaults to 0.2
        #[method(friction)]
        pub unsafe fn friction(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`friction`][Self::friction].
        #[method(setFriction:)]
        pub unsafe fn setFriction(&self, friction: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the charge on the body. Charge determines the degree to which a body is affected by
        /// electric and magnetic fields. Note that this is a unitless quantity, it is up to the developer to
        /// set charge and field strength appropriately. Defaults to 0.0
        #[method(charge)]
        pub unsafe fn charge(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`charge`][Self::charge].
        #[method(setCharge:)]
        pub unsafe fn setCharge(&self, charge: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Determines the 'bounciness' of the physics body (0.0 - 1.0). Defaults to 0.2
        #[method(restitution)]
        pub unsafe fn restitution(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`restitution`][Self::restitution].
        #[method(setRestitution:)]
        pub unsafe fn setRestitution(&self, restitution: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Optionally reduce the body's linear velocity each frame to simulate fluid/air friction. Value should be zero or greater. Defaults to 0.1.
        /// Used in conjunction with per frame impulses, an object can be made to move at a constant speed. For example, if an object 64 points in size
        /// and default density and a linearDamping of 25 will slide across the screen in a few seconds if an impulse of magnitude 10 is applied every update.
        #[method(linearDamping)]
        pub unsafe fn linearDamping(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`linearDamping`][Self::linearDamping].
        #[method(setLinearDamping:)]
        pub unsafe fn setLinearDamping(&self, linear_damping: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Optionally reduce the body's angular velocity each frame to simulate rotational friction. (0.0 - 1.0). Defaults to 0.1
        #[method(angularDamping)]
        pub unsafe fn angularDamping(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`angularDamping`][Self::angularDamping].
        #[method(setAngularDamping:)]
        pub unsafe fn setAngularDamping(&self, angular_damping: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The density of the body.
        ///
        /// The unit is arbitrary, as long as the relative densities are consistent throughout the application. Note that density and mass are inherently related (they are directly proportional), so changing one also changes the other. Both are provided so either can be used depending on what is more relevant to your usage.
        #[method(density)]
        pub unsafe fn density(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`density`][Self::density].
        #[method(setDensity:)]
        pub unsafe fn setDensity(&self, density: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The mass of the body.
        ///
        /// The unit is arbitrary, as long as the relative masses are consistent throughout the application. Note that density and mass are inherently related (they are directly proportional), so changing one also changes the other. Both are provided so either can be used depending on what is more relevant to your usage.
        #[method(mass)]
        pub unsafe fn mass(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`mass`][Self::mass].
        #[method(setMass:)]
        pub unsafe fn setMass(&self, mass: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The area of the body.
        ///
        /// The unit is arbitrary, as long as the relative areas are consistent throughout the application.
        #[method(area)]
        pub unsafe fn area(&self) -> CGFloat;

        /// Bodies are affected by field forces such as gravity if this property is set and the field's category mask is set appropriately. The default value is YES.
        ///
        /// If this is set a force is applied to the object based on the mass. Set the field force vector in the scene to modify the strength of the force.
        #[method(affectedByGravity)]
        pub unsafe fn affectedByGravity(&self) -> bool;

        /// Setter for [`affectedByGravity`][Self::affectedByGravity].
        #[method(setAffectedByGravity:)]
        pub unsafe fn setAffectedByGravity(&self, affected_by_gravity: bool);

        /// Defines what logical 'categories' of fields this body responds to. Defaults to all bits set (all categories).
        /// Can be forced off via affectedByGravity.
        #[method(fieldBitMask)]
        pub unsafe fn fieldBitMask(&self) -> u32;

        /// Setter for [`fieldBitMask`][Self::fieldBitMask].
        #[method(setFieldBitMask:)]
        pub unsafe fn setFieldBitMask(&self, field_bit_mask: u32);

        /// Defines what logical 'categories' this body belongs to. Defaults to all bits set (all categories).
        #[method(categoryBitMask)]
        pub unsafe fn categoryBitMask(&self) -> u32;

        /// Setter for [`categoryBitMask`][Self::categoryBitMask].
        #[method(setCategoryBitMask:)]
        pub unsafe fn setCategoryBitMask(&self, category_bit_mask: u32);

        /// Defines what logical 'categories' of bodies this body responds to collisions with. Defaults to all bits set (all categories).
        #[method(collisionBitMask)]
        pub unsafe fn collisionBitMask(&self) -> u32;

        /// Setter for [`collisionBitMask`][Self::collisionBitMask].
        #[method(setCollisionBitMask:)]
        pub unsafe fn setCollisionBitMask(&self, collision_bit_mask: u32);

        /// Defines what logical 'categories' of bodies this body generates intersection notifications with. Defaults to all bits cleared (no categories).
        #[method(contactTestBitMask)]
        pub unsafe fn contactTestBitMask(&self) -> u32;

        /// Setter for [`contactTestBitMask`][Self::contactTestBitMask].
        #[method(setContactTestBitMask:)]
        pub unsafe fn setContactTestBitMask(&self, contact_test_bit_mask: u32);

        #[cfg(feature = "SKPhysicsJoint")]
        #[method_id(@__retain_semantics Other joints)]
        pub unsafe fn joints(&self) -> Retained<NSArray<SKPhysicsJoint>>;

        #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// The representedObject this physicsBody is currently bound to, or nil if it is not.
        #[method_id(@__retain_semantics Other node)]
        pub unsafe fn node(&self, mtm: MainThreadMarker) -> Option<Retained<SKNode>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(velocity)]
        pub unsafe fn velocity(&self) -> CGVector;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`velocity`][Self::velocity].
        #[method(setVelocity:)]
        pub unsafe fn setVelocity(&self, velocity: CGVector);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(angularVelocity)]
        pub unsafe fn angularVelocity(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`angularVelocity`][Self::angularVelocity].
        #[method(setAngularVelocity:)]
        pub unsafe fn setAngularVelocity(&self, angular_velocity: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(applyForce:)]
        pub unsafe fn applyForce(&self, force: CGVector);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(applyForce:atPoint:)]
        pub unsafe fn applyForce_atPoint(&self, force: CGVector, point: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(applyTorque:)]
        pub unsafe fn applyTorque(&self, torque: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(applyImpulse:)]
        pub unsafe fn applyImpulse(&self, impulse: CGVector);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(applyImpulse:atPoint:)]
        pub unsafe fn applyImpulse_atPoint(&self, impulse: CGVector, point: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(applyAngularImpulse:)]
        pub unsafe fn applyAngularImpulse(&self, impulse: CGFloat);

        #[method_id(@__retain_semantics Other allContactedBodies)]
        pub unsafe fn allContactedBodies(&self) -> Retained<NSArray<SKPhysicsBody>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKPhysicsBody {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);