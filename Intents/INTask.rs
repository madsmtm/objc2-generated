//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/intask?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INTask;
);

extern_conformance!(
    unsafe impl NSCoding for INTask {}
);

extern_conformance!(
    unsafe impl NSCopying for INTask {}
);

unsafe impl CopyingHelper for INTask {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INTask {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INTask {}
);

impl INTask {
    extern_methods!(
        #[cfg(all(
            feature = "INSpatialEventTrigger",
            feature = "INSpeakableString",
            feature = "INTaskPriority",
            feature = "INTaskStatus",
            feature = "INTaskType",
            feature = "INTemporalEventTrigger"
        ))]
        #[unsafe(method(initWithTitle:status:taskType:spatialEventTrigger:temporalEventTrigger:createdDateComponents:modifiedDateComponents:identifier:priority:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTitle_status_taskType_spatialEventTrigger_temporalEventTrigger_createdDateComponents_modifiedDateComponents_identifier_priority(
            this: Allocated<Self>,
            title: &INSpeakableString,
            status: INTaskStatus,
            task_type: INTaskType,
            spatial_event_trigger: Option<&INSpatialEventTrigger>,
            temporal_event_trigger: Option<&INTemporalEventTrigger>,
            created_date_components: Option<&NSDateComponents>,
            modified_date_components: Option<&NSDateComponents>,
            identifier: Option<&NSString>,
            priority: INTaskPriority,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "INSpatialEventTrigger",
            feature = "INSpeakableString",
            feature = "INTaskStatus",
            feature = "INTaskType",
            feature = "INTemporalEventTrigger"
        ))]
        #[unsafe(method(initWithTitle:status:taskType:spatialEventTrigger:temporalEventTrigger:createdDateComponents:modifiedDateComponents:identifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTitle_status_taskType_spatialEventTrigger_temporalEventTrigger_createdDateComponents_modifiedDateComponents_identifier(
            this: Allocated<Self>,
            title: &INSpeakableString,
            status: INTaskStatus,
            task_type: INTaskType,
            spatial_event_trigger: Option<&INSpatialEventTrigger>,
            temporal_event_trigger: Option<&INTemporalEventTrigger>,
            created_date_components: Option<&NSDateComponents>,
            modified_date_components: Option<&NSDateComponents>,
            identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "INSpeakableString")]
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Retained<INSpeakableString>;

        #[cfg(feature = "INTaskStatus")]
        #[unsafe(method(status))]
        #[unsafe(method_family = none)]
        pub unsafe fn status(&self) -> INTaskStatus;

        #[cfg(feature = "INSpatialEventTrigger")]
        #[unsafe(method(spatialEventTrigger))]
        #[unsafe(method_family = none)]
        pub unsafe fn spatialEventTrigger(&self) -> Option<Retained<INSpatialEventTrigger>>;

        #[cfg(feature = "INTemporalEventTrigger")]
        #[unsafe(method(temporalEventTrigger))]
        #[unsafe(method_family = none)]
        pub unsafe fn temporalEventTrigger(&self) -> Option<Retained<INTemporalEventTrigger>>;

        #[unsafe(method(createdDateComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn createdDateComponents(&self) -> Option<Retained<NSDateComponents>>;

        #[unsafe(method(modifiedDateComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn modifiedDateComponents(&self) -> Option<Retained<NSDateComponents>>;

        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "INTaskType")]
        #[unsafe(method(taskType))]
        #[unsafe(method_family = none)]
        pub unsafe fn taskType(&self) -> INTaskType;

        #[cfg(feature = "INTaskPriority")]
        #[unsafe(method(priority))]
        #[unsafe(method_family = none)]
        pub unsafe fn priority(&self) -> INTaskPriority;
    );
}

/// Methods declared on superclass `NSObject`.
impl INTask {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
