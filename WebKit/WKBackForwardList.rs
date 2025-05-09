//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A WKBackForwardList object is a list of webpages previously
    /// visited in a web view that can be reached by going back or forward.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wkbackforwardlist?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKBackForwardList;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKBackForwardList {}
);

impl WKBackForwardList {
    extern_methods!(
        #[cfg(feature = "WKBackForwardListItem")]
        /// The current item.
        #[unsafe(method(currentItem))]
        #[unsafe(method_family = none)]
        pub unsafe fn currentItem(&self) -> Option<Retained<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        /// The item immediately preceding the current item, or nil
        /// if there isn't one.
        #[unsafe(method(backItem))]
        #[unsafe(method_family = none)]
        pub unsafe fn backItem(&self) -> Option<Retained<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        /// The item immediately following the current item, or nil
        /// if there isn't one.
        #[unsafe(method(forwardItem))]
        #[unsafe(method_family = none)]
        pub unsafe fn forwardItem(&self) -> Option<Retained<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        /// Returns the item at a specified distance from the current
        /// item.
        ///
        /// Parameter `index`: Index of the desired list item relative to the current item:
        /// 0 for the current item, -1 for the immediately preceding item, 1 for the
        /// immediately following item, and so on.
        ///
        /// Returns: The item at the specified distance from the current item, or nil
        /// if the index parameter exceeds the limits of the list.
        #[unsafe(method(itemAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn itemAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Retained<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        /// The portion of the list preceding the current item.
        ///
        /// The items are in the order in which they were originally
        /// visited.
        #[unsafe(method(backList))]
        #[unsafe(method_family = none)]
        pub unsafe fn backList(&self) -> Retained<NSArray<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        /// The portion of the list following the current item.
        ///
        /// The items are in the order in which they were originally
        /// visited.
        #[unsafe(method(forwardList))]
        #[unsafe(method_family = none)]
        pub unsafe fn forwardList(&self) -> Retained<NSArray<WKBackForwardListItem>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl WKBackForwardList {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
