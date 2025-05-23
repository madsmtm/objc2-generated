//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// The individual node(s) that make up a GKOctree.
    /// Used as a hint for faster removal via [GKOctree removeData:WithNode:]
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkoctreenode?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKOctreeNode;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GKOctreeNode {}
);

impl GKOctreeNode {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl GKOctreeNode {
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
    /// A tree data structure where each level has 8 children that subdivide a given space into the eight octants.
    /// Stores arbitrary NSObject elements via points and boxes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gameplaykit/gkoctree?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKOctree<ElementType: ?Sized = AnyObject>;
);

extern_conformance!(
    unsafe impl<ElementType: ?Sized> NSObjectProtocol for GKOctree<ElementType> {}
);

impl<ElementType: Message> GKOctree<ElementType> {
    extern_methods!(
        /// Removes the given NSObject from this octree
        /// Note that this is an exhaustive search and is can be slow for larger trees.
        /// Cache the relevant GKOctreeNode and use removeElement:WithNode: for better performance.
        ///
        ///
        /// Parameter `element`: the element to be removed
        ///
        /// Returns: returns YES if the data was removed, NO otherwise
        #[unsafe(method(removeElement:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeElement(&self, element: &ElementType) -> bool;

        /// Removes the given NSObject from the given node
        /// Note that this is not an exhaustive search and is faster than removeData:
        ///
        ///
        /// Parameter `element`: the element to be removed
        ///
        /// Parameter `node`: the node in which this data resides
        ///
        /// Returns: returns YES if the element was removed, NO otherwise
        #[unsafe(method(removeElement:withNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeElement_withNode(
            &self,
            element: &ElementType,
            node: &GKOctreeNode,
        ) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl<ElementType: Message> GKOctree<ElementType> {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
