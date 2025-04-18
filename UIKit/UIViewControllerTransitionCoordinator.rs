//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitransitioncontextviewcontrollerkey?language=objc)
// NS_TYPED_ENUM
pub type UITransitionContextViewControllerKey = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitransitioncontextviewkey?language=objc)
// NS_TYPED_ENUM
pub type UITransitionContextViewKey = NSString;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollertransitioncoordinatorcontext?language=objc)
    pub unsafe trait UIViewControllerTransitionCoordinatorContext:
        NSObjectProtocol + MainThreadOnly
    {
        #[unsafe(method(isAnimated))]
        #[unsafe(method_family = none)]
        unsafe fn isAnimated(&self) -> bool;

        #[cfg(feature = "UIViewController")]
        #[unsafe(method(presentationStyle))]
        #[unsafe(method_family = none)]
        unsafe fn presentationStyle(&self) -> UIModalPresentationStyle;

        /// initiallyInteractive indicates whether the transition was initiated as an interactive transition.
        /// It never changes during the course of a transition.
        /// It can only be YES if isAnimated is YES.
        /// If it is NO, then isInteractive can only be YES if isInterruptible is YES
        #[unsafe(method(initiallyInteractive))]
        #[unsafe(method_family = none)]
        unsafe fn initiallyInteractive(&self) -> bool;

        #[unsafe(method(isInterruptible))]
        #[unsafe(method_family = none)]
        unsafe fn isInterruptible(&self) -> bool;

        #[unsafe(method(isInteractive))]
        #[unsafe(method_family = none)]
        unsafe fn isInteractive(&self) -> bool;

        #[unsafe(method(isCancelled))]
        #[unsafe(method_family = none)]
        unsafe fn isCancelled(&self) -> bool;

        #[unsafe(method(transitionDuration))]
        #[unsafe(method_family = none)]
        unsafe fn transitionDuration(&self) -> NSTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(percentComplete))]
        #[unsafe(method_family = none)]
        unsafe fn percentComplete(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(completionVelocity))]
        #[unsafe(method_family = none)]
        unsafe fn completionVelocity(&self) -> CGFloat;

        #[cfg(feature = "UIView")]
        #[unsafe(method(completionCurve))]
        #[unsafe(method_family = none)]
        unsafe fn completionCurve(&self) -> UIViewAnimationCurve;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[unsafe(method(viewControllerForKey:))]
        #[unsafe(method_family = none)]
        unsafe fn viewControllerForKey(
            &self,
            key: &UITransitionContextViewControllerKey,
        ) -> Option<Retained<UIViewController>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[unsafe(method(viewForKey:))]
        #[unsafe(method_family = none)]
        unsafe fn viewForKey(&self, key: &UITransitionContextViewKey) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[unsafe(method(containerView))]
        #[unsafe(method_family = none)]
        unsafe fn containerView(&self) -> Retained<UIView>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(targetTransform))]
        #[unsafe(method_family = none)]
        unsafe fn targetTransform(&self) -> CGAffineTransform;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollertransitioncoordinator?language=objc)
    pub unsafe trait UIViewControllerTransitionCoordinator:
        UIViewControllerTransitionCoordinatorContext + MainThreadOnly
    {
        #[cfg(feature = "block2")]
        #[unsafe(method(animateAlongsideTransition:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn animateAlongsideTransition_completion(
            &self,
            animation: Option<
                &block2::DynBlock<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>,
                    ),
                >,
            >,
            completion: Option<
                &block2::DynBlock<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>,
                    ),
                >,
            >,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "block2"))]
        #[unsafe(method(animateAlongsideTransitionInView:animation:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn animateAlongsideTransitionInView_animation_completion(
            &self,
            view: Option<&UIView>,
            animation: Option<
                &block2::DynBlock<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>,
                    ),
                >,
            >,
            completion: Option<
                &block2::DynBlock<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>,
                    ),
                >,
            >,
        ) -> bool;

        #[cfg(feature = "block2")]
        #[deprecated]
        #[unsafe(method(notifyWhenInteractionEndsUsingBlock:))]
        #[unsafe(method_family = none)]
        unsafe fn notifyWhenInteractionEndsUsingBlock(
            &self,
            handler: &block2::DynBlock<
                dyn Fn(NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>),
            >,
        );

        #[cfg(feature = "block2")]
        #[unsafe(method(notifyWhenInteractionChangesUsingBlock:))]
        #[unsafe(method_family = none)]
        unsafe fn notifyWhenInteractionChangesUsingBlock(
            &self,
            handler: &block2::DynBlock<
                dyn Fn(NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>),
            >,
        );
    }
);

/// UIViewControllerTransitionCoordinator.
#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
impl UIViewController {
    extern_methods!(
        #[unsafe(method(transitionCoordinator))]
        #[unsafe(method_family = none)]
        pub unsafe fn transitionCoordinator(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerTransitionCoordinator>>>;
    );
}
