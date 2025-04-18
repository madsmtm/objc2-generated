//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// UIFocusDebugger provides a collection of runtime utilities for debugging issues related to focus interaction.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uifocusdebugger?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusDebugger;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UIFocusDebugger {}
);

impl UIFocusDebugger {
    extern_methods!(
        /// Outputs an overview of all supported debugging utilities and other relevant information.
        /// - To use in Swift, enter `po UIFocusDebugger.help()` when paused in lldb.
        /// - To use in Objective-C, enter `po [UIFocusDebugger help]` when paused in lldb.
        #[unsafe(method(help))]
        #[unsafe(method_family = none)]
        pub unsafe fn help(
            mtm: MainThreadMarker,
        ) -> Retained<ProtocolObject<dyn UIFocusDebuggerOutput>>;

        /// Outputs information for the currently focused item.
        /// - To use in Swift, enter `po UIFocusDebugger.status()` when paused in lldb.
        /// - To use in Objective-C, enter `po [UIFocusDebugger status]` when paused in lldb.
        #[unsafe(method(status))]
        #[unsafe(method_family = none)]
        pub unsafe fn status(
            mtm: MainThreadMarker,
        ) -> Retained<ProtocolObject<dyn UIFocusDebuggerOutput>>;

        #[cfg(feature = "UIFocus")]
        /// Outputs a diagnosis of the specified item's focusability, including any known issues that may be preventing focusability.
        /// - To use in Swift, enter `po UIFocusDebugger.checkFocusability(for:
        /// <item
        /// reference>)` when paused in lldb.
        /// - To use in Objective-C, enter `po [UIFocusDebugger checkFocusabilityForItem:
        /// <item
        /// reference>]` when paused in lldb.
        #[unsafe(method(checkFocusabilityForItem:))]
        #[unsafe(method_family = none)]
        pub unsafe fn checkFocusabilityForItem(
            item: &ProtocolObject<dyn UIFocusItem>,
        ) -> Retained<ProtocolObject<dyn UIFocusDebuggerOutput>>;

        #[cfg(feature = "UIFocus")]
        /// Simulates a fake focus update requested by the specified environment (e.g. `[focusSystem requestFocusUpdateToEnvironment:environment]`), outlining each step of the process for determining the next focused item.
        /// - To use in Swift, enter `po UIFocusDebugger.simulateFocusUpdateRequest(from:
        /// <environment
        /// reference>)` when paused in lldb.
        /// - To use in Objective-C, enter `po [UIFocusDebugger simulateFocusUpdateRequestFromEnvironment:
        /// <environment
        /// reference>]` when paused in lldb.
        #[unsafe(method(simulateFocusUpdateRequestFromEnvironment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn simulateFocusUpdateRequestFromEnvironment(
            environment: &ProtocolObject<dyn UIFocusEnvironment>,
        ) -> Retained<ProtocolObject<dyn UIFocusDebuggerOutput>>;

        #[cfg(feature = "UIFocus")]
        /// Outputs a diagnosis of the focus groups of the specified environment and its children.
        /// Pass a focus system as the environment to get the full focus group tree for this focus system.
        /// - To use in Swift, enter `po UIFocusDebugger.focusGroups(for:
        /// <environment
        /// reference>)` when paused in lldb.
        /// - To use in Objective-C, enter `po [UIFocusDebugger focusGroupsForEnvironment:
        /// <environment
        /// reference>]` when paused in lldb.
        #[unsafe(method(focusGroupsForEnvironment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn focusGroupsForEnvironment(
            environment: &ProtocolObject<dyn UIFocusEnvironment>,
        ) -> Retained<ProtocolObject<dyn UIFocusDebuggerOutput>>;

        #[cfg(feature = "UIFocus")]
        /// Outputs a diagnoses of the preferred focus environments tree.
        /// - To use in Swift, enter `po UIFocusDebugger.preferredFocusEnvironments(for:
        /// <environment
        /// reference>)` when paused in lldb.
        /// - To use in Objective-C, enter `po [UIFocusDebugger preferredFocusEnvironmentsForEnvironment:
        /// <environment
        /// reference>]` when paused in lldb.
        #[unsafe(method(preferredFocusEnvironmentsForEnvironment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn preferredFocusEnvironmentsForEnvironment(
            environment: &ProtocolObject<dyn UIFocusEnvironment>,
        ) -> Retained<ProtocolObject<dyn UIFocusDebuggerOutput>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl UIFocusDebugger {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifocusdebuggeroutput?language=objc)
    pub unsafe trait UIFocusDebuggerOutput: NSObjectProtocol + MainThreadOnly {}
);
