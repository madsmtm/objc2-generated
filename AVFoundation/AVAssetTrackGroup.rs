//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassettrackgroup?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetTrackGroup;
);

unsafe impl Send for AVAssetTrackGroup {}

unsafe impl Sync for AVAssetTrackGroup {}

unsafe impl NSCopying for AVAssetTrackGroup {}

unsafe impl CopyingHelper for AVAssetTrackGroup {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVAssetTrackGroup {}

extern_methods!(
    unsafe impl AVAssetTrackGroup {
        #[method_id(@__retain_semantics Other trackIDs)]
        pub unsafe fn trackIDs(&self) -> Retained<NSArray<NSNumber>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAssetTrackGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
