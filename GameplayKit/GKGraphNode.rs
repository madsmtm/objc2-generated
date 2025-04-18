//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A node in a directed graph. Edges are directed and can have variable costs.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkgraphnode?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKGraphNode;
);

extern_conformance!(
    unsafe impl NSCoding for GKGraphNode {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKGraphNode {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for GKGraphNode {}
);

impl GKGraphNode {
    extern_methods!(
        /// List of other graph nodes that this node has an edge leading to.
        #[unsafe(method(connectedNodes))]
        #[unsafe(method_family = none)]
        pub unsafe fn connectedNodes(&self) -> Retained<NSArray<GKGraphNode>>;

        /// Add a connection to a group of other nodes indicating those nodes can be reached from this node.
        /// A new connection is not created if it already exists.
        ///
        /// Parameter `nodes`: The array of nodes that are end points for their respective connections
        ///
        /// Parameter `bidirectional`: should a connection also be added connecting the destination node back to this node?
        #[unsafe(method(addConnectionsToNodes:bidirectional:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addConnectionsToNodes_bidirectional(
            &self,
            nodes: &NSArray<GKGraphNode>,
            bidirectional: bool,
        );

        /// Removes connections to a group of other nodes indicating those nodes can no longer be reached from this node.
        /// Nothing happens if a particular connection does not exist.
        ///
        /// Parameter `nodes`: The array of nodes that are end points of the edges to be removed
        ///
        /// Parameter `bidirectional`: should the connection also be added the destination node back to this node also be removed if it exists?
        #[unsafe(method(removeConnectionsToNodes:bidirectional:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeConnectionsToNodes_bidirectional(
            &self,
            nodes: &NSArray<GKGraphNode>,
            bidirectional: bool,
        );

        /// Returns the estimated heuristic cost to reach the indicated node from this node
        ///
        /// Parameter `node`: The end point of the edge who's cost is to be estimated
        #[unsafe(method(estimatedCostToNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn estimatedCostToNode(&self, node: &GKGraphNode) -> c_float;

        /// Returns the actual cost to reach the indicated node from this node
        #[unsafe(method(costToNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn costToNode(&self, node: &GKGraphNode) -> c_float;

        /// Attempts to find the optimal path between this node and the indicated goal node.
        /// If such a path exists, it is returned in start to end order.
        /// If it doesn't exist, the array returned will be empty.
        ///
        /// Parameter `goalNode`: the goal node of the pathfinding attempt
        #[unsafe(method(findPathToNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn findPathToNode(
            &self,
            goal_node: &GKGraphNode,
        ) -> Retained<NSArray<GKGraphNode>>;

        /// As with findPathToNode: except this node is the goal node and a startNode is specified
        ///
        /// Parameter `startNode`: the start node of the pathfinding attempt
        #[unsafe(method(findPathFromNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn findPathFromNode(
            &self,
            start_node: &GKGraphNode,
        ) -> Retained<NSArray<GKGraphNode>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl GKGraphNode {
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
    /// GKGraphNode coupled with a 2D position
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkgraphnode2d?language=objc)
    #[unsafe(super(GKGraphNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKGraphNode2D;
);

extern_conformance!(
    unsafe impl NSCoding for GKGraphNode2D {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKGraphNode2D {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for GKGraphNode2D {}
);

impl GKGraphNode2D {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl GKGraphNode2D {
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
    /// GKGraphNode coupled with a 3D position
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkgraphnode3d?language=objc)
    #[unsafe(super(GKGraphNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKGraphNode3D;
);

extern_conformance!(
    unsafe impl NSCoding for GKGraphNode3D {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKGraphNode3D {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for GKGraphNode3D {}
);

impl GKGraphNode3D {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl GKGraphNode3D {
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
    /// GKGraphNode coupled with a position on a 2D grid
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkgridgraphnode?language=objc)
    #[unsafe(super(GKGraphNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKGridGraphNode;
);

extern_conformance!(
    unsafe impl NSCoding for GKGridGraphNode {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKGridGraphNode {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for GKGridGraphNode {}
);

impl GKGridGraphNode {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl GKGridGraphNode {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
