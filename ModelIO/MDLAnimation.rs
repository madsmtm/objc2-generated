//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlskeleton?language=objc)
    #[unsafe(super(MDLObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MDLObject")]
    pub struct MDLSkeleton;
);

#[cfg(all(feature = "MDLObject", feature = "MDLTypes"))]
extern_conformance!(
    unsafe impl MDLNamed for MDLSkeleton {}
);

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl NSCopying for MDLSkeleton {}
);

#[cfg(feature = "MDLObject")]
unsafe impl CopyingHelper for MDLSkeleton {
    type Result = Self;
}

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MDLSkeleton {}
);

#[cfg(feature = "MDLObject")]
impl MDLSkeleton {
    extern_methods!(
        #[unsafe(method(jointPaths))]
        #[unsafe(method_family = none)]
        pub unsafe fn jointPaths(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "MDLValueTypes")]
        #[unsafe(method(jointBindTransforms))]
        #[unsafe(method_family = none)]
        pub unsafe fn jointBindTransforms(&self) -> Retained<MDLMatrix4x4Array>;

        #[cfg(feature = "MDLValueTypes")]
        #[unsafe(method(jointRestTransforms))]
        #[unsafe(method_family = none)]
        pub unsafe fn jointRestTransforms(&self) -> Retained<MDLMatrix4x4Array>;

        #[unsafe(method(initWithName:jointPaths:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName_jointPaths(
            this: Allocated<Self>,
            name: &NSString,
            joint_paths: &NSArray<NSString>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MDLObject")]
impl MDLSkeleton {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdljointanimation?language=objc)
    pub unsafe trait MDLJointAnimation {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlpackedjointanimation?language=objc)
    #[unsafe(super(MDLObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MDLObject")]
    pub struct MDLPackedJointAnimation;
);

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl MDLJointAnimation for MDLPackedJointAnimation {}
);

#[cfg(all(feature = "MDLObject", feature = "MDLTypes"))]
extern_conformance!(
    unsafe impl MDLNamed for MDLPackedJointAnimation {}
);

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl NSCopying for MDLPackedJointAnimation {}
);

#[cfg(feature = "MDLObject")]
unsafe impl CopyingHelper for MDLPackedJointAnimation {
    type Result = Self;
}

#[cfg(feature = "MDLObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MDLPackedJointAnimation {}
);

#[cfg(feature = "MDLObject")]
impl MDLPackedJointAnimation {
    extern_methods!(
        #[unsafe(method(jointPaths))]
        #[unsafe(method_family = none)]
        pub unsafe fn jointPaths(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "MDLAnimatedValueTypes")]
        #[unsafe(method(translations))]
        #[unsafe(method_family = none)]
        pub unsafe fn translations(&self) -> Retained<MDLAnimatedVector3Array>;

        #[cfg(feature = "MDLAnimatedValueTypes")]
        #[unsafe(method(rotations))]
        #[unsafe(method_family = none)]
        pub unsafe fn rotations(&self) -> Retained<MDLAnimatedQuaternionArray>;

        #[cfg(feature = "MDLAnimatedValueTypes")]
        #[unsafe(method(scales))]
        #[unsafe(method_family = none)]
        pub unsafe fn scales(&self) -> Retained<MDLAnimatedVector3Array>;

        #[unsafe(method(initWithName:jointPaths:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName_jointPaths(
            this: Allocated<Self>,
            name: &NSString,
            joint_paths: &NSArray<NSString>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MDLObject")]
impl MDLPackedJointAnimation {
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
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlanimationbindcomponent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLAnimationBindComponent;
);

#[cfg(feature = "MDLTypes")]
extern_conformance!(
    unsafe impl MDLComponent for MDLAnimationBindComponent {}
);

extern_conformance!(
    unsafe impl NSCopying for MDLAnimationBindComponent {}
);

unsafe impl CopyingHelper for MDLAnimationBindComponent {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MDLAnimationBindComponent {}
);

impl MDLAnimationBindComponent {
    extern_methods!(
        #[cfg(feature = "MDLObject")]
        #[unsafe(method(skeleton))]
        #[unsafe(method_family = none)]
        pub unsafe fn skeleton(&self) -> Option<Retained<MDLSkeleton>>;

        #[cfg(feature = "MDLObject")]
        /// Setter for [`skeleton`][Self::skeleton].
        #[unsafe(method(setSkeleton:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSkeleton(&self, skeleton: Option<&MDLSkeleton>);

        #[unsafe(method(jointAnimation))]
        #[unsafe(method_family = none)]
        pub unsafe fn jointAnimation(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MDLJointAnimation>>>;

        /// Setter for [`jointAnimation`][Self::jointAnimation].
        #[unsafe(method(setJointAnimation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setJointAnimation(
            &self,
            joint_animation: Option<&ProtocolObject<dyn MDLJointAnimation>>,
        );

        #[unsafe(method(jointPaths))]
        #[unsafe(method_family = none)]
        pub unsafe fn jointPaths(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`jointPaths`][Self::jointPaths].
        #[unsafe(method(setJointPaths:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setJointPaths(&self, joint_paths: Option<&NSArray<NSString>>);
    );
}

/// Methods declared on superclass `NSObject`.
impl MDLAnimationBindComponent {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
