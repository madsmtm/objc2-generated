//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// Specifies the key path for a characteristic in a NSPredicate
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcharacteristickeypath?language=objc)
    pub static HMCharacteristicKeyPath: &'static NSString;
}

extern "C" {
    /// Specifies the key path for a characteristic value in a NSPredicate
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcharacteristicvaluekeypath?language=objc)
    pub static HMCharacteristicValueKeyPath: &'static NSString;
}

extern "C" {
    /// Specifies the key path for a presence event in a NSPredicate
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmpresencekeypath?language=objc)
    pub static HMPresenceKeyPath: &'static NSString;
}

extern_class!(
    /// Triggers based on events.
    ///
    ///
    /// This class represents a trigger that is based on events.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmeventtrigger?language=objc)
    #[unsafe(super(HMTrigger, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HMTrigger")]
    pub struct HMEventTrigger;
);

#[cfg(feature = "HMTrigger")]
unsafe impl Send for HMEventTrigger {}

#[cfg(feature = "HMTrigger")]
unsafe impl Sync for HMEventTrigger {}

#[cfg(feature = "HMTrigger")]
unsafe impl NSObjectProtocol for HMEventTrigger {}

extern_methods!(
    #[cfg(feature = "HMTrigger")]
    unsafe impl HMEventTrigger {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "HMEvent")]
        /// Initializes a new event trigger object.
        ///
        ///
        /// Parameter `name`: Name of the event trigger.
        ///
        ///
        /// Parameter `events`: Array of events that can trigger the evaluation of the trigger. Note: The trigger will
        /// be evaluated if any one of the events is true.
        ///
        ///
        /// Parameter `predicate`: The predicate to evaluate before executing any of the actions sets associated to this
        /// event.
        ///
        ///
        /// Returns: Instance object representing the event trigger.
        #[unsafe(method_family(init))]
        #[method_id(initWithName:events:predicate:)]
        pub unsafe fn initWithName_events_predicate(
            this: Allocated<Self>,
            name: &NSString,
            events: &NSArray<HMEvent>,
            predicate: Option<&NSPredicate>,
        ) -> Retained<Self>;

        #[cfg(feature = "HMEvent")]
        /// Initializes a new event trigger object.
        ///
        ///
        /// Parameter `name`: Name of the event trigger.
        ///
        ///
        /// Parameter `events`: Array of events that can trigger the evaluation of the trigger. Note: The trigger will
        /// be evaluated if any one of the events is true.
        ///
        ///
        /// Parameter `endEvents`: Array of end events that can trigger the restoration to the state before the scene was run.
        ///
        ///
        /// Parameter `recurrences`: Specifies the days of the week when the trigger is to be evaluated. Only the 'weekday' property
        /// is honored in NSDateComponents.
        ///
        ///
        /// Parameter `predicate`: The predicate to evaluate before executing any of the actions sets associated to this
        /// event.
        ///
        ///
        /// Returns: Instance object representing the event trigger.
        #[unsafe(method_family(init))]
        #[method_id(initWithName:events:endEvents:recurrences:predicate:)]
        pub unsafe fn initWithName_events_endEvents_recurrences_predicate(
            this: Allocated<Self>,
            name: &NSString,
            events: &NSArray<HMEvent>,
            end_events: Option<&NSArray<HMEvent>>,
            recurrences: Option<&NSArray<NSDateComponents>>,
            predicate: Option<&NSPredicate>,
        ) -> Retained<Self>;

        #[cfg(feature = "HMEvent")]
        /// The events associated with the trigger.
        #[unsafe(method_family(none))]
        #[method_id(events)]
        pub unsafe fn events(&self) -> Retained<NSArray<HMEvent>>;

        #[cfg(feature = "HMEvent")]
        /// The events that correspond to executing the restore of the scene before the trigger was executed.
        /// E.g. Execute the scene for 10 mins and restore original state is achieved by specifying a corresponding
        /// HMDurationEvent in the list of endEvents.
        #[unsafe(method_family(none))]
        #[method_id(endEvents)]
        pub unsafe fn endEvents(&self) -> Retained<NSArray<HMEvent>>;

        /// The predicate to evaluate before executing the action sets associated with the trigger.
        #[unsafe(method_family(none))]
        #[method_id(predicate)]
        pub unsafe fn predicate(&self) -> Option<Retained<NSPredicate>>;

        /// recurrences Specifies the recurrences for when the trigger is evaluated. This only supports days of the week.
        #[unsafe(method_family(none))]
        #[method_id(recurrences)]
        pub unsafe fn recurrences(&self) -> Option<Retained<NSArray<NSDateComponents>>>;

        /// Specifies whether this trigger is executed only once after which the trigger is disabled.
        #[method(executeOnce)]
        pub unsafe fn executeOnce(&self) -> bool;

        #[cfg(feature = "HMEventTriggerActivationState")]
        /// Specifies the current activation state of the trigger.
        #[method(triggerActivationState)]
        pub unsafe fn triggerActivationState(&self) -> HMEventTriggerActivationState;

        #[cfg(all(feature = "HMEvent", feature = "block2"))]
        /// Adds a new event to the event trigger.
        ///
        ///
        /// Parameter `event`: Event to add to the event trigger.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[deprecated = "Use updateEvents:completionHandler: instead"]
        #[method(addEvent:completionHandler:)]
        pub unsafe fn addEvent_completionHandler(
            &self,
            event: &HMEvent,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "HMEvent", feature = "block2"))]
        /// Removes the specified event from the event trigger.
        ///
        ///
        /// Parameter `event`: Event to remove from the event trigger.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[deprecated = "Use updateEvents:completionHandler: instead"]
        #[method(removeEvent:completionHandler:)]
        pub unsafe fn removeEvent_completionHandler(
            &self,
            event: &HMEvent,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "HMEvent", feature = "block2"))]
        /// Updates the set of events in the event trigger.
        ///
        ///
        /// Parameter `events`: Events to update in the event trigger
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[method(updateEvents:completionHandler:)]
        pub unsafe fn updateEvents_completionHandler(
            &self,
            events: &NSArray<HMEvent>,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "HMEvent", feature = "block2"))]
        /// Updates the set of events in the event trigger.
        ///
        ///
        /// Parameter `endEvents`: Events to update in the event trigger
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[method(updateEndEvents:completionHandler:)]
        pub unsafe fn updateEndEvents_completionHandler(
            &self,
            end_events: &NSArray<HMEvent>,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// This method replaces the predicate used to evaluate execution of the action sets associated with the trigger.
        ///
        ///
        /// Parameter `predicate`: The new predicate for the event trigger.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request,
        /// error will be nil on success.
        #[method(updatePredicate:completionHandler:)]
        pub unsafe fn updatePredicate_completionHandler(
            &self,
            predicate: Option<&NSPredicate>,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// This method replaces the recurrences which secifies the days of the week when the trigger is to be evaluated.
        ///
        ///
        /// Parameter `recurrences`: The new recurrences for the event trigger.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request,
        /// error will be nil on success.
        #[method(updateRecurrences:completionHandler:)]
        pub unsafe fn updateRecurrences_completionHandler(
            &self,
            recurrences: Option<&NSArray<NSDateComponents>>,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// This method is used to update whether the event trigger repeats or not.
        ///
        ///
        /// Parameter `executeOnce`: Specifies whether the event trigger is repeated or not.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[method(updateExecuteOnce:completionHandler:)]
        pub unsafe fn updateExecuteOnce_completionHandler(
            &self,
            execute_once: bool,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HMTrigger")]
    unsafe impl HMEventTrigger {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSPredicate
    #[cfg(feature = "HMTrigger")]
    unsafe impl HMEventTrigger {
        /// Creates a predicate that will evaluate whether the event occurred before a significant event.
        ///
        ///
        /// Parameter `significantEvent`: The significant event to compare against.
        /// The valid values are: HMSignificantEventSunrise and HMSignificantEventSunset.
        ///
        ///
        /// Parameter `offset`: An offset from the time of the signficant event. To specify an offset before the significant event, the
        /// properties of the NSDateComponents must be negative value. e.g. To specify 30 mins before sunset, the
        /// 'minute' property must be set to -30.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[deprecated = "Use predicateForEvaluatingTriggerOccurringBeforeSignificantEvent: instead"]
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerOccurringBeforeSignificantEvent:applyingOffset:)]
        pub unsafe fn predicateForEvaluatingTriggerOccurringBeforeSignificantEvent_applyingOffset(
            significant_event: &NSString,
            offset: Option<&NSDateComponents>,
        ) -> Retained<NSPredicate>;

        #[cfg(all(
            feature = "HMEvent",
            feature = "HMSignificantTimeEvent",
            feature = "HMTimeEvent"
        ))]
        /// Creates a predicate that will evaluate whether the event occurred before a significant event.
        ///
        ///
        /// Parameter `significantEvent`: The significant event to compare against.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerOccurringBeforeSignificantEvent:)]
        pub unsafe fn predicateForEvaluatingTriggerOccurringBeforeSignificantEvent(
            significant_event: &HMSignificantTimeEvent,
        ) -> Retained<NSPredicate>;

        /// Creates a predicate that will evaluate whether the event occurred after a significant event.
        ///
        ///
        /// Parameter `significantEvent`: The significant event to compare against.
        /// The valid values are: HMSignificantEventSunrise and HMSignificantEventSunset.
        ///
        ///
        /// Parameter `offset`: An offset from the time of the signficant event. To specify an offset after the significant event, the
        /// properties of the NSDateComponents must be positive value. e.g. To specify 30 mins after sunset, the
        /// 'minute' property must be set to 30.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[deprecated = "Use predicateForEvaluatingTriggerOccurringAfterSignificantEvent: instead"]
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerOccurringAfterSignificantEvent:applyingOffset:)]
        pub unsafe fn predicateForEvaluatingTriggerOccurringAfterSignificantEvent_applyingOffset(
            significant_event: &NSString,
            offset: Option<&NSDateComponents>,
        ) -> Retained<NSPredicate>;

        #[cfg(all(
            feature = "HMEvent",
            feature = "HMSignificantTimeEvent",
            feature = "HMTimeEvent"
        ))]
        /// Creates a predicate that will evaluate whether the event occurred after a significant event.
        ///
        ///
        /// Parameter `significantEvent`: The significant event to compare against.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerOccurringAfterSignificantEvent:)]
        pub unsafe fn predicateForEvaluatingTriggerOccurringAfterSignificantEvent(
            significant_event: &HMSignificantTimeEvent,
        ) -> Retained<NSPredicate>;

        #[cfg(all(
            feature = "HMEvent",
            feature = "HMSignificantTimeEvent",
            feature = "HMTimeEvent"
        ))]
        /// Creates a predicate that will evaluate whether the event occurred between two significant events.
        ///
        ///
        /// Parameter `firstSignificantEvent`: The first significant event.
        ///
        ///
        /// Parameter `secondSignificantEvent`: The second significant event.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerOccurringBetweenSignificantEvent:secondSignificantEvent:)]
        pub unsafe fn predicateForEvaluatingTriggerOccurringBetweenSignificantEvent_secondSignificantEvent(
            first_significant_event: &HMSignificantTimeEvent,
            second_significant_event: &HMSignificantTimeEvent,
        ) -> Retained<NSPredicate>;

        /// Creates a predicate that will evaluate whether the event occurred before the time specified.
        ///
        ///
        /// Parameter `dateComponents`: Date components representing the time to compare against when the event occurs.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerOccurringBeforeDateWithComponents:)]
        pub unsafe fn predicateForEvaluatingTriggerOccurringBeforeDateWithComponents(
            date_components: &NSDateComponents,
        ) -> Retained<NSPredicate>;

        /// Creates a predicate that will evaluate whether the event occurred at the time specified.
        ///
        ///
        /// Parameter `dateComponents`: Date components representing the time to compare against when the event occurs.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerOccurringOnDateWithComponents:)]
        pub unsafe fn predicateForEvaluatingTriggerOccurringOnDateWithComponents(
            date_components: &NSDateComponents,
        ) -> Retained<NSPredicate>;

        /// Creates a predicate that will evaluate whether the event occurred at or after the time specified.
        ///
        ///
        /// Parameter `dateComponents`: Date components representing the time to compare against when the event occurs.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerOccurringAfterDateWithComponents:)]
        pub unsafe fn predicateForEvaluatingTriggerOccurringAfterDateWithComponents(
            date_components: &NSDateComponents,
        ) -> Retained<NSPredicate>;

        /// Creates a predicate that will evaluate whether the event occurred between two times.
        ///
        ///
        /// Parameter `firstDateComponents`: The first date component.
        ///
        ///
        /// Parameter `secondDateWithComponents`: The second date component.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerOccurringBetweenDateWithComponents:secondDateWithComponents:)]
        pub unsafe fn predicateForEvaluatingTriggerOccurringBetweenDateWithComponents_secondDateWithComponents(
            first_date_components: &NSDateComponents,
            second_date_with_components: &NSDateComponents,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "HMCharacteristic")]
        /// Creates a predicate that will evaluate whether a characteristic value is related to the specified value.
        ///
        ///
        /// Parameter `characteristic`: The characteristic that is evaluated as part of the predicate.
        ///
        ///
        /// Parameter `operatorType`: The relation between the characteristic and the target value.
        /// This can be either Less Than, Greater Than, Less Than or Equal, Greater Than or Equal, Equal, or Not Equal.
        /// Anything else will cause an exception to be thrown.
        ///
        ///
        /// Parameter `value`: The value of the characteristic to compare when evaluating the predicate.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerWithCharacteristic:relatedBy:toValue:)]
        pub unsafe fn predicateForEvaluatingTriggerWithCharacteristic_relatedBy_toValue(
            characteristic: &HMCharacteristic,
            operator_type: NSPredicateOperatorType,
            value: &AnyObject,
        ) -> Retained<NSPredicate>;

        #[cfg(all(feature = "HMEvent", feature = "HMPresenceEvent"))]
        /// Creates a predicate that will evaluate based on the presence event.
        ///
        ///
        /// Parameter `presenceEvent`: The presenceEvent that is evaluated as part of the predicate.
        ///
        ///
        /// Returns: Predicate object representing a condition to evaluate before executing the action set.
        #[unsafe(method_family(none))]
        #[method_id(predicateForEvaluatingTriggerWithPresence:)]
        pub unsafe fn predicateForEvaluatingTriggerWithPresence(
            presence_event: &HMPresenceEvent,
        ) -> Retained<NSPredicate>;
    }
);
