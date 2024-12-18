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
unsafe impl NSObjectProtocol for UIFieldBehavior {}

extern_methods!(
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIFieldBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(addItem:)]
        pub unsafe fn addItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Retained<NSArray<ProtocolObject<dyn UIDynamicItem>>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(position)]
        pub unsafe fn position(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setPosition:)]
        pub unsafe fn setPosition(&self, position: CGPoint);

        #[cfg(feature = "UIRegion")]
        #[method_id(@__retain_semantics Other region)]
        pub unsafe fn region(&self) -> Retained<UIRegion>;

        #[cfg(feature = "UIRegion")]
        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: &UIRegion);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(strength)]
        pub unsafe fn strength(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setStrength:)]
        pub unsafe fn setStrength(&self, strength: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(falloff)]
        pub unsafe fn falloff(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setFalloff:)]
        pub unsafe fn setFalloff(&self, falloff: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumRadius)]
        pub unsafe fn minimumRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMinimumRadius:)]
        pub unsafe fn setMinimumRadius(&self, minimum_radius: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(direction)]
        pub unsafe fn direction(&self) -> CGVector;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setDirection:)]
        pub unsafe fn setDirection(&self, direction: CGVector);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(smoothness)]
        pub unsafe fn smoothness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setSmoothness:)]
        pub unsafe fn setSmoothness(&self, smoothness: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(animationSpeed)]
        pub unsafe fn animationSpeed(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAnimationSpeed:)]
        pub unsafe fn setAnimationSpeed(&self, animation_speed: CGFloat);

        #[method_id(@__retain_semantics Other dragField)]
        pub unsafe fn dragField(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other vortexField)]
        pub unsafe fn vortexField(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other radialGravityFieldWithPosition:)]
        pub unsafe fn radialGravityFieldWithPosition(
            position: CGPoint,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other linearGravityFieldWithVector:)]
        pub unsafe fn linearGravityFieldWithVector(
            direction: CGVector,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other velocityFieldWithVector:)]
        pub unsafe fn velocityFieldWithVector(
            direction: CGVector,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other noiseFieldWithSmoothness:animationSpeed:)]
        pub unsafe fn noiseFieldWithSmoothness_animationSpeed(
            smoothness: CGFloat,
            speed: CGFloat,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other turbulenceFieldWithSmoothness:animationSpeed:)]
        pub unsafe fn turbulenceFieldWithSmoothness_animationSpeed(
            smoothness: CGFloat,
            speed: CGFloat,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other springField)]
        pub unsafe fn springField(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other electricField)]
        pub unsafe fn electricField(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other magneticField)]
        pub unsafe fn magneticField(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Other fieldWithEvaluationBlock:)]
        pub unsafe fn fieldWithEvaluationBlock(
            block: &block2::Block<
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
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIFieldBehavior {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
