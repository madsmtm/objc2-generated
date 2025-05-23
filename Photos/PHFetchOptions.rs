//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phfetchoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHFetchOptions;
);

extern_conformance!(
    unsafe impl NSCopying for PHFetchOptions {}
);

unsafe impl CopyingHelper for PHFetchOptions {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for PHFetchOptions {}
);

impl PHFetchOptions {
    extern_methods!(
        #[unsafe(method(predicate))]
        #[unsafe(method_family = none)]
        pub unsafe fn predicate(&self) -> Option<Retained<NSPredicate>>;

        /// Setter for [`predicate`][Self::predicate].
        #[unsafe(method(setPredicate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

        #[unsafe(method(sortDescriptors))]
        #[unsafe(method_family = none)]
        pub unsafe fn sortDescriptors(&self) -> Option<Retained<NSArray<NSSortDescriptor>>>;

        /// Setter for [`sortDescriptors`][Self::sortDescriptors].
        #[unsafe(method(setSortDescriptors:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSortDescriptors(
            &self,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
        );

        #[unsafe(method(includeHiddenAssets))]
        #[unsafe(method_family = none)]
        pub unsafe fn includeHiddenAssets(&self) -> bool;

        /// Setter for [`includeHiddenAssets`][Self::includeHiddenAssets].
        #[unsafe(method(setIncludeHiddenAssets:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIncludeHiddenAssets(&self, include_hidden_assets: bool);

        #[unsafe(method(includeAllBurstAssets))]
        #[unsafe(method_family = none)]
        pub unsafe fn includeAllBurstAssets(&self) -> bool;

        /// Setter for [`includeAllBurstAssets`][Self::includeAllBurstAssets].
        #[unsafe(method(setIncludeAllBurstAssets:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIncludeAllBurstAssets(&self, include_all_burst_assets: bool);

        #[cfg(feature = "PhotosTypes")]
        #[unsafe(method(includeAssetSourceTypes))]
        #[unsafe(method_family = none)]
        pub unsafe fn includeAssetSourceTypes(&self) -> PHAssetSourceType;

        #[cfg(feature = "PhotosTypes")]
        /// Setter for [`includeAssetSourceTypes`][Self::includeAssetSourceTypes].
        #[unsafe(method(setIncludeAssetSourceTypes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIncludeAssetSourceTypes(
            &self,
            include_asset_source_types: PHAssetSourceType,
        );

        #[unsafe(method(fetchLimit))]
        #[unsafe(method_family = none)]
        pub unsafe fn fetchLimit(&self) -> NSUInteger;

        /// Setter for [`fetchLimit`][Self::fetchLimit].
        #[unsafe(method(setFetchLimit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFetchLimit(&self, fetch_limit: NSUInteger);

        #[unsafe(method(wantsIncrementalChangeDetails))]
        #[unsafe(method_family = none)]
        pub unsafe fn wantsIncrementalChangeDetails(&self) -> bool;

        /// Setter for [`wantsIncrementalChangeDetails`][Self::wantsIncrementalChangeDetails].
        #[unsafe(method(setWantsIncrementalChangeDetails:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWantsIncrementalChangeDetails(
            &self,
            wants_incremental_change_details: bool,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl PHFetchOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
