//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-image")]
#[cfg(not(target_os = "watchos"))]
use objc2_core_image::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern "C" {
    /// Rendering arguments
    ///
    /// These keys are used for the 'semantic' argument of -[SCNProgram setSemantic:forSymbol:options:]
    /// Transforms are SCNMatrix4 wrapped in NSValues.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnmodeltransform?language=objc)
    pub static SCNModelTransform: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnviewtransform?language=objc)
    pub static SCNViewTransform: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnprojectiontransform?language=objc)
    pub static SCNProjectionTransform: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnnormaltransform?language=objc)
    pub static SCNNormalTransform: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnmodelviewtransform?language=objc)
    pub static SCNModelViewTransform: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnmodelviewprojectiontransform?language=objc)
    pub static SCNModelViewProjectionTransform: &'static NSString;
}

/// The available modes of movability.
///
/// Movable nodes are not captured when computing light probes.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnmovabilityhint?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNMovabilityHint(pub NSInteger);
impl SCNMovabilityHint {
    #[doc(alias = "SCNMovabilityHintFixed")]
    pub const Fixed: Self = Self(0);
    #[doc(alias = "SCNMovabilityHintMovable")]
    pub const Movable: Self = Self(1);
}

unsafe impl Encode for SCNMovabilityHint {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNMovabilityHint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Control the focus (UIFocus) behavior.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnnodefocusbehavior?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNNodeFocusBehavior(pub NSInteger);
impl SCNNodeFocusBehavior {
    #[doc(alias = "SCNNodeFocusBehaviorNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "SCNNodeFocusBehaviorOccluding")]
    pub const Occluding: Self = Self(1);
    #[doc(alias = "SCNNodeFocusBehaviorFocusable")]
    pub const Focusable: Self = Self(2);
}

unsafe impl Encode for SCNNodeFocusBehavior {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNNodeFocusBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// SCNNode is the model class for node-tree objects.
    ///
    /// It encapsulates the position, rotations, and other transforms of a node, which define a coordinate system.
    /// The coordinate systems of all the sub-nodes are relative to the one of their parent node.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnnode?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNNode;
);

unsafe impl NSCoding for SCNNode {}

unsafe impl NSCopying for SCNNode {}

unsafe impl CopyingHelper for SCNNode {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SCNNode {}

unsafe impl NSSecureCoding for SCNNode {}

#[cfg(feature = "SCNAction")]
unsafe impl SCNActionable for SCNNode {}

#[cfg(feature = "SCNAnimation")]
unsafe impl SCNAnimatable for SCNNode {}

#[cfg(feature = "SCNBoundingVolume")]
unsafe impl SCNBoundingVolume for SCNNode {}

extern_methods!(
    unsafe impl SCNNode {
        /// Creates and initializes a node instance.
        #[method_id(@__retain_semantics Other node)]
        pub unsafe fn node() -> Retained<Self>;

        #[cfg(feature = "SCNGeometry")]
        /// Creates and initializes a node instance with the specified geometry attached.
        ///
        /// Parameter `geometry`: The geometry to attach.
        #[method_id(@__retain_semantics Other nodeWithGeometry:)]
        pub unsafe fn nodeWithGeometry(geometry: Option<&SCNGeometry>) -> Retained<SCNNode>;

        /// Returns a copy of the receiver. The returned instance is autoreleased.
        ///
        /// The copy is recursive: every child node will be cloned, too. For a non-recursive copy, use copy instead.
        /// The copied nodes will share their attached objects (light, geometry, camera, ...) with the original instances;
        /// if you want, for example, to change the materials of the copy independently of the original object, you'll
        /// have to copy the geometry of the node separately.
        #[method_id(@__retain_semantics Other clone)]
        pub unsafe fn clone(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other flattenedClone)]
        pub unsafe fn flattenedClone(&self) -> Retained<Self>;

        /// Determines the name of the receiver.
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        /// Setter for [`name`][Self::name].
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "SCNLight")]
        /// Determines the light attached to the receiver.
        #[method_id(@__retain_semantics Other light)]
        pub unsafe fn light(&self) -> Option<Retained<SCNLight>>;

        #[cfg(feature = "SCNLight")]
        /// Setter for [`light`][Self::light].
        #[method(setLight:)]
        pub unsafe fn setLight(&self, light: Option<&SCNLight>);

        #[cfg(feature = "SCNCamera")]
        /// Determines the camera attached to the receiver.
        #[method_id(@__retain_semantics Other camera)]
        pub unsafe fn camera(&self) -> Option<Retained<SCNCamera>>;

        #[cfg(feature = "SCNCamera")]
        /// Setter for [`camera`][Self::camera].
        #[method(setCamera:)]
        pub unsafe fn setCamera(&self, camera: Option<&SCNCamera>);

        #[cfg(feature = "SCNGeometry")]
        /// Returns the geometry attached to the receiver.
        #[method_id(@__retain_semantics Other geometry)]
        pub unsafe fn geometry(&self) -> Option<Retained<SCNGeometry>>;

        #[cfg(feature = "SCNGeometry")]
        /// Setter for [`geometry`][Self::geometry].
        #[method(setGeometry:)]
        pub unsafe fn setGeometry(&self, geometry: Option<&SCNGeometry>);

        #[cfg(feature = "SCNSkinner")]
        /// Returns the skinner attached to the receiver.
        #[method_id(@__retain_semantics Other skinner)]
        pub unsafe fn skinner(&self) -> Option<Retained<SCNSkinner>>;

        #[cfg(feature = "SCNSkinner")]
        /// Setter for [`skinner`][Self::skinner].
        #[method(setSkinner:)]
        pub unsafe fn setSkinner(&self, skinner: Option<&SCNSkinner>);

        #[cfg(feature = "SCNMorpher")]
        /// Returns the morpher attached to the receiver.
        #[method_id(@__retain_semantics Other morpher)]
        pub unsafe fn morpher(&self) -> Option<Retained<SCNMorpher>>;

        #[cfg(feature = "SCNMorpher")]
        /// Setter for [`morpher`][Self::morpher].
        #[method(setMorpher:)]
        pub unsafe fn setMorpher(&self, morpher: Option<&SCNMorpher>);

        #[cfg(all(
            feature = "SceneKitTypes",
            feature = "objc2-core-foundation",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        /// Determines the receiver's transform. Animatable.
        ///
        /// The transform is the combination of the position, rotation and scale defined below. So when the transform is set, the receiver's position, rotation and scale are changed to match the new transform.
        #[method(transform)]
        pub unsafe fn transform(&self) -> SCNMatrix4;

        #[cfg(all(
            feature = "SceneKitTypes",
            feature = "objc2-core-foundation",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        /// Setter for [`transform`][Self::transform].
        #[method(setTransform:)]
        pub unsafe fn setTransform(&self, transform: SCNMatrix4);

        #[cfg(all(
            feature = "SceneKitTypes",
            feature = "objc2-core-foundation",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        /// Determines the receiver's transform in world space (relative to the scene's root node). Animatable.
        #[method(worldTransform)]
        pub unsafe fn worldTransform(&self) -> SCNMatrix4;

        #[cfg(all(
            feature = "SceneKitTypes",
            feature = "objc2-core-foundation",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        #[method(setWorldTransform:)]
        pub unsafe fn setWorldTransform(&self, world_transform: SCNMatrix4);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Determines the receiver's position. Animatable.
        #[method(position)]
        pub unsafe fn position(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`position`][Self::position].
        #[method(setPosition:)]
        pub unsafe fn setPosition(&self, position: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Determines the receiver's position in world space (relative to the scene's root node).
        #[method(worldPosition)]
        pub unsafe fn worldPosition(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`worldPosition`][Self::worldPosition].
        #[method(setWorldPosition:)]
        pub unsafe fn setWorldPosition(&self, world_position: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Determines the receiver's rotation. Animatable.
        ///
        /// The rotation is axis angle rotation. The three first components are the axis, the fourth one is the rotation (in radian).
        #[method(rotation)]
        pub unsafe fn rotation(&self) -> SCNVector4;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`rotation`][Self::rotation].
        #[method(setRotation:)]
        pub unsafe fn setRotation(&self, rotation: SCNVector4);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Determines the receiver's orientation as a unit quaternion. Animatable.
        #[method(orientation)]
        pub unsafe fn orientation(&self) -> SCNQuaternion;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`orientation`][Self::orientation].
        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: SCNQuaternion);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Determines the receiver's orientation in world space (relative to the scene's root node). Animatable.
        #[method(worldOrientation)]
        pub unsafe fn worldOrientation(&self) -> SCNQuaternion;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`worldOrientation`][Self::worldOrientation].
        #[method(setWorldOrientation:)]
        pub unsafe fn setWorldOrientation(&self, world_orientation: SCNQuaternion);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Determines the receiver's euler angles. Animatable.
        ///
        /// The order of components in this vector matches the axes of rotation:
        /// 1. Pitch (the x component) is the rotation about the node's x-axis (in radians)
        /// 2. Yaw   (the y component) is the rotation about the node's y-axis (in radians)
        /// 3. Roll  (the z component) is the rotation about the node's z-axis (in radians)
        /// SceneKit applies these rotations in the reverse order of the components:
        /// 1. first roll
        /// 2. then yaw
        /// 3. then pitch
        #[method(eulerAngles)]
        pub unsafe fn eulerAngles(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`eulerAngles`][Self::eulerAngles].
        #[method(setEulerAngles:)]
        pub unsafe fn setEulerAngles(&self, euler_angles: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Determines the receiver's scale. Animatable.
        #[method(scale)]
        pub unsafe fn scale(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Setter for [`scale`][Self::scale].
        #[method(setScale:)]
        pub unsafe fn setScale(&self, scale: SCNVector3);

        #[cfg(all(
            feature = "SceneKitTypes",
            feature = "objc2-core-foundation",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        /// Determines the receiver's pivot. Animatable.
        #[method(pivot)]
        pub unsafe fn pivot(&self) -> SCNMatrix4;

        #[cfg(all(
            feature = "SceneKitTypes",
            feature = "objc2-core-foundation",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        /// Setter for [`pivot`][Self::pivot].
        #[method(setPivot:)]
        pub unsafe fn setPivot(&self, pivot: SCNMatrix4);

        /// Determines whether the receiver is displayed. Defaults to NO. Animatable.
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        /// Setter for [`isHidden`][Self::isHidden].
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[cfg(feature = "objc2-core-foundation")]
        /// Determines the opacity of the receiver. Default is 1. Animatable.
        #[method(opacity)]
        pub unsafe fn opacity(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`opacity`][Self::opacity].
        #[method(setOpacity:)]
        pub unsafe fn setOpacity(&self, opacity: CGFloat);

        /// Determines the rendering order of the receiver.
        ///
        /// Nodes with greater rendering orders are rendered last. Defaults to 0.
        #[method(renderingOrder)]
        pub unsafe fn renderingOrder(&self) -> NSInteger;

        /// Setter for [`renderingOrder`][Self::renderingOrder].
        #[method(setRenderingOrder:)]
        pub unsafe fn setRenderingOrder(&self, rendering_order: NSInteger);

        /// Determines if the node is rendered in shadow maps. Defaults to YES.
        #[method(castsShadow)]
        pub unsafe fn castsShadow(&self) -> bool;

        /// Setter for [`castsShadow`][Self::castsShadow].
        #[method(setCastsShadow:)]
        pub unsafe fn setCastsShadow(&self, casts_shadow: bool);

        /// Communicates to SceneKit’s rendering system about how you want to move content in your scene; it does not affect your ability to change the node’s position or add animations or physics to the node. Defaults to SCNMovabilityHintFixed.
        #[method(movabilityHint)]
        pub unsafe fn movabilityHint(&self) -> SCNMovabilityHint;

        /// Setter for [`movabilityHint`][Self::movabilityHint].
        #[method(setMovabilityHint:)]
        pub unsafe fn setMovabilityHint(&self, movability_hint: SCNMovabilityHint);

        /// Returns the parent node of the receiver.
        #[method_id(@__retain_semantics Other parentNode)]
        pub unsafe fn parentNode(&self) -> Option<Retained<SCNNode>>;

        /// Returns the child node array of the receiver.
        #[method_id(@__retain_semantics Other childNodes)]
        pub unsafe fn childNodes(&self) -> Retained<NSArray<SCNNode>>;

        /// Appends the node to the receiver’s childNodes array.
        ///
        /// Parameter `child`: The node to be added to the receiver’s childNodes array.
        #[method(addChildNode:)]
        pub unsafe fn addChildNode(&self, child: &SCNNode);

        /// Insert a node in the childNodes array at the specified index.
        ///
        /// Parameter `child`: The node to insert.
        ///
        /// Parameter `index`: Index in the childNodes array to insert the node.
        #[method(insertChildNode:atIndex:)]
        pub unsafe fn insertChildNode_atIndex(&self, child: &SCNNode, index: NSUInteger);

        /// Removes the node from the childNodes array of the receiver’s parentNode.
        #[method(removeFromParentNode)]
        pub unsafe fn removeFromParentNode(&self);

        /// Remove `child' from the childNode array of the receiver and insert 'child2' if non-nil in its position.
        ///
        /// If the parentNode of `child' is not the receiver, the behavior is undefined.
        ///
        /// Parameter `oldChild`: The node to replace in the childNodes array.
        ///
        /// Parameter `newChild`: The new node that will replace the previous one.
        #[method(replaceChildNode:with:)]
        pub unsafe fn replaceChildNode_with(&self, old_child: &SCNNode, new_child: &SCNNode);

        /// Returns the first node found in the node tree with the specified name.
        ///
        /// The search uses a pre-order tree traversal.
        ///
        /// Parameter `name`: The name of the node you are searching for.
        ///
        /// Parameter `recursively`: Set to YES if you want the search to look through the sub-nodes recursively.
        #[method_id(@__retain_semantics Other childNodeWithName:recursively:)]
        pub unsafe fn childNodeWithName_recursively(
            &self,
            name: &NSString,
            recursively: bool,
        ) -> Option<Retained<SCNNode>>;

        #[cfg(feature = "block2")]
        /// Returns the child nodes of the receiver that passes a test in a given Block.
        ///
        /// The search is recursive and uses a pre-order tree traversal.
        ///
        /// Parameter `predicate`: The block to apply to child nodes of the receiver. The block takes two arguments: "child" is a child node and "stop" is a reference to a Boolean value. The block can set the value to YES to stop further processing of the node hierarchy. The stop argument is an out-only argument. You should only ever set this Boolean to YES within the Block. The Block returns a Boolean value that indicates whether "child" passed the test.
        #[method_id(@__retain_semantics Other childNodesPassingTest:)]
        pub unsafe fn childNodesPassingTest(
            &self,
            predicate: &block2::Block<dyn Fn(NonNull<SCNNode>, NonNull<Bool>) -> Bool + '_>,
        ) -> Retained<NSArray<SCNNode>>;

        #[cfg(feature = "block2")]
        /// Executes a given block on each child node under the receiver.
        ///
        /// The search is recursive and uses a pre-order tree traversal.
        ///
        /// Parameter `block`: The block to apply to child nodes of the receiver. The block takes two arguments: "child" is a child node and "stop" is a reference to a Boolean value. The block can set the value to YES to stop further processing of the node hierarchy. The stop argument is an out-only argument. You should only ever set this Boolean to YES within the Block.
        #[method(enumerateChildNodesUsingBlock:)]
        pub unsafe fn enumerateChildNodesUsingBlock(
            &self,
            block: &block2::Block<dyn Fn(NonNull<SCNNode>, NonNull<Bool>) + '_>,
        );

        #[cfg(feature = "block2")]
        /// Executes a given block on the receiver and its child nodes.
        ///
        /// The search is recursive and uses a pre-order tree traversal.
        ///
        /// Parameter `block`: The block to apply to the receiver and its child nodes. The block takes two arguments: "node" is a node in the hierarchy of the receiver (including the receiver) and "stop" is a reference to a Boolean value. The block can set the value to YES to stop further processing of the node hierarchy. The stop argument is an out-only argument. You should only ever set this Boolean to YES within the Block.
        #[method(enumerateHierarchyUsingBlock:)]
        pub unsafe fn enumerateHierarchyUsingBlock(
            &self,
            block: &block2::Block<dyn Fn(NonNull<SCNNode>, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Converts a position from the receiver’s coordinate system to that of the specified node.
        ///
        /// Parameter `position`: A position specified in the local coordinate system of the receiver.
        ///
        /// Parameter `node`: The node into whose coordinate system "position" is to be converted. If "node" is nil, this method instead converts to world coordinates.
        #[method(convertPosition:toNode:)]
        pub unsafe fn convertPosition_toNode(
            &self,
            position: SCNVector3,
            node: Option<&SCNNode>,
        ) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Converts a position from the coordinate system of a given node to that of the receiver.
        ///
        /// Parameter `position`: A position specified in the local coordinate system of "node".
        ///
        /// Parameter `node`: The node from whose coordinate system "position" is to be converted. If "node" is nil, this method instead converts from world coordinates.
        #[method(convertPosition:fromNode:)]
        pub unsafe fn convertPosition_fromNode(
            &self,
            position: SCNVector3,
            node: Option<&SCNNode>,
        ) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Converts a vector from the coordinate system of a given node to that of the receiver.
        ///
        ///
        /// Parameter `vector`: A vector specified in the local coordinate system the receiver.
        ///
        /// Parameter `node`: The node defining the space from which the vector should be transformed. If "node" is nil, this method instead converts from world coordinates.
        ///
        ///
        /// Returns: vector transformed from receiver local space to node local space.
        #[method(convertVector:toNode:)]
        pub unsafe fn convertVector_toNode(
            &self,
            vector: SCNVector3,
            node: Option<&SCNNode>,
        ) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Converts a vector from the coordinate system of a given node to that of the receiver.
        ///
        ///
        /// Parameter `vector`: A vector specified in the local coordinate system of "node".
        ///
        /// Parameter `node`: The node defining the space to which the vector should be transformed to. If "node" is nil, this method instead converts from world coordinates.
        ///
        ///
        /// Returns: vector transformed from node space to reveiver local space.
        #[method(convertVector:fromNode:)]
        pub unsafe fn convertVector_fromNode(
            &self,
            vector: SCNVector3,
            node: Option<&SCNNode>,
        ) -> SCNVector3;

        #[cfg(all(
            feature = "SceneKitTypes",
            feature = "objc2-core-foundation",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        /// Converts a transform from the receiver’s coordinate system to that of the specified node.
        ///
        /// Parameter `transform`: A transform specified in the local coordinate system of the receiver.
        ///
        /// Parameter `node`: The node into whose coordinate system "transform" is to be converted. If "node" is nil, this method instead converts to world coordinates.
        #[method(convertTransform:toNode:)]
        pub unsafe fn convertTransform_toNode(
            &self,
            transform: SCNMatrix4,
            node: Option<&SCNNode>,
        ) -> SCNMatrix4;

        #[cfg(all(
            feature = "SceneKitTypes",
            feature = "objc2-core-foundation",
            feature = "objc2-quartz-core"
        ))]
        #[cfg(not(target_os = "watchos"))]
        /// Converts a transform from the coordinate system of a given node to that of the receiver.
        ///
        /// Parameter `transform`: A transform specified in the local coordinate system of "node".
        ///
        /// Parameter `node`: The node from whose coordinate system "transform" is to be converted. If "node" is nil, this method instead converts from world coordinates.
        #[method(convertTransform:fromNode:)]
        pub unsafe fn convertTransform_fromNode(
            &self,
            transform: SCNMatrix4,
            node: Option<&SCNNode>,
        ) -> SCNMatrix4;

        #[cfg(feature = "SCNPhysicsBody")]
        /// The description of the physics body of the receiver.
        ///
        /// Default is nil.
        #[method_id(@__retain_semantics Other physicsBody)]
        pub unsafe fn physicsBody(&self) -> Option<Retained<SCNPhysicsBody>>;

        #[cfg(feature = "SCNPhysicsBody")]
        /// Setter for [`physicsBody`][Self::physicsBody].
        #[method(setPhysicsBody:)]
        pub unsafe fn setPhysicsBody(&self, physics_body: Option<&SCNPhysicsBody>);

        #[cfg(feature = "SCNPhysicsField")]
        /// The description of the physics field of the receiver.
        ///
        /// Default is nil.
        #[method_id(@__retain_semantics Other physicsField)]
        pub unsafe fn physicsField(&self) -> Option<Retained<SCNPhysicsField>>;

        #[cfg(feature = "SCNPhysicsField")]
        /// Setter for [`physicsField`][Self::physicsField].
        #[method(setPhysicsField:)]
        pub unsafe fn setPhysicsField(&self, physics_field: Option<&SCNPhysicsField>);

        #[cfg(feature = "SCNConstraint")]
        /// An array of SCNConstraint that are applied to the receiver.
        ///
        /// Adding or removing a constraint can be implicitly animated based on the current transaction.
        #[method_id(@__retain_semantics Other constraints)]
        pub unsafe fn constraints(&self) -> Option<Retained<NSArray<SCNConstraint>>>;

        #[cfg(feature = "SCNConstraint")]
        /// Setter for [`constraints`][Self::constraints].
        #[method(setConstraints:)]
        pub unsafe fn setConstraints(&self, constraints: Option<&NSArray<SCNConstraint>>);

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        /// An array of Core Image filters that are applied to the rendering of the receiver and its child nodes. Animatable.
        ///
        /// Defaults to nil. Filter properties should be modified by calling setValue:forKeyPath: on each node that the filter is attached to. If the inputs of the filter are modified directly after the filter is attached to a node, the behavior is undefined.
        #[method_id(@__retain_semantics Other filters)]
        pub unsafe fn filters(&self) -> Option<Retained<NSArray<CIFilter>>>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        /// Setter for [`filters`][Self::filters].
        #[method(setFilters:)]
        pub unsafe fn setFilters(&self, filters: Option<&NSArray<CIFilter>>);

        /// Returns the presentation node.
        ///
        /// Returns a copy of the node containing all the properties as they were at the start of the current transaction, with any active animations applied.
        /// This gives a close approximation to the version of the node that is currently displayed.
        /// The effect of attempting to modify the returned node in any way is undefined. The returned node has no parent and no child nodes.
        #[method_id(@__retain_semantics Other presentationNode)]
        pub unsafe fn presentationNode(&self) -> Retained<SCNNode>;

        /// Controls whether or not the node's actions and animations are updated or paused. Defaults to NO.
        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        /// Setter for [`isPaused`][Self::isPaused].
        #[method(setPaused:)]
        pub unsafe fn setPaused(&self, paused: bool);

        /// Specifies the receiver's renderer delegate object.
        ///
        /// Setting a renderer delegate prevents the SceneKit renderer from drawing the node and lets you use custom OpenGL code instead.
        /// The preferred way to customize the rendering is to tweak the material properties of the different materials of the node's geometry. SCNMaterial conforms to the SCNShadable protocol and allows for more advanced rendering using GLSL.
        /// You would typically use a renderer delegate with a node that has no geometry and only serves as a location in space. An example would be attaching a particle system to that node and render it with custom OpenGL code.
        #[method_id(@__retain_semantics Other rendererDelegate)]
        pub unsafe fn rendererDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn SCNNodeRendererDelegate>>>;

        /// Setter for [`rendererDelegate`][Self::rendererDelegate].
        #[method(setRendererDelegate:)]
        pub unsafe fn setRendererDelegate(
            &self,
            renderer_delegate: Option<&ProtocolObject<dyn SCNNodeRendererDelegate>>,
        );

        #[cfg(all(
            feature = "SCNHitTest",
            feature = "SceneKitTypes",
            feature = "objc2-core-foundation"
        ))]
        /// Returns an array of SCNHitTestResult for each node in the receiver's sub tree that intersects the specified segment.
        ///
        /// Parameter `pointA`: The first point of the segment relative to the receiver.
        ///
        /// Parameter `pointB`: The second point of the segment relative to the receiver.
        ///
        /// Parameter `options`: Optional parameters (see the "Hit test options" section in SCNSceneRenderer.h for the available options).
        ///
        /// See SCNSceneRenderer.h for a screen-space hit testing method.
        #[method_id(@__retain_semantics Other hitTestWithSegmentFromPoint:toPoint:options:)]
        pub unsafe fn hitTestWithSegmentFromPoint_toPoint_options(
            &self,
            point_a: SCNVector3,
            point_b: SCNVector3,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<NSArray<SCNHitTestResult>>;

        /// Defines what logical 'categories' the receiver belongs too. Defaults to 1.
        ///
        /// Categories can be used to
        /// 1. exclude nodes from the influence of a given light (see SCNLight.categoryBitMask)
        /// 2. include/exclude nodes from render passes (see SCNTechnique.h)
        /// 3. specify which nodes to use when hit-testing (see SCNHitTestOptionCategoryBitMask)
        #[method(categoryBitMask)]
        pub unsafe fn categoryBitMask(&self) -> NSUInteger;

        /// Setter for [`categoryBitMask`][Self::categoryBitMask].
        #[method(setCategoryBitMask:)]
        pub unsafe fn setCategoryBitMask(&self, category_bit_mask: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCNNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// Transforms
    unsafe impl SCNNode {
        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// The local unit Y axis (0, 1, 0).
        #[method(localUp)]
        pub unsafe fn localUp() -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// The local unit X axis (1, 0, 0).
        #[method(localRight)]
        pub unsafe fn localRight() -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// The local unit -Z axis (0, 0, -1).
        #[method(localFront)]
        pub unsafe fn localFront() -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// The local unit Y axis (0, 1, 0) in world space.
        #[method(worldUp)]
        pub unsafe fn worldUp(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// The local unit X axis (1, 0, 0) in world space.
        #[method(worldRight)]
        pub unsafe fn worldRight(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// The local unit -Z axis (0, 0, -1) in world space.
        #[method(worldFront)]
        pub unsafe fn worldFront(&self) -> SCNVector3;

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Convenience for calling lookAt:up:localFront: with worldUp set to `self.worldUp`
        /// and localFront [SCNNode localFront].
        ///
        /// Parameter `worldTarget`: target position in world space.
        #[method(lookAt:)]
        pub unsafe fn lookAt(&self, world_target: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Set the orientation of the node so its front vector is pointing toward a given
        /// target. Using a reference up vector in world space and a front vector in
        /// local space.
        ///
        ///
        /// Parameter `worldTarget`: position in world space.
        ///
        /// Parameter `worldUp`: the up vector in world space.
        ///
        /// Parameter `localFront`: the front vector in local space.
        #[method(lookAt:up:localFront:)]
        pub unsafe fn lookAt_up_localFront(
            &self,
            world_target: SCNVector3,
            world_up: SCNVector3,
            local_front: SCNVector3,
        );

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Translate the current node position along the given vector in local space.
        ///
        ///
        /// Parameter `translation`: the translation in local space.
        #[method(localTranslateBy:)]
        pub unsafe fn localTranslateBy(&self, translation: SCNVector3);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Apply a the given rotation to the current one.
        ///
        ///
        /// Parameter `rotation`: rotation in local space.
        #[method(localRotateBy:)]
        pub unsafe fn localRotateBy(&self, rotation: SCNQuaternion);

        #[cfg(all(feature = "SceneKitTypes", feature = "objc2-core-foundation"))]
        /// Apply a rotation relative to a target point in parent space.
        ///
        ///
        /// Parameter `worldRotation`: rotation to apply in world space.
        ///
        /// Parameter `worldTarget`: position of the target in world space.
        #[method(rotateBy:aroundTarget:)]
        pub unsafe fn rotateBy_aroundTarget(
            &self,
            world_rotation: SCNQuaternion,
            world_target: SCNVector3,
        );
    }
);

extern_protocol!(
    /// The SCNNodeRendererDelegate protocol declares the methods that an instance of SCNNode invokes to let a delegate customize its rendering.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnnoderendererdelegate?language=objc)
    pub unsafe trait SCNNodeRendererDelegate: NSObjectProtocol {
        #[cfg(feature = "SCNRenderer")]
        /// Invoked when a node is rendered.
        ///
        /// The preferred way to customize the rendering is to tweak the material properties of the different materials of the node's geometry. SCNMaterial conforms to the SCNShadable protocol and allows for more advanced rendering using GLSL.
        /// You would typically use a renderer delegate with a node that has no geometry and only serves as a location in space. An example would be attaching a particle system to that node and render it with custom OpenGL code.
        /// Only drawing calls and the means to achieve them are supposed to be performed during the renderer delegate callback, any changes in the model (nodes, geometry...) would involve unexpected results.
        ///
        /// Parameter `node`: The node to render.
        ///
        /// Parameter `renderer`: The scene renderer to render into.
        ///
        /// Parameter `arguments`: A dictionary whose values are SCNMatrix4 matrices wrapped in NSValue objects.
        #[optional]
        #[method(renderNode:renderer:arguments:)]
        unsafe fn renderNode_renderer_arguments(
            &self,
            node: &SCNNode,
            renderer: &SCNRenderer,
            arguments: &NSDictionary<NSString, AnyObject>,
        );
    }
);

extern_methods!(
    /// SIMD
    unsafe impl SCNNode {}
);

extern_methods!(
    /// Focus
    unsafe impl SCNNode {
        /// Controls the behavior of the receiver regarding the UIFocus system. Defaults to SCNNodeFocusBehaviorNone.
        #[method(focusBehavior)]
        pub unsafe fn focusBehavior(&self) -> SCNNodeFocusBehavior;

        /// Setter for [`focusBehavior`][Self::focusBehavior].
        #[method(setFocusBehavior:)]
        pub unsafe fn setFocusBehavior(&self, focus_behavior: SCNNodeFocusBehavior);
    }
);