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
    /// SKRange object used to define a range of allowable values
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/skrange?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKRange;
);

extern_conformance!(
    unsafe impl NSCoding for SKRange {}
);

extern_conformance!(
    unsafe impl NSCopying for SKRange {}
);

unsafe impl CopyingHelper for SKRange {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for SKRange {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for SKRange {}
);

impl SKRange {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithLowerLimit:upperLimit:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLowerLimit_upperLimit(
            this: Allocated<Self>,
            lower: CGFloat,
            upper: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rangeWithLowerLimit:upperLimit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rangeWithLowerLimit_upperLimit(
            lower: CGFloat,
            upper: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rangeWithLowerLimit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rangeWithLowerLimit(lower: CGFloat) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rangeWithUpperLimit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rangeWithUpperLimit(upper: CGFloat) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rangeWithConstantValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rangeWithConstantValue(value: CGFloat) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rangeWithValue:variance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rangeWithValue_variance(value: CGFloat, variance: CGFloat) -> Retained<Self>;

        #[unsafe(method(rangeWithNoLimits))]
        #[unsafe(method_family = none)]
        pub unsafe fn rangeWithNoLimits() -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(lowerLimit))]
        #[unsafe(method_family = none)]
        pub unsafe fn lowerLimit(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`lowerLimit`][Self::lowerLimit].
        #[unsafe(method(setLowerLimit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLowerLimit(&self, lower_limit: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(upperLimit))]
        #[unsafe(method_family = none)]
        pub unsafe fn upperLimit(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`upperLimit`][Self::upperLimit].
        #[unsafe(method(setUpperLimit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUpperLimit(&self, upper_limit: CGFloat);
    );
}

/// Methods declared on superclass `NSObject`.
impl SKRange {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// SKConstraints are evaluated each frame after actions and physics.
    /// The node's transform will be changed to staisfy the constarint
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/skconstraint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKConstraint;
);

extern_conformance!(
    unsafe impl NSCoding for SKConstraint {}
);

extern_conformance!(
    unsafe impl NSCopying for SKConstraint {}
);

unsafe impl CopyingHelper for SKConstraint {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for SKConstraint {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for SKConstraint {}
);

impl SKConstraint {
    extern_methods!(
        #[unsafe(method(enabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn enabled(&self) -> bool;

        /// Setter for [`enabled`][Self::enabled].
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[unsafe(method(referenceNode))]
        #[unsafe(method_family = none)]
        pub unsafe fn referenceNode(&self, mtm: MainThreadMarker) -> Option<Retained<SKNode>>;

        #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// Setter for [`referenceNode`][Self::referenceNode].
        #[unsafe(method(setReferenceNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setReferenceNode(&self, reference_node: Option<&SKNode>);

        /// Constrain the node's position to a range
        #[unsafe(method(positionX:))]
        #[unsafe(method_family = none)]
        pub unsafe fn positionX(range: &SKRange) -> Retained<Self>;

        #[unsafe(method(positionY:))]
        #[unsafe(method_family = none)]
        pub unsafe fn positionY(range: &SKRange) -> Retained<Self>;

        #[unsafe(method(positionX:Y:))]
        #[unsafe(method_family = none)]
        pub unsafe fn positionX_Y(x_range: &SKRange, y_range: &SKRange) -> Retained<Self>;

        #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// Constrain the node's position to be within a distance of a point or node
        #[unsafe(method(distance:toNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn distance_toNode(range: &SKRange, node: &SKNode) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(distance:toPoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn distance_toPoint(range: &SKRange, point: CGPoint) -> Retained<Self>;

        #[cfg(all(
            feature = "SKNode",
            feature = "objc2-app-kit",
            feature = "objc2-core-foundation"
        ))]
        #[cfg(target_os = "macos")]
        #[unsafe(method(distance:toPoint:inNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn distance_toPoint_inNode(
            range: &SKRange,
            point: CGPoint,
            node: &SKNode,
        ) -> Retained<Self>;

        /// Constrain the node's rotation to a range
        #[unsafe(method(zRotation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn zRotation(z_range: &SKRange) -> Retained<Self>;

        #[cfg(all(feature = "SKNode", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// Constrain the node's rotation to orient to a point or node
        #[unsafe(method(orientToNode:offset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn orientToNode_offset(node: &SKNode, radians: &SKRange) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(orientToPoint:offset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn orientToPoint_offset(point: CGPoint, radians: &SKRange) -> Retained<Self>;

        #[cfg(all(
            feature = "SKNode",
            feature = "objc2-app-kit",
            feature = "objc2-core-foundation"
        ))]
        #[cfg(target_os = "macos")]
        #[unsafe(method(orientToPoint:inNode:offset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn orientToPoint_inNode_offset(
            point: CGPoint,
            node: &SKNode,
            radians: &SKRange,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl SKConstraint {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
