//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifieldbehavior?language=objc)
    #[unsafe(super(UIDynamicBehavior, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDynamicBehavior")]
    pub struct UIFieldBehavior;
);

#[cfg(feature = "UIDynamicBehavior")]
extern_conformance!(
    unsafe impl NSObjectProtocol for UIFieldBehavior {}
);

#[cfg(feature = "UIDynamicBehavior")]
impl UIFieldBehavior {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(addItem:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[unsafe(method(removeItem:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[unsafe(method(items))]
        #[unsafe(method_family = none)]
        pub unsafe fn items(&self) -> Retained<NSArray<ProtocolObject<dyn UIDynamicItem>>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// The position (origin) of the field in the reference coordinate system
        #[unsafe(method(position))]
        #[unsafe(method_family = none)]
        pub unsafe fn position(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`position`][Self::position].
        #[unsafe(method(setPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPosition(&self, position: CGPoint);

        #[cfg(feature = "UIRegion")]
        /// The region property is the domain of the field's effect. No force is applied to objects outside the region.
        /// The default region is the infiniteRegion
        #[unsafe(method(region))]
        #[unsafe(method_family = none)]
        pub unsafe fn region(&self) -> Retained<UIRegion>;

        #[cfg(feature = "UIRegion")]
        /// Setter for [`region`][Self::region].
        #[unsafe(method(setRegion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRegion(&self, region: &UIRegion);

        #[cfg(feature = "objc2-core-foundation")]
        /// Strength scaling value. default 1.0
        #[unsafe(method(strength))]
        #[unsafe(method_family = none)]
        pub unsafe fn strength(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`strength`][Self::strength].
        #[unsafe(method(setStrength:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStrength(&self, strength: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The falloff exponent used to calculate field strength at a distance.
        /// Falloff starts at the minimum radius.
        /// The default exponent is zero, which results in a uniform field with no falloff.
        ///
        /// See: minimumRadius
        #[unsafe(method(falloff))]
        #[unsafe(method_family = none)]
        pub unsafe fn falloff(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`falloff`][Self::falloff].
        #[unsafe(method(setFalloff:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFalloff(&self, falloff: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Minimum radius of effect. Default is very small.
        #[unsafe(method(minimumRadius))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumRadius`][Self::minimumRadius].
        #[unsafe(method(setMinimumRadius:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimumRadius(&self, minimum_radius: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The direction of the field.
        /// If the field is non-directional, a zero vector will be returned
        ///
        /// See: linearGravityFieldWithVector:direction
        ///
        /// See: velocityFieldWithVector:direction
        #[unsafe(method(direction))]
        #[unsafe(method_family = none)]
        pub unsafe fn direction(&self) -> CGVector;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`direction`][Self::direction].
        #[unsafe(method(setDirection:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDirection(&self, direction: CGVector);

        #[cfg(feature = "objc2-core-foundation")]
        /// Fields without a smoothness component will return 0
        ///
        /// See: noiseFieldWithSmoothness:smoothness:animationSpeed
        ///
        /// See: turbulenceFieldWithSmoothness:smoothness:animationSpeed
        #[unsafe(method(smoothness))]
        #[unsafe(method_family = none)]
        pub unsafe fn smoothness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`smoothness`][Self::smoothness].
        #[unsafe(method(setSmoothness:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSmoothness(&self, smoothness: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Fields that can be animated can have non zero values. A value of 2 will animate twice as fast as a value of 1.
        ///
        /// See: noiseFieldWithSmoothness:smoothness:animationSpeed
        ///
        /// See: turbulenceFieldWithSmoothness:smoothness:animationSpeed
        #[unsafe(method(animationSpeed))]
        #[unsafe(method_family = none)]
        pub unsafe fn animationSpeed(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`animationSpeed`][Self::animationSpeed].
        #[unsafe(method(setAnimationSpeed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAnimationSpeed(&self, animation_speed: CGFloat);

        /// Slows an object proportionally to the object’s velocity.
        /// Use this to simulate effects such as friction from motion through the air.
        #[unsafe(method(dragField))]
        #[unsafe(method_family = none)]
        pub unsafe fn dragField(mtm: MainThreadMarker) -> Retained<Self>;

        /// Applies a force tangential to the direction from the sample point to the field's position.
        /// The force will be CCW to the direction. Make the strength negative to apply force in the CW direction.
        /// Amount is proportional to distance from center and the object's mass. This can be used to create rotational effects.
        #[unsafe(method(vortexField))]
        #[unsafe(method_family = none)]
        pub unsafe fn vortexField(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Applies a force in the direction of the origin of the field in local space. To repel objects, use a negative strength.
        /// The force is proportional to the distance from the field origin. Varies with the mass of the object according to F = ma
        ///
        /// Parameter `position`: the origin of the field
        ///
        /// See: position
        #[unsafe(method(radialGravityFieldWithPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn radialGravityFieldWithPosition(
            position: CGPoint,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Applies a force in the direction of the vector in the local space. To repel objects, use a negative strength.
        /// The force is the same everywhere in the field. Varies with the mass of the object according to F = ma
        ///
        /// Parameter `direction`: The direction the force is applied in the x,y plane. The length of the direction vector is multiplied by
        /// the field's strength property to get the final calculated force. All components of the direction vector are used to calculate the length.
        ///
        /// See: direction
        #[unsafe(method(linearGravityFieldWithVector:))]
        #[unsafe(method_family = none)]
        pub unsafe fn linearGravityFieldWithVector(
            direction: CGVector,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Uses the supplied velocity vector for any object entering the field’s region of effect.
        /// Velocity fields override the effect of any other acceleration applied to the body.
        ///
        /// Parameter `direction`: The directed velocity that will be applied to the body.
        ///
        /// See: direction
        #[unsafe(method(velocityFieldWithVector:))]
        #[unsafe(method_family = none)]
        pub unsafe fn velocityFieldWithVector(
            direction: CGVector,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// A time varying differentiable Perlin simplex noise field. By default a smooth noise is calculated,
        /// and the field is time varying. To freeze the noise in place, set animationSpeed to 0.0. Mass is ignored.
        ///
        /// Parameter `smoothness`: value of 0 means as noisy as possible, 1 means as smooth as possible
        ///
        /// Parameter `animationSpeed`: is the general field rate of change in Hz
        ///
        /// See: smoothness
        ///
        /// See: animationSpeed
        #[unsafe(method(noiseFieldWithSmoothness:animationSpeed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn noiseFieldWithSmoothness_animationSpeed(
            smoothness: CGFloat,
            speed: CGFloat,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Just like Noise, except the strength of the noise is proportional to the velocity of the object in the field.
        ///
        /// Parameter `smoothness`: value of 0 means as noisy as possible, 1 means as smooth as possible
        ///
        /// Parameter `animationSpeed`: is the general field rate of change in Hz
        ///
        /// See: smoothness
        ///
        /// See: animationSpeed
        #[unsafe(method(turbulenceFieldWithSmoothness:animationSpeed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn turbulenceFieldWithSmoothness_animationSpeed(
            smoothness: CGFloat,
            speed: CGFloat,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        /// A Hooke’s law force - a force linearly proportional to distance from the center of the field. An object in this
        /// field will oscillate with a period proportional to the inverse of the mass.
        /// An example use is to keep objects confined to a particular region.
        #[unsafe(method(springField))]
        #[unsafe(method_family = none)]
        pub unsafe fn springField(mtm: MainThreadMarker) -> Retained<Self>;

        /// A force proportional to the charge on the object. A charge property has been
        /// added to UIDynamicItemBehavior to accomplish this. An example use of this field is to make objects behavior differently
        /// from one another when they enter a region, or to make an object's behavior different than its mass based behavior
        /// This field models the first part of the Lorentz equation, F = qE
        #[unsafe(method(electricField))]
        #[unsafe(method_family = none)]
        pub unsafe fn electricField(mtm: MainThreadMarker) -> Retained<Self>;

        /// The magnetic field is a uniform field in the positive-z direction (coming out of the screen). When the velocity
        /// of a charged dynamic item is perpendicular to the uniform magnetic field, the item feels a resulting force normal
        /// to both the velocity and the B field. This results CCW circular motion. You can adjust the strength of the B field
        /// to be negative which will result in circular motion being CW instead of CCW. An example use of this field is to make
        /// objects behavior differently from one another when they enter a region, or to make an object's behavior different
        /// than its mass based behavior. This field models the second part of the Lorentz equation, F = qvB
        #[unsafe(method(magneticField))]
        #[unsafe(method_family = none)]
        pub unsafe fn magneticField(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
        /// A field force with a custom force evaluator.
        ///
        /// Parameter `field`: the field being evaluated
        ///
        /// Parameter `position`: The location to evaluate the force at
        ///
        /// Parameter `velocity`: The velocity to be considered during force evaluation. Useful for calculating drag.
        ///
        /// Parameter `mass`: The mass to be taken into account during force evaluation
        ///
        /// Parameter `charge`: The charge to be taken into account during force evaluation
        ///
        /// Parameter `deltaTime`: The current time step
        #[unsafe(method(fieldWithEvaluationBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn fieldWithEvaluationBlock(
            block: &block2::DynBlock<
                dyn Fn(
                    NonNull<UIFieldBehavior>,
                    CGPoint,
                    CGVector,
                    CGFloat,
                    CGFloat,
                    NSTimeInterval,
                ) -> CGVector,
            >,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "UIDynamicBehavior")]
impl UIFieldBehavior {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
