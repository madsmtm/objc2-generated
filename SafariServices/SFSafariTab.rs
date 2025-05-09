//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/safariservices/sfsafaritab?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSafariTab;
);

extern_conformance!(
    unsafe impl NSCoding for SFSafariTab {}
);

extern_conformance!(
    unsafe impl NSCopying for SFSafariTab {}
);

unsafe impl CopyingHelper for SFSafariTab {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for SFSafariTab {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for SFSafariTab {}
);

impl SFSafariTab {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "SFSafariPage", feature = "block2"))]
        /// This calls the completion handler passing the active page in the tab.
        #[unsafe(method(getActivePageWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getActivePageWithCompletionHandler(
            &self,
            completion_handler: &block2::DynBlock<dyn Fn(*mut SFSafariPage)>,
        );

        #[cfg(all(feature = "SFSafariPage", feature = "block2"))]
        /// This calls the completion handler passing all the pages in the tab. This includes the active page and any pages being preloaded by Safari.
        #[unsafe(method(getPagesWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getPagesWithCompletionHandler(
            &self,
            completion_handler: &block2::DynBlock<dyn Fn(*mut NSArray<SFSafariPage>)>,
        );

        #[cfg(all(feature = "SFSafariWindow", feature = "block2"))]
        /// This calls completion handler with the window containing this tab. If the tab is pinned, the window is nil.
        #[unsafe(method(getContainingWindowWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getContainingWindowWithCompletionHandler(
            &self,
            completion_handler: &block2::DynBlock<dyn Fn(*mut SFSafariWindow)>,
        );

        #[cfg(feature = "block2")]
        /// Activates this tab in the window it belongs to.
        #[unsafe(method(activateWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn activateWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::DynBlock<dyn Fn()>>,
        );

        /// Navigates this tab to the given URL. The extension doesn't need permission to access the URL to navigate to it.
        #[unsafe(method(navigateToURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn navigateToURL(&self, url: &NSURL);

        /// Closes this tab. If this is the last tab in its window, the window is also closed.
        #[unsafe(method(close))]
        #[unsafe(method_family = none)]
        pub unsafe fn close(&self);
    );
}
