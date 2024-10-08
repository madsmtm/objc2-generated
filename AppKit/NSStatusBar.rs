//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub static NSVariableStatusItemLength: CGFloat = -1.0 as _;

pub static NSSquareStatusItemLength: CGFloat = -2.0 as _;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStatusBar;

    unsafe impl ClassType for NSStatusBar {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSStatusBar {}

extern_methods!(
    unsafe impl NSStatusBar {
        #[method_id(@__retain_semantics Other systemStatusBar)]
        pub unsafe fn systemStatusBar() -> Retained<NSStatusBar>;

        #[cfg(feature = "NSStatusItem")]
        #[method_id(@__retain_semantics Other statusItemWithLength:)]
        pub unsafe fn statusItemWithLength(&self, length: CGFloat) -> Retained<NSStatusItem>;

        #[cfg(feature = "NSStatusItem")]
        #[method(removeStatusItem:)]
        pub unsafe fn removeStatusItem(&self, item: &NSStatusItem);

        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;

        #[method(thickness)]
        pub unsafe fn thickness(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSStatusBar {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
