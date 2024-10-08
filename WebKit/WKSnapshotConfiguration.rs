//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKSnapshotConfiguration;

    unsafe impl ClassType for WKSnapshotConfiguration {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSCopying for WKSnapshotConfiguration {}

unsafe impl CopyingHelper for WKSnapshotConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for WKSnapshotConfiguration {}

extern_methods!(
    unsafe impl WKSnapshotConfiguration {
        #[method(rect)]
        pub unsafe fn rect(&self) -> CGRect;

        #[method(setRect:)]
        pub unsafe fn setRect(&self, rect: CGRect);

        #[method_id(@__retain_semantics Other snapshotWidth)]
        pub unsafe fn snapshotWidth(&self) -> Option<Retained<NSNumber>>;

        #[method(setSnapshotWidth:)]
        pub unsafe fn setSnapshotWidth(&self, snapshot_width: Option<&NSNumber>);

        #[method(afterScreenUpdates)]
        pub unsafe fn afterScreenUpdates(&self) -> bool;

        #[method(setAfterScreenUpdates:)]
        pub unsafe fn setAfterScreenUpdates(&self, after_screen_updates: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKSnapshotConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
