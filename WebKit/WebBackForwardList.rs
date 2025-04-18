//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// WebBackForwardList holds an ordered list of WebHistoryItems that comprises the back and
    /// forward lists.
    ///
    /// Note that the methods which modify instances of this class do not cause
    /// navigation to happen in other layers of the stack;  they are only for maintaining this data
    /// structure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/webbackforwardlist?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct WebBackForwardList;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WebBackForwardList {}
);

impl WebBackForwardList {
    extern_methods!(
        #[cfg(feature = "WebHistoryItem")]
        /// Adds an entry to the list.
        ///
        /// Parameter `item`: The entry to add.
        ///
        /// The added entry is inserted immediately after the current entry.
        /// If the current position in the list is not at the end of the list, elements in the
        /// forward list will be dropped at this point.  In addition, entries may be dropped to keep
        /// the size of the list within the maximum size.
        #[deprecated]
        #[unsafe(method(addItem:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addItem(&self, item: Option<&WebHistoryItem>);

        /// Move the current pointer back to the entry before the current entry.
        #[deprecated]
        #[unsafe(method(goBack))]
        #[unsafe(method_family = none)]
        pub unsafe fn goBack(&self);

        /// Move the current pointer ahead to the entry after the current entry.
        #[deprecated]
        #[unsafe(method(goForward))]
        #[unsafe(method_family = none)]
        pub unsafe fn goForward(&self);

        #[cfg(feature = "WebHistoryItem")]
        /// Move the current pointer to the given entry.
        ///
        /// Parameter `item`: The history item to move the pointer to
        #[deprecated]
        #[unsafe(method(goToItem:))]
        #[unsafe(method_family = none)]
        pub unsafe fn goToItem(&self, item: Option<&WebHistoryItem>);

        #[cfg(feature = "WebHistoryItem")]
        /// The entry right before the current entry, or nil if there isn't one.
        #[deprecated]
        #[unsafe(method(backItem))]
        #[unsafe(method_family = none)]
        pub unsafe fn backItem(&self) -> Option<Retained<WebHistoryItem>>;

        #[cfg(feature = "WebHistoryItem")]
        /// Returns the current entry.
        #[deprecated]
        #[unsafe(method(currentItem))]
        #[unsafe(method_family = none)]
        pub unsafe fn currentItem(&self) -> Option<Retained<WebHistoryItem>>;

        #[cfg(feature = "WebHistoryItem")]
        /// The entry right after the current entry, or nil if there isn't one.
        #[deprecated]
        #[unsafe(method(forwardItem))]
        #[unsafe(method_family = none)]
        pub unsafe fn forwardItem(&self) -> Option<Retained<WebHistoryItem>>;

        /// Returns a portion of the list before the current entry.
        ///
        /// Parameter `limit`: A cap on the size of the array returned.
        ///
        /// Returns: An array of items before the current entry, or nil if there are none.  The entries are in the order that they were originally visited.
        #[deprecated]
        #[unsafe(method(backListWithLimit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn backListWithLimit(&self, limit: c_int) -> Option<Retained<NSArray>>;

        /// Returns a portion of the list after the current entry.
        ///
        /// Parameter `limit`: A cap on the size of the array returned.
        ///
        /// Returns: An array of items after the current entry, or nil if there are none.  The entries are in the order that they were originally visited.
        #[deprecated]
        #[unsafe(method(forwardListWithLimit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn forwardListWithLimit(&self, limit: c_int) -> Option<Retained<NSArray>>;

        /// The list's maximum size.
        #[deprecated]
        #[unsafe(method(capacity))]
        #[unsafe(method_family = none)]
        pub unsafe fn capacity(&self) -> c_int;

        /// Setter for [`capacity`][Self::capacity].
        #[deprecated]
        #[unsafe(method(setCapacity:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCapacity(&self, capacity: c_int);

        /// The number of items in the list.
        #[deprecated]
        #[unsafe(method(backListCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn backListCount(&self) -> c_int;

        /// Returns: The number of items in the list.
        #[deprecated]
        #[unsafe(method(forwardListCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn forwardListCount(&self) -> c_int;

        #[cfg(feature = "WebHistoryItem")]
        /// Parameter `item`: The item that will be checked for presence in the WebBackForwardList.
        ///
        /// Returns: Returns YES if the item is in the list.
        #[deprecated]
        #[unsafe(method(containsItem:))]
        #[unsafe(method_family = none)]
        pub unsafe fn containsItem(&self, item: Option<&WebHistoryItem>) -> bool;

        #[cfg(feature = "WebHistoryItem")]
        /// Returns an entry the given distance from the current entry.
        ///
        /// Parameter `index`: Index of the desired list item relative to the current item; 0 is current item, -1 is back item, 1 is forward item, etc.
        ///
        /// Returns: The entry the given distance from the current entry. If index exceeds the limits of the list, nil is returned.
        #[deprecated]
        #[unsafe(method(itemAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn itemAtIndex(&self, index: c_int) -> Option<Retained<WebHistoryItem>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl WebBackForwardList {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// WebBackForwardListDeprecated.
impl WebBackForwardList {
    extern_methods!(
        /// The size passed to this method determines whether the WebView
        /// associated with this WebBackForwardList will use the shared page cache.
        ///
        /// Parameter `size`: If size is 0, the WebView associated with this WebBackForwardList
        /// will not use the shared page cache. Otherwise, it will.
        #[deprecated]
        #[unsafe(method(setPageCacheSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPageCacheSize(&self, size: NSUInteger);

        /// Returns the size of the shared page cache, or 0.
        ///
        /// Returns: The size of the shared page cache (in pages), or 0 if the WebView
        /// associated with this WebBackForwardList will not use the shared page cache.
        #[deprecated]
        #[unsafe(method(pageCacheSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn pageCacheSize(&self) -> NSUInteger;
    );
}
