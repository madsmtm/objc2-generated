//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/spritekit/skphysicscontact?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKPhysicsContact;
);

unsafe impl NSObjectProtocol for SKPhysicsContact {}

extern_methods!(
    unsafe impl SKPhysicsContact {
        #[cfg(feature = "SKPhysicsBody")]
        #[method_id(@__retain_semantics Other bodyA)]
        pub unsafe fn bodyA(&self) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "SKPhysicsBody")]
        #[method_id(@__retain_semantics Other bodyB)]
        pub unsafe fn bodyB(&self) -> Retained<SKPhysicsBody>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contactPoint)]
        pub unsafe fn contactPoint(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contactNormal)]
        pub unsafe fn contactNormal(&self) -> CGVector;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(collisionImpulse)]
        pub unsafe fn collisionImpulse(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKPhysicsContact {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);