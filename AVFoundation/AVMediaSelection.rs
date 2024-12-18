//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmediaselection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMediaSelection;
);

unsafe impl NSCopying for AVMediaSelection {}

unsafe impl CopyingHelper for AVMediaSelection {
    type Result = Self;
}

unsafe impl NSMutableCopying for AVMediaSelection {}

unsafe impl MutableCopyingHelper for AVMediaSelection {
    type Result = AVMutableMediaSelection;
}

unsafe impl NSObjectProtocol for AVMediaSelection {}

extern_methods!(
    unsafe impl AVMediaSelection {
        #[cfg(feature = "AVAsset")]
        #[method_id(@__retain_semantics Other asset)]
        pub unsafe fn asset(&self) -> Option<Retained<AVAsset>>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method_id(@__retain_semantics Other selectedMediaOptionInMediaSelectionGroup:)]
        pub unsafe fn selectedMediaOptionInMediaSelectionGroup(
            &self,
            media_selection_group: &AVMediaSelectionGroup,
        ) -> Option<Retained<AVMediaSelectionOption>>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method(mediaSelectionCriteriaCanBeAppliedAutomaticallyToMediaSelectionGroup:)]
        pub unsafe fn mediaSelectionCriteriaCanBeAppliedAutomaticallyToMediaSelectionGroup(
            &self,
            media_selection_group: &AVMediaSelectionGroup,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMediaSelection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmutablemediaselection?language=objc)
    #[unsafe(super(AVMediaSelection, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMutableMediaSelection;
);

unsafe impl NSCopying for AVMutableMediaSelection {}

unsafe impl CopyingHelper for AVMutableMediaSelection {
    type Result = AVMediaSelection;
}

unsafe impl NSMutableCopying for AVMutableMediaSelection {}

unsafe impl MutableCopyingHelper for AVMutableMediaSelection {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVMutableMediaSelection {}

extern_methods!(
    unsafe impl AVMutableMediaSelection {
        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method(selectMediaOption:inMediaSelectionGroup:)]
        pub unsafe fn selectMediaOption_inMediaSelectionGroup(
            &self,
            media_selection_option: Option<&AVMediaSelectionOption>,
            media_selection_group: &AVMediaSelectionGroup,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMutableMediaSelection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
