//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UILexiconEntry;

    unsafe impl ClassType for UILexiconEntry {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSCopying for UILexiconEntry {}

unsafe impl CopyingHelper for UILexiconEntry {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UILexiconEntry {}

extern_methods!(
    unsafe impl UILexiconEntry {
        #[method_id(@__retain_semantics Other documentText)]
        pub unsafe fn documentText(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other userInput)]
        pub unsafe fn userInput(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UILexiconEntry {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UILexicon;

    unsafe impl ClassType for UILexicon {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSCopying for UILexicon {}

unsafe impl CopyingHelper for UILexicon {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UILexicon {}

extern_methods!(
    unsafe impl UILexicon {
        #[method_id(@__retain_semantics Other entries)]
        pub unsafe fn entries(&self) -> Retained<NSArray<UILexiconEntry>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UILexicon {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
