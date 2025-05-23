//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/careplicatorlayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CAReplicatorLayer;
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
extern_conformance!(
    unsafe impl CAMediaTiming for CAReplicatorLayer {}
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl NSCoding for CAReplicatorLayer {}
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CAReplicatorLayer {}
);

#[cfg(feature = "CALayer")]
extern_conformance!(
    unsafe impl NSSecureCoding for CAReplicatorLayer {}
);

#[cfg(feature = "CALayer")]
impl CAReplicatorLayer {
    extern_methods!(
        #[unsafe(method(instanceCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn instanceCount(&self) -> NSInteger;

        /// Setter for [`instanceCount`][Self::instanceCount].
        #[unsafe(method(setInstanceCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInstanceCount(&self, instance_count: NSInteger);

        #[unsafe(method(preservesDepth))]
        #[unsafe(method_family = none)]
        pub unsafe fn preservesDepth(&self) -> bool;

        /// Setter for [`preservesDepth`][Self::preservesDepth].
        #[unsafe(method(setPreservesDepth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPreservesDepth(&self, preserves_depth: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(instanceDelay))]
        #[unsafe(method_family = none)]
        pub unsafe fn instanceDelay(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`instanceDelay`][Self::instanceDelay].
        #[unsafe(method(setInstanceDelay:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInstanceDelay(&self, instance_delay: CFTimeInterval);

        #[cfg(all(feature = "CATransform3D", feature = "objc2-core-foundation"))]
        #[unsafe(method(instanceTransform))]
        #[unsafe(method_family = none)]
        pub unsafe fn instanceTransform(&self) -> CATransform3D;

        #[cfg(all(feature = "CATransform3D", feature = "objc2-core-foundation"))]
        /// Setter for [`instanceTransform`][Self::instanceTransform].
        #[unsafe(method(setInstanceTransform:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInstanceTransform(&self, instance_transform: CATransform3D);

        #[cfg(feature = "objc2-core-graphics")]
        #[unsafe(method(instanceColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn instanceColor(&self) -> Option<Retained<CGColor>>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`instanceColor`][Self::instanceColor].
        #[unsafe(method(setInstanceColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInstanceColor(&self, instance_color: Option<&CGColor>);

        #[unsafe(method(instanceRedOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn instanceRedOffset(&self) -> c_float;

        /// Setter for [`instanceRedOffset`][Self::instanceRedOffset].
        #[unsafe(method(setInstanceRedOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInstanceRedOffset(&self, instance_red_offset: c_float);

        #[unsafe(method(instanceGreenOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn instanceGreenOffset(&self) -> c_float;

        /// Setter for [`instanceGreenOffset`][Self::instanceGreenOffset].
        #[unsafe(method(setInstanceGreenOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInstanceGreenOffset(&self, instance_green_offset: c_float);

        #[unsafe(method(instanceBlueOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn instanceBlueOffset(&self) -> c_float;

        /// Setter for [`instanceBlueOffset`][Self::instanceBlueOffset].
        #[unsafe(method(setInstanceBlueOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInstanceBlueOffset(&self, instance_blue_offset: c_float);

        #[unsafe(method(instanceAlphaOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn instanceAlphaOffset(&self) -> c_float;

        /// Setter for [`instanceAlphaOffset`][Self::instanceAlphaOffset].
        #[unsafe(method(setInstanceAlphaOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInstanceAlphaOffset(&self, instance_alpha_offset: c_float);
    );
}

/// Methods declared on superclass `CALayer`.
#[cfg(feature = "CALayer")]
impl CAReplicatorLayer {
    extern_methods!(
        /// Layer creation and initialization. *
        #[unsafe(method(layer))]
        #[unsafe(method_family = none)]
        pub unsafe fn layer() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithLayer:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "CALayer")]
impl CAReplicatorLayer {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
