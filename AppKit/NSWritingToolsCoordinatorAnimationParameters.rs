//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// An object you use to configure additional tasks or animations to
    /// run alongside the Writing Tools animations.
    ///
    /// When Writing Tools replaces text in one of your context objects,
    /// it provides an `NSWritingToolsCoordinator.AnimationParameters` object for
    /// you to use to configure any additional animations. During a Writing
    /// Tools session, you hide the text under evaluation and provide a
    /// targeted preview of your content. Writing Tools animations changes
    /// to that preview, but you might need to provide additional animations
    /// for other parts of your view’s content. For example, you might
    /// need to animate any layout changes caused by the insertion or
    /// removal of text in other parts of your view. Use this object to
    /// configure those animations.
    ///
    /// You don’t create an `NSWritingToolsCoordinator.AnimationParameters`
    /// object directly. Instead, the system creates one and passes it to the
    /// ``NSWritingToolsCoordinator/writingToolsCoordinator(_:replaceRange:inContext:proposedText:reason:animationParameters:completion:)``
    /// method of your ``NSWritingToolsCoordinator/Delegate`` object. Use that
    /// object to specify the blocks to run during and after the system animations.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nswritingtoolscoordinatoranimationparameters?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWritingToolsCoordinatorAnimationParameters;
);

unsafe impl Send for NSWritingToolsCoordinatorAnimationParameters {}

unsafe impl Sync for NSWritingToolsCoordinatorAnimationParameters {}

unsafe impl NSObjectProtocol for NSWritingToolsCoordinatorAnimationParameters {}

extern_methods!(
    unsafe impl NSWritingToolsCoordinatorAnimationParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// The number of seconds it takes the system animations to run.
        #[method(duration)]
        pub unsafe fn duration(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// The number of seconds the system waits before starting its animations.
        #[method(delay)]
        pub unsafe fn delay(&self) -> CGFloat;

        #[cfg(feature = "block2")]
        /// A custom block that runs at the same time as the system animations.
        ///
        /// If you have animations you want to run at the same time as the system
        /// animations, assign a block to this property and use it to run your
        /// animations. The block you provide must have no return value and take
        /// a floating-point value as a parameter. The parameter indicates the
        /// current progress of the animations as a percentage value between
        /// `0.0` to `1.0`. The system executes your block multiple times during
        /// the course of the animations, providing an updated completion value each time.
        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> *mut block2::Block<dyn Fn(c_float)>;

        #[cfg(feature = "block2")]
        /// Setter for [`progressHandler`][Self::progressHandler].
        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(
            &self,
            progress_handler: Option<&block2::Block<dyn Fn(c_float)>>,
        );

        #[cfg(feature = "block2")]
        /// A custom block to run when the system animations finish.
        ///
        /// Set this property to a block that you want the system to run when any
        /// animations finish. The block you provide must have no return value
        /// and no parameters. The system executes this block once when the current
        /// animation finish.
        #[method(completionHandler)]
        pub unsafe fn completionHandler(&self) -> *mut block2::Block<dyn Fn()>;

        #[cfg(feature = "block2")]
        /// Setter for [`completionHandler`][Self::completionHandler].
        #[method(setCompletionHandler:)]
        pub unsafe fn setCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSWritingToolsCoordinatorAnimationParameters {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);