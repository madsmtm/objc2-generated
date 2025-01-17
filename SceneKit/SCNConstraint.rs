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
    /// A SCNConstraint is an abstract class that represents a single constraint that can be applied to a node.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnconstraint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNConstraint;
);

unsafe impl NSCoding for SCNConstraint {}

unsafe impl NSCopying for SCNConstraint {}

unsafe impl CopyingHelper for SCNConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNConstraint {}

unsafe impl NSSecureCoding for SCNConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNConstraint {}

extern_methods!(
    unsafe impl SCNConstraint {
        /// Determines whether the constraint is enabled or not. Defaults to YES.
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the inflence factor of the receiver. Defaults to 1. Animatable
        #[method(influenceFactor)]
        pub unsafe fn influenceFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`influenceFactor`][Self::influenceFactor].
        #[method(setInfluenceFactor:)]
        pub unsafe fn setInfluenceFactor(&self, influence_factor: CGFloat);

        /// Specifies whether or not the contraint should applies incrementally and have it's effect being cumulated over the rendered frames. Defaults to YES starting macOS 10.13, iOS 11, tvOS 11 and watchOS 4. Defaults to NO in previous versions.
        #[method(isIncremental)]
        pub unsafe fn isIncremental(&self) -> bool;

        /// Setter for [`isIncremental`][Self::isIncremental].
        #[method(setIncremental:)]
        pub unsafe fn setIncremental(&self, incremental: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A SCNLookAtConstraint applies on a node's orientation so that it always look at another node.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlookatconstraint?language=objc)
    #[unsafe(super(SCNConstraint, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNLookAtConstraint;
);

unsafe impl NSCoding for SCNLookAtConstraint {}

unsafe impl NSCopying for SCNLookAtConstraint {}

unsafe impl CopyingHelper for SCNLookAtConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNLookAtConstraint {}

unsafe impl NSSecureCoding for SCNLookAtConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNLookAtConstraint {}

extern_methods!(
    unsafe impl SCNLookAtConstraint {
        #[cfg(feature = "SCNNode")]
        /// Creates and returns a SCNLookAtConstraint object with the specified target.
        ///
        /// Parameter `target`: The target node to look at.
        #[unsafe(method_family(none))]
        #[method_id(lookAtConstraintWithTarget:)]
        pub unsafe fn lookAtConstraintWithTarget(target: Option<&SCNNode>) -> Retained<Self>;

        #[cfg(feature = "SCNNode")]
        /// Defines the target node to look at.
        #[unsafe(method_family(none))]
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Retained<SCNNode>>;

        #[cfg(feature = "SCNNode")]
        /// Setter for [`target`][Self::target].
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&SCNNode>);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Offset look at position in target space. Defaults to zero. Animatable
        #[method(targetOffset)]
        pub unsafe fn targetOffset(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`targetOffset`][Self::targetOffset].
        #[method(setTargetOffset:)]
        pub unsafe fn setTargetOffset(&self, target_offset: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Front direction in the constraint owner local space. Defaults to -[SCNNode localFront]. Animatable
        #[method(localFront)]
        pub unsafe fn localFront(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`localFront`][Self::localFront].
        #[method(setLocalFront:)]
        pub unsafe fn setLocalFront(&self, local_front: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Up reference direction in world space. Defaults to -[SCNNode localUp]. Animatable
        #[method(worldUp)]
        pub unsafe fn worldUp(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`worldUp`][Self::worldUp].
        #[method(setWorldUp:)]
        pub unsafe fn setWorldUp(&self, world_up: SCNVector3);

        /// Specifies whether the receiver enables the gimbal lock. Defaults to NO.
        ///
        /// Enabling the gimbal lock prevents the receiver from rotating the constrained node around to roll axis.
        #[method(gimbalLockEnabled)]
        pub unsafe fn gimbalLockEnabled(&self) -> bool;

        /// Setter for [`gimbalLockEnabled`][Self::gimbalLockEnabled].
        #[method(setGimbalLockEnabled:)]
        pub unsafe fn setGimbalLockEnabled(&self, gimbal_lock_enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNLookAtConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnbillboardaxis?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNBillboardAxis(pub NSUInteger);
bitflags::bitflags! {
    impl SCNBillboardAxis: NSUInteger {
        #[doc(alias = "SCNBillboardAxisX")]
        const X = 0x1<<0;
        #[doc(alias = "SCNBillboardAxisY")]
        const Y = 0x1<<1;
        #[doc(alias = "SCNBillboardAxisZ")]
        const Z = 0x1<<2;
        #[doc(alias = "SCNBillboardAxisAll")]
        const All = SCNBillboardAxis::X.0|SCNBillboardAxis::Y.0|SCNBillboardAxis::Z.0;
    }
}

unsafe impl Encode for SCNBillboardAxis {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for SCNBillboardAxis {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnbillboardconstraint?language=objc)
    #[unsafe(super(SCNConstraint, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNBillboardConstraint;
);

unsafe impl NSCoding for SCNBillboardConstraint {}

unsafe impl NSCopying for SCNBillboardConstraint {}

unsafe impl CopyingHelper for SCNBillboardConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNBillboardConstraint {}

unsafe impl NSSecureCoding for SCNBillboardConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNBillboardConstraint {}

extern_methods!(
    unsafe impl SCNBillboardConstraint {
        /// Creates and returns a SCNBillboardConstraint constraint.
        ///
        /// A billboard constraint forces the receiver to look into the direction of the current point of view.
        #[unsafe(method_family(none))]
        #[method_id(billboardConstraint)]
        pub unsafe fn billboardConstraint() -> Retained<Self>;

        /// Specifies the axes on which the billboarding orientation operates. Defaults to SCNBillboardAxisAll.
        #[method(freeAxes)]
        pub unsafe fn freeAxes(&self) -> SCNBillboardAxis;

        /// Setter for [`freeAxes`][Self::freeAxes].
        #[method(setFreeAxes:)]
        pub unsafe fn setFreeAxes(&self, free_axes: SCNBillboardAxis);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNBillboardConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A SCNTransformConstraint applies on the transform of a node via a custom block.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scntransformconstraint?language=objc)
    #[unsafe(super(SCNConstraint, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNTransformConstraint;
);

unsafe impl NSCoding for SCNTransformConstraint {}

unsafe impl NSCopying for SCNTransformConstraint {}

unsafe impl CopyingHelper for SCNTransformConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNTransformConstraint {}

unsafe impl NSSecureCoding for SCNTransformConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNTransformConstraint {}

extern_methods!(
    unsafe impl SCNTransformConstraint {
        #[cfg(all(
            feature = "SCNNode",
            feature = "SceneKitTypes",
            feature = "block2",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        /// Creates and returns a SCNTransformConstraint object with the specified parameters.
        ///
        /// Parameter `world`: Determines whether the constraint is evaluated in world or local space.
        ///
        /// Parameter `block`: The custom block to call to evaluate the constraint.
        ///
        /// The node and its transform are passed to the block. The transform returned by the block will be used to render the node.
        #[unsafe(method_family(none))]
        #[method_id(transformConstraintInWorldSpace:withBlock:)]
        pub unsafe fn transformConstraintInWorldSpace_withBlock(
            world: bool,
            block: &block2::Block<dyn Fn(NonNull<SCNNode>, SCNMatrix4) -> SCNMatrix4>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "SCNNode",
            feature = "SceneKitTypes",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        /// Creates and returns a SCNTransformConstraint object with the specified parameters.
        ///
        /// Parameter `world`: Determines whether the constraint is evaluated in world or local space.
        ///
        /// Parameter `block`: The custom block to call to evaluate the constraint.
        ///
        /// The node and its position are passed to the block. The position returned by the block will be used to render the node.
        #[unsafe(method_family(none))]
        #[method_id(positionConstraintInWorldSpace:withBlock:)]
        pub unsafe fn positionConstraintInWorldSpace_withBlock(
            world: bool,
            block: &block2::Block<dyn Fn(NonNull<SCNNode>, SCNVector3) -> SCNVector3>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "SCNNode",
            feature = "SceneKitTypes",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        /// Creates and returns a SCNTransformConstraint object with the specified parameters.
        ///
        /// Parameter `world`: Determines whether the constraint is evaluated in world or local space.
        ///
        /// Parameter `block`: The custom block to call to evaluate the constraint.
        ///
        /// The node and its quaternion are passed to the block. The quaternion returned by the block will be used to render the node.
        #[unsafe(method_family(none))]
        #[method_id(orientationConstraintInWorldSpace:withBlock:)]
        pub unsafe fn orientationConstraintInWorldSpace_withBlock(
            world: bool,
            block: &block2::Block<dyn Fn(NonNull<SCNNode>, SCNQuaternion) -> SCNQuaternion>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNTransformConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A SCNIKConstraint applies an inverse kinematics constraint
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnikconstraint?language=objc)
    #[unsafe(super(SCNConstraint, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNIKConstraint;
);

unsafe impl NSCoding for SCNIKConstraint {}

unsafe impl NSCopying for SCNIKConstraint {}

unsafe impl CopyingHelper for SCNIKConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNIKConstraint {}

unsafe impl NSSecureCoding for SCNIKConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNIKConstraint {}

extern_methods!(
    unsafe impl SCNIKConstraint {
        #[cfg(feature = "SCNNode")]
        /// Creates and returns a SCNIKConstraint object with the specified parameter.
        ///
        /// Parameter `chainRootNode`: The root node of the kinematic chain.
        ///
        /// "chainRootNode" must be an ancestor of the node on which the constraint is applied.
        #[unsafe(method_family(init))]
        #[method_id(initWithChainRootNode:)]
        pub unsafe fn initWithChainRootNode(
            this: Allocated<Self>,
            chain_root_node: &SCNNode,
        ) -> Retained<Self>;

        #[cfg(feature = "SCNNode")]
        /// Creates and returns a SCNIKConstraint object with the specified parameter.
        ///
        /// Parameter `chainRootNode`: The root node of the kinematic chain.
        ///
        /// "chainRootNode" must be an ancestor of the node on which the constraint is applied.
        #[unsafe(method_family(none))]
        #[method_id(inverseKinematicsConstraintWithChainRootNode:)]
        pub unsafe fn inverseKinematicsConstraintWithChainRootNode(
            chain_root_node: &SCNNode,
        ) -> Retained<Self>;

        #[cfg(feature = "SCNNode")]
        /// Specifies the root node of the kinematic chain.
        #[unsafe(method_family(none))]
        #[method_id(chainRootNode)]
        pub unsafe fn chainRootNode(&self) -> Retained<SCNNode>;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Specifies the target position (in world space coordinates) of the end joint (i.e the node that owns the IK constraint). Defaults to (0,0,0). Animatable.
        #[method(targetPosition)]
        pub unsafe fn targetPosition(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`targetPosition`][Self::targetPosition].
        #[method(setTargetPosition:)]
        pub unsafe fn setTargetPosition(&self, target_position: SCNVector3);

        #[cfg(all(feature = "SCNNode", feature = "objc2-core-foundation"))]
        /// Specifies the maximum rotation allowed (in degrees) for the specified joint from its initial orientation. Defaults to 180.
        #[method(setMaxAllowedRotationAngle:forJoint:)]
        pub unsafe fn setMaxAllowedRotationAngle_forJoint(&self, angle: CGFloat, node: &SCNNode);

        #[cfg(all(feature = "SCNNode", feature = "objc2-core-foundation"))]
        #[method(maxAllowedRotationAngleForJoint:)]
        pub unsafe fn maxAllowedRotationAngleForJoint(&self, node: &SCNNode) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNIKConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A SCNDistanceConstraint ensure a minimum/maximum distance with a target node.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scndistanceconstraint?language=objc)
    #[unsafe(super(SCNConstraint, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNDistanceConstraint;
);

unsafe impl NSCoding for SCNDistanceConstraint {}

unsafe impl NSCopying for SCNDistanceConstraint {}

unsafe impl CopyingHelper for SCNDistanceConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNDistanceConstraint {}

unsafe impl NSSecureCoding for SCNDistanceConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNDistanceConstraint {}

extern_methods!(
    unsafe impl SCNDistanceConstraint {
        #[cfg(feature = "SCNNode")]
        /// Creates and returns a SCNDistanceConstraint constraint.
        #[unsafe(method_family(none))]
        #[method_id(distanceConstraintWithTarget:)]
        pub unsafe fn distanceConstraintWithTarget(target: Option<&SCNNode>) -> Retained<Self>;

        #[cfg(feature = "SCNNode")]
        /// Defines the target node to keep distance with.
        #[unsafe(method_family(none))]
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Retained<SCNNode>>;

        #[cfg(feature = "SCNNode")]
        /// Setter for [`target`][Self::target].
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&SCNNode>);

        #[cfg(feature = "objc2-core-foundation")]
        /// The minimum distance. Defaults to 0. Animatable.
        #[method(minimumDistance)]
        pub unsafe fn minimumDistance(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumDistance`][Self::minimumDistance].
        #[method(setMinimumDistance:)]
        pub unsafe fn setMinimumDistance(&self, minimum_distance: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// The minimum distance. Defaults to MAXFLOAT. Animatable.
        #[method(maximumDistance)]
        pub unsafe fn maximumDistance(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maximumDistance`][Self::maximumDistance].
        #[method(setMaximumDistance:)]
        pub unsafe fn setMaximumDistance(&self, maximum_distance: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNDistanceConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A SCNReplicatorConstraint replicates the position/orientation/scale of a target node
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnreplicatorconstraint?language=objc)
    #[unsafe(super(SCNConstraint, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNReplicatorConstraint;
);

unsafe impl NSCoding for SCNReplicatorConstraint {}

unsafe impl NSCopying for SCNReplicatorConstraint {}

unsafe impl CopyingHelper for SCNReplicatorConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNReplicatorConstraint {}

unsafe impl NSSecureCoding for SCNReplicatorConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNReplicatorConstraint {}

extern_methods!(
    unsafe impl SCNReplicatorConstraint {
        #[cfg(feature = "SCNNode")]
        /// Creates and returns a SCNReplicatorConstraint constraint.
        #[unsafe(method_family(none))]
        #[method_id(replicatorConstraintWithTarget:)]
        pub unsafe fn replicatorConstraintWithTarget(target: Option<&SCNNode>) -> Retained<Self>;

        #[cfg(feature = "SCNNode")]
        /// Defines the target node to replicate
        #[unsafe(method_family(none))]
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Retained<SCNNode>>;

        #[cfg(feature = "SCNNode")]
        /// Setter for [`target`][Self::target].
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&SCNNode>);

        /// Defines whether or not the constraint should replicate the target orientation. Defaults to YES.
        #[method(replicatesOrientation)]
        pub unsafe fn replicatesOrientation(&self) -> bool;

        /// Setter for [`replicatesOrientation`][Self::replicatesOrientation].
        #[method(setReplicatesOrientation:)]
        pub unsafe fn setReplicatesOrientation(&self, replicates_orientation: bool);

        /// Defines whether or not the constraint should replicate the target position. Defaults to YES.
        #[method(replicatesPosition)]
        pub unsafe fn replicatesPosition(&self) -> bool;

        /// Setter for [`replicatesPosition`][Self::replicatesPosition].
        #[method(setReplicatesPosition:)]
        pub unsafe fn setReplicatesPosition(&self, replicates_position: bool);

        /// Defines whether or not the constraint should replicate the target scale. Defaults to YES.
        #[method(replicatesScale)]
        pub unsafe fn replicatesScale(&self) -> bool;

        /// Setter for [`replicatesScale`][Self::replicatesScale].
        #[method(setReplicatesScale:)]
        pub unsafe fn setReplicatesScale(&self, replicates_scale: bool);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Defines an addition orientation offset. Defaults to no offset. Animatable.
        #[method(orientationOffset)]
        pub unsafe fn orientationOffset(&self) -> SCNQuaternion;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`orientationOffset`][Self::orientationOffset].
        #[method(setOrientationOffset:)]
        pub unsafe fn setOrientationOffset(&self, orientation_offset: SCNQuaternion);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Defines an addition orientation offset. Defaults to no offset. Animatable.
        #[method(positionOffset)]
        pub unsafe fn positionOffset(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`positionOffset`][Self::positionOffset].
        #[method(setPositionOffset:)]
        pub unsafe fn setPositionOffset(&self, position_offset: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Defines an addition scale offset. Defaults to no offset. Animatable.
        #[method(scaleOffset)]
        pub unsafe fn scaleOffset(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`scaleOffset`][Self::scaleOffset].
        #[method(setScaleOffset:)]
        pub unsafe fn setScaleOffset(&self, scale_offset: SCNVector3);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNReplicatorConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A SCNAccelerationConstraint caps the acceleration and velocity of a node
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnaccelerationconstraint?language=objc)
    #[unsafe(super(SCNConstraint, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNAccelerationConstraint;
);

unsafe impl NSCoding for SCNAccelerationConstraint {}

unsafe impl NSCopying for SCNAccelerationConstraint {}

unsafe impl CopyingHelper for SCNAccelerationConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNAccelerationConstraint {}

unsafe impl NSSecureCoding for SCNAccelerationConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNAccelerationConstraint {}

extern_methods!(
    unsafe impl SCNAccelerationConstraint {
        /// Creates and returns a SCNAccelerationConstraint object.
        #[unsafe(method_family(none))]
        #[method_id(accelerationConstraint)]
        pub unsafe fn accelerationConstraint() -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Controls the maximum linear acceleration. Defaults to MAXFLOAT. Animatable.
        ///
        /// The maximum linear acceleration is in m.s^-2
        #[method(maximumLinearAcceleration)]
        pub unsafe fn maximumLinearAcceleration(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maximumLinearAcceleration`][Self::maximumLinearAcceleration].
        #[method(setMaximumLinearAcceleration:)]
        pub unsafe fn setMaximumLinearAcceleration(&self, maximum_linear_acceleration: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Controls the maximum linear velocity. Defaults to MAXFLOAT. Animatable.
        ///
        /// The maximum linear velocity is in m.s
        #[method(maximumLinearVelocity)]
        pub unsafe fn maximumLinearVelocity(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maximumLinearVelocity`][Self::maximumLinearVelocity].
        #[method(setMaximumLinearVelocity:)]
        pub unsafe fn setMaximumLinearVelocity(&self, maximum_linear_velocity: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Controls the distance at which the node should start decelerating. Defaults to 0. Animatable.
        #[method(decelerationDistance)]
        pub unsafe fn decelerationDistance(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`decelerationDistance`][Self::decelerationDistance].
        #[method(setDecelerationDistance:)]
        pub unsafe fn setDecelerationDistance(&self, deceleration_distance: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        /// Specifies the damping factor of the receiver. Optionally reduce the body's linear velocity each frame to simulate fluid/air friction. Value should be zero or greater. Defaults to 0.1. Animatable.
        #[method(damping)]
        pub unsafe fn damping(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`damping`][Self::damping].
        #[method(setDamping:)]
        pub unsafe fn setDamping(&self, damping: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNAccelerationConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A SCNSliderConstraint constraint makes a node to collide and slide against a category of nodes
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnsliderconstraint?language=objc)
    #[unsafe(super(SCNConstraint, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNSliderConstraint;
);

unsafe impl NSCoding for SCNSliderConstraint {}

unsafe impl NSCopying for SCNSliderConstraint {}

unsafe impl CopyingHelper for SCNSliderConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNSliderConstraint {}

unsafe impl NSSecureCoding for SCNSliderConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNSliderConstraint {}

extern_methods!(
    unsafe impl SCNSliderConstraint {
        /// Creates and returns a SCNSliderConstraint object.
        #[unsafe(method_family(none))]
        #[method_id(sliderConstraint)]
        pub unsafe fn sliderConstraint() -> Retained<Self>;

        /// Defines the category of node to collide against. Defaults to 0.
        #[method(collisionCategoryBitMask)]
        pub unsafe fn collisionCategoryBitMask(&self) -> NSUInteger;

        /// Setter for [`collisionCategoryBitMask`][Self::collisionCategoryBitMask].
        #[method(setCollisionCategoryBitMask:)]
        pub unsafe fn setCollisionCategoryBitMask(&self, collision_category_bit_mask: NSUInteger);

        #[cfg(feature = "objc2-core-foundation")]
        /// Defines the radius of the slider. Defaults to 1.
        #[method(radius)]
        pub unsafe fn radius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`radius`][Self::radius].
        #[method(setRadius:)]
        pub unsafe fn setRadius(&self, radius: CGFloat);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Defines the offset of the slider. Defaults to (0,0,0).
        #[method(offset)]
        pub unsafe fn offset(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`offset`][Self::offset].
        #[method(setOffset:)]
        pub unsafe fn setOffset(&self, offset: SCNVector3);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNSliderConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnavoidoccluderconstraintdelegate?language=objc)
    pub unsafe trait SCNAvoidOccluderConstraintDelegate: NSObjectProtocol {
        #[cfg(feature = "SCNNode")]
        #[optional]
        #[method(avoidOccluderConstraint:shouldAvoidOccluder:forNode:)]
        unsafe fn avoidOccluderConstraint_shouldAvoidOccluder_forNode(
            &self,
            constraint: &SCNAvoidOccluderConstraint,
            occluder: &SCNNode,
            node: &SCNNode,
        ) -> bool;

        #[cfg(feature = "SCNNode")]
        #[optional]
        #[method(avoidOccluderConstraint:didAvoidOccluder:forNode:)]
        unsafe fn avoidOccluderConstraint_didAvoidOccluder_forNode(
            &self,
            constraint: &SCNAvoidOccluderConstraint,
            occluder: &SCNNode,
            node: &SCNNode,
        );
    }
);

extern_class!(
    /// A SCNAvoidOccluderConstraint constraints place the receiver at a position that prevent nodes with the specified category to occlude the target.
    ///
    /// The target node and it's children are ignored as potential occluders.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnavoidoccluderconstraint?language=objc)
    #[unsafe(super(SCNConstraint, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNAvoidOccluderConstraint;
);

unsafe impl NSCoding for SCNAvoidOccluderConstraint {}

unsafe impl NSCopying for SCNAvoidOccluderConstraint {}

unsafe impl CopyingHelper for SCNAvoidOccluderConstraint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNAvoidOccluderConstraint {}

unsafe impl NSSecureCoding for SCNAvoidOccluderConstraint {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNAvoidOccluderConstraint {}

extern_methods!(
    unsafe impl SCNAvoidOccluderConstraint {
        #[cfg(feature = "SCNNode")]
        /// Creates and returns a SCNAvoidOccluderConstraint object.
        #[unsafe(method_family(none))]
        #[method_id(avoidOccluderConstraintWithTarget:)]
        pub unsafe fn avoidOccluderConstraintWithTarget(target: Option<&SCNNode>)
            -> Retained<Self>;

        /// The receiver's delegate
        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Retained<ProtocolObject<dyn SCNAvoidOccluderConstraintDelegate>>;

        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: &ProtocolObject<dyn SCNAvoidOccluderConstraintDelegate>,
        );

        #[cfg(feature = "SCNNode")]
        /// Defines the target node
        #[unsafe(method_family(none))]
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Retained<SCNNode>>;

        #[cfg(feature = "SCNNode")]
        /// Setter for [`target`][Self::target].
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&SCNNode>);

        /// Defines the category of node to consider as occluder. Defaults to 1.
        #[method(occluderCategoryBitMask)]
        pub unsafe fn occluderCategoryBitMask(&self) -> NSUInteger;

        /// Setter for [`occluderCategoryBitMask`][Self::occluderCategoryBitMask].
        #[method(setOccluderCategoryBitMask:)]
        pub unsafe fn setOccluderCategoryBitMask(&self, occluder_category_bit_mask: NSUInteger);

        #[cfg(feature = "objc2-core-foundation")]
        /// Defines the bias the apply after moving the receiver to avoid occluders. Defaults to 10e-5.
        ///
        /// A positive bias will move the receiver closer to the target.
        #[method(bias)]
        pub unsafe fn bias(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`bias`][Self::bias].
        #[method(setBias:)]
        pub unsafe fn setBias(&self, bias: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNAvoidOccluderConstraint {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
