//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// Allows to formally participate in UI updates and influence UI update behavior.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uiupdatelink?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIUpdateLink;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIUpdateLink {}
);

impl UIUpdateLink {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        #[unsafe(method(updateLinkForWindowScene:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateLinkForWindowScene(
            window_scene: &UIWindowScene,
        ) -> Retained<UIUpdateLink>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[unsafe(method(updateLinkForView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateLinkForView(view: &UIView) -> Retained<UIUpdateLink>;

        #[cfg(all(
            feature = "UIUpdateActionPhase",
            feature = "UIUpdateInfo",
            feature = "block2"
        ))]
        #[unsafe(method(addActionToPhase:handler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addActionToPhase_handler(
            &self,
            phase: &UIUpdateActionPhase,
            handler: &block2::DynBlock<dyn Fn(NonNull<UIUpdateLink>, NonNull<UIUpdateInfo>)>,
        );

        #[cfg(feature = "UIUpdateActionPhase")]
        #[unsafe(method(addActionToPhase:target:selector:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addActionToPhase_target_selector(
            &self,
            phase: &UIUpdateActionPhase,
            target: &AnyObject,
            selector: Sel,
        );

        /// It's required to enable the Update Link for it to have effect and for its actions to be invoked.
        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        /// By default, `UIUpdateLink` is a passive UI update observer. Its actions will only be called when UI update is being
        /// produced. When this property is set to `YES`, `UIUpdateLink` will request continuous UI updates by itself.
        #[unsafe(method(requiresContinuousUpdates))]
        #[unsafe(method_family = none)]
        pub unsafe fn requiresContinuousUpdates(&self) -> bool;

        /// Setter for [`requiresContinuousUpdates`][Self::requiresContinuousUpdates].
        #[unsafe(method(setRequiresContinuousUpdates:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRequiresContinuousUpdates(&self, requires_continuous_updates: bool);

        /// Request dispatch of low-latency eligible events in `LowLatencyEventDispatch` phase. Low latency eligible events are
        /// dispatch in the middle of the UI update, meaning that to handle them application has half the time, compared to
        /// events dispatched normally. Consult `-[UIUpdateInfo completionDeadlineTime]` for exact completion deadline time.
        #[unsafe(method(wantsLowLatencyEventDispatch))]
        #[unsafe(method_family = none)]
        pub unsafe fn wantsLowLatencyEventDispatch(&self) -> bool;

        /// Setter for [`wantsLowLatencyEventDispatch`][Self::wantsLowLatencyEventDispatch].
        #[unsafe(method(setWantsLowLatencyEventDispatch:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWantsLowLatencyEventDispatch(
            &self,
            wants_low_latency_event_dispatch: bool,
        );

        /// Request immediate frame presentation. When enabled, system will request immediate rendering of the display frame
        /// after last `CATransaction` commit for the current UI update. This allows to reduce input to display latency, as
        /// rendered display frame will be presented one frame duration sooner. However, for this to happen amount of work
        /// submitted to render server should be minimal, otherwise it will not be able to submit frame for presentation in
        /// time. This capability is primarily useful for pencil drawing applications where low input to display latency is
        /// critical for good user experience. Applications that request immediate presentation must be profiled thoroughly to
        /// ensure that amount of application and render server work is adequate. When application requests immediate
        /// presentation, but fails to keep work complexity at minimum, user will experience on screen judder, as frames will
        /// not be presented at their intended time.
        #[unsafe(method(wantsImmediatePresentation))]
        #[unsafe(method_family = none)]
        pub unsafe fn wantsImmediatePresentation(&self) -> bool;

        /// Setter for [`wantsImmediatePresentation`][Self::wantsImmediatePresentation].
        #[unsafe(method(setWantsImmediatePresentation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWantsImmediatePresentation(&self, wants_immediate_presentation: bool);

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        /// Preferred frame rate range. Even when not forcing periodic updates, this will still express intention to the system.
        /// Use `CAFrameRateRangeDefault` (default value) to not request any specific frame rate range.
        #[unsafe(method(preferredFrameRateRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange;

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        /// Setter for [`preferredFrameRateRange`][Self::preferredFrameRateRange].
        #[unsafe(method(setPreferredFrameRateRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPreferredFrameRateRange(
            &self,
            preferred_frame_rate_range: CAFrameRateRange,
        );

        #[cfg(feature = "UIUpdateInfo")]
        /// During UI update, returns `UIUpdateInfo` instance describing current UI update state. Returns `nil` outside of UI
        /// update.
        #[unsafe(method(currentUpdateInfo))]
        #[unsafe(method_family = none)]
        pub unsafe fn currentUpdateInfo(&self) -> Option<Retained<UIUpdateInfo>>;
    );
}

/// Convenience.
impl UIUpdateLink {
    extern_methods!(
        #[cfg(all(feature = "UIUpdateInfo", feature = "block2"))]
        /// Adds action to `UIUpdateActionPhase.beforeCADisplayLinkDispatch` phase.
        #[unsafe(method(addActionWithHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addActionWithHandler(
            &self,
            handler: &block2::DynBlock<dyn Fn(NonNull<UIUpdateLink>, NonNull<UIUpdateInfo>)>,
        );

        /// Adds action to `UIUpdateActionPhase.beforeCADisplayLinkDispatch` phase.
        #[unsafe(method(addActionWithTarget:selector:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addActionWithTarget_selector(&self, target: &AnyObject, selector: Sel);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIUpdateInfo",
            feature = "UIWindowScene",
            feature = "block2"
        ))]
        /// Adds action to `UIUpdateActionPhase.beforeCADisplayLinkDispatch` phase.
        #[unsafe(method(updateLinkForWindowScene:actionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateLinkForWindowScene_actionHandler(
            window_scene: &UIWindowScene,
            handler: &block2::DynBlock<dyn Fn(NonNull<UIUpdateLink>, NonNull<UIUpdateInfo>)>,
        ) -> Retained<UIUpdateLink>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        /// Adds action to `UIUpdateActionPhase.beforeCADisplayLinkDispatch` phase.
        #[unsafe(method(updateLinkForWindowScene:actionTarget:selector:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateLinkForWindowScene_actionTarget_selector(
            window_scene: &UIWindowScene,
            target: &AnyObject,
            selector: Sel,
        ) -> Retained<UIUpdateLink>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIUpdateInfo",
            feature = "UIView",
            feature = "block2"
        ))]
        /// Adds action to `UIUpdateActionPhase.beforeCADisplayLinkDispatch` phase.
        #[unsafe(method(updateLinkForView:actionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateLinkForView_actionHandler(
            view: &UIView,
            handler: &block2::DynBlock<dyn Fn(NonNull<UIUpdateLink>, NonNull<UIUpdateInfo>)>,
        ) -> Retained<UIUpdateLink>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// Adds action to `UIUpdateActionPhase.beforeCADisplayLinkDispatch` phase.
        #[unsafe(method(updateLinkForView:actionTarget:selector:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateLinkForView_actionTarget_selector(
            view: &UIView,
            target: &AnyObject,
            selector: Sel,
        ) -> Retained<UIUpdateLink>;
    );
}
