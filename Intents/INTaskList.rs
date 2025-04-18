//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/intasklist?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INTaskList;
);

extern_conformance!(
    unsafe impl NSCoding for INTaskList {}
);

extern_conformance!(
    unsafe impl NSCopying for INTaskList {}
);

unsafe impl CopyingHelper for INTaskList {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INTaskList {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INTaskList {}
);

impl INTaskList {
    extern_methods!(
        #[cfg(all(feature = "INSpeakableString", feature = "INTask"))]
        #[unsafe(method(initWithTitle:tasks:groupName:createdDateComponents:modifiedDateComponents:identifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTitle_tasks_groupName_createdDateComponents_modifiedDateComponents_identifier(
            this: Allocated<Self>,
            title: &INSpeakableString,
            tasks: &NSArray<INTask>,
            group_name: Option<&INSpeakableString>,
            created_date_components: Option<&NSDateComponents>,
            modified_date_components: Option<&NSDateComponents>,
            identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "INSpeakableString")]
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Retained<INSpeakableString>;

        #[cfg(feature = "INTask")]
        #[unsafe(method(tasks))]
        #[unsafe(method_family = none)]
        pub unsafe fn tasks(&self) -> Retained<NSArray<INTask>>;

        #[cfg(feature = "INSpeakableString")]
        #[unsafe(method(groupName))]
        #[unsafe(method_family = none)]
        pub unsafe fn groupName(&self) -> Option<Retained<INSpeakableString>>;

        #[unsafe(method(createdDateComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn createdDateComponents(&self) -> Option<Retained<NSDateComponents>>;

        #[unsafe(method(modifiedDateComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn modifiedDateComponents(&self) -> Option<Retained<NSDateComponents>>;

        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSString>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl INTaskList {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
