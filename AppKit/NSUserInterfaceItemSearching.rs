//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuserinterfaceitemsearching?language=objc)
    pub unsafe trait NSUserInterfaceItemSearching: NSObjectProtocol {
        #[cfg(feature = "block2")]
        #[unsafe(method(searchForItemsWithSearchString:resultLimit:matchedItemHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn searchForItemsWithSearchString_resultLimit_matchedItemHandler(
            &self,
            search_string: &NSString,
            result_limit: NSInteger,
            handle_matched_items: &block2::DynBlock<dyn Fn(NonNull<NSArray>)>,
        );

        #[unsafe(method(localizedTitlesForItem:))]
        #[unsafe(method_family = none)]
        unsafe fn localizedTitlesForItem(&self, item: &AnyObject) -> Retained<NSArray<NSString>>;

        #[optional]
        #[unsafe(method(performActionForItem:))]
        #[unsafe(method_family = none)]
        unsafe fn performActionForItem(&self, item: &AnyObject);

        #[optional]
        #[unsafe(method(showAllHelpTopicsForSearchString:))]
        #[unsafe(method_family = none)]
        unsafe fn showAllHelpTopicsForSearchString(&self, search_string: &NSString);
    }
);

/// NSUserInterfaceItemSearching.
#[cfg(all(feature = "NSApplication", feature = "NSResponder"))]
impl NSApplication {
    extern_methods!(
        #[unsafe(method(registerUserInterfaceItemSearchHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerUserInterfaceItemSearchHandler(
            &self,
            handler: &ProtocolObject<dyn NSUserInterfaceItemSearching>,
        );

        #[unsafe(method(unregisterUserInterfaceItemSearchHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn unregisterUserInterfaceItemSearchHandler(
            &self,
            handler: &ProtocolObject<dyn NSUserInterfaceItemSearching>,
        );

        #[unsafe(method(searchString:inUserInterfaceItemString:searchRange:foundRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn searchString_inUserInterfaceItemString_searchRange_foundRange(
            &self,
            search_string: &NSString,
            string_to_search: &NSString,
            search_range: NSRange,
            found_range: *mut NSRange,
        ) -> bool;
    );
}
