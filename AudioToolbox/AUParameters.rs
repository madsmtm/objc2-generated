//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// A value of an audio unit parameter.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auvalue?language=objc)
pub type AUValue = c_float;

/// Numeric identifier for audio unit parameter.
///
/// Note that parameter addresses are not necessarily persistent, unless the individual audio
/// unit documents a promise to maintain its addressing scheme. Hosts should bind to parameters'
/// key paths.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparameteraddress?language=objc)
pub type AUParameterAddress = u64;

/// Identifies the different types of parameter automation events.
///
///
/// Audio Units may generate parameter changes from their user interfaces. Hosts may attach
/// significance to the beginning and end of a UI gesture (typically touching and releasing
/// a fader). These gestures are conveyed through these types of automation events.
///
///
/// The event contains an updated value for the parameter.
///
/// The event marks an initial "touch" gesture on a UI element.
///
/// The event marks a final "release" gesture on a UI element.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparameterautomationeventtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AUParameterAutomationEventType(pub u32);
impl AUParameterAutomationEventType {
    #[doc(alias = "AUParameterAutomationEventTypeValue")]
    pub const Value: Self = Self(0);
    #[doc(alias = "AUParameterAutomationEventTypeTouch")]
    pub const Touch: Self = Self(1);
    #[doc(alias = "AUParameterAutomationEventTypeRelease")]
    pub const Release: Self = Self(2);
}

unsafe impl Encode for AUParameterAutomationEventType {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for AUParameterAutomationEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// An event recording the changing of a parameter at a particular host time.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aurecordedparameterevent?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AURecordedParameterEvent {
    /// The host time at which the event occurred.
    pub hostTime: u64,
    /// The address of the parameter whose value changed.
    pub address: AUParameterAddress,
    /// The value of the parameter at that time.
    pub value: AUValue,
}

unsafe impl Encode for AURecordedParameterEvent {
    const ENCODING: Encoding = Encoding::Struct(
        "AURecordedParameterEvent",
        &[
            <u64>::ENCODING,
            <AUParameterAddress>::ENCODING,
            <AUValue>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for AURecordedParameterEvent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// An event recording the changing of a parameter, possibly including events
/// such as touch and release gestures, at a particular host time.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparameterautomationevent?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUParameterAutomationEvent {
    /// The host time at which the event occurred.
    pub hostTime: u64,
    /// The address of the parameter whose value changed.
    pub address: AUParameterAddress,
    /// The value of the parameter at that time.
    pub value: AUValue,
    /// The type of the event.
    pub eventType: AUParameterAutomationEventType,
    pub reserved: u64,
}

unsafe impl Encode for AUParameterAutomationEvent {
    const ENCODING: Encoding = Encoding::Struct(
        "AUParameterAutomationEvent",
        &[
            <u64>::ENCODING,
            <AUParameterAddress>::ENCODING,
            <AUValue>::ENCODING,
            <AUParameterAutomationEventType>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for AUParameterAutomationEvent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A block called to signal that the value of a parameter has changed.
///
/// See the discussion of -[AUParameterNode tokenByAddingParameterObserver:].
///
/// Parameter `address`: The address of the parameter whose value changed.
///
/// Parameter `value`: The current value of the parameter.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparameterobserver?language=objc)
#[cfg(feature = "block2")]
pub type AUParameterObserver = *mut block2::Block<dyn Fn(AUParameterAddress, AUValue)>;

/// A block called to record parameter changes as automation events.
///
/// See the discussion of -[AUParameterNode tokenByAddingParameterRecordingObserver:].
///
/// Parameter `numberEvents`: The number of events being delivered.
///
/// Parameter `events`: The events being delivered.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparameterrecordingobserver?language=objc)
#[cfg(feature = "block2")]
pub type AUParameterRecordingObserver =
    *mut block2::Block<dyn Fn(NSInteger, NonNull<AURecordedParameterEvent>)>;

/// A block called to record parameter changes as automation events.
///
/// See the discussion of -[AUParameterNode tokenByAddingParameterAutomationObserver:].
///
/// Parameter `numberEvents`: The number of events being delivered.
///
/// Parameter `events`: The events being delivered.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparameterautomationobserver?language=objc)
#[cfg(feature = "block2")]
pub type AUParameterAutomationObserver =
    *mut block2::Block<dyn Fn(NSInteger, NonNull<AUParameterAutomationEvent>)>;

/// A token representing an installed AUParameterObserver, AUParameterRecordingObserver,
/// or AUParameterAutomationObserver.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparameterobservertoken?language=objc)
pub type AUParameterObserverToken = *mut c_void;

extern_class!(
    /// A node in an audio unit's tree of parameters.
    ///
    /// Nodes are instances of either AUParameterGroup or AUParameter.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparameternode?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AUParameterNode;
);

unsafe impl NSObjectProtocol for AUParameterNode {}

extern_methods!(
    unsafe impl AUParameterNode {
        /// A non-localized, permanent name for a parameter or group.
        ///
        /// The identifier must be unique for all child nodes under any given parent. From release to
        /// release, an audio unit must not change its parameters' identifiers; this will invalidate any
        /// hosts' documents that refer to the parameters.
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        /// Generated by concatenating the identifiers of a node's parents with its own.
        ///
        /// Unless an audio unit specifically documents that its parameter addresses are stable and
        /// persistent, hosts, when recording parameter values, should bind to the keyPath.
        ///
        /// The individual node identifiers in a key path are separated by periods. (".")
        ///
        /// Passing a node's keyPath to -[tree valueForKeyPath:] should return the same node.
        #[method_id(@__retain_semantics Other keyPath)]
        pub unsafe fn keyPath(&self) -> Retained<NSString>;

        /// A localized name to display for the parameter.
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Retained<NSString>;

        /// A version of displayName possibly abbreviated to the given desired length, in characters.
        ///
        /// The default implementation simply returns displayName.
        #[method_id(@__retain_semantics Other displayNameWithLength:)]
        pub unsafe fn displayNameWithLength(&self, maximum_length: NSInteger)
            -> Retained<NSString>;

        #[cfg(feature = "block2")]
        /// Add an observer for a parameter or all parameters in a group/tree.
        ///
        /// An audio unit view can use an AUParameterObserver to be notified of changes
        /// to a single parameter, or to all parameters in a group/tree.
        ///
        /// These callbacks are throttled so as to limit the rate of redundant notifications
        /// in the case of frequent changes to a single parameter.
        ///
        /// This block is called in an arbitrary thread context. It is responsible for thread-safety.
        /// It must not make any calls to add or remove other observers, including itself;
        /// this will deadlock.
        ///
        /// An audio unit's implementation should interact with the parameter object via
        /// implementorValueObserver and implementorValueProvider.
        ///
        /// Parameter observers are bound to a specific parameter instance. If this parameter is
        /// destroyed, e.g. if the parameter tree is re-constructed, the previously set parameter
        /// observers will no longer be valid. Clients can observe changes to the parameter tree
        /// via KVO. See the discussion of -[AUAudioUnit parameterTree].
        ///
        /// Parameter `observer`: A block to call after the value of a parameter has changed.
        ///
        /// Returns: A token which can be passed to removeParameterObserver: or to -[AUParameter setValue:originator:]
        #[method(tokenByAddingParameterObserver:)]
        pub unsafe fn tokenByAddingParameterObserver(
            &self,
            observer: AUParameterObserver,
        ) -> AUParameterObserverToken;

        #[cfg(feature = "block2")]
        /// Add a recording observer for a parameter or all parameters in a group/tree.
        ///
        /// This is a variant of tokenByAddingParameterAutomationObserver where the callback receives
        /// AURecordedParameterEvents instead of AUParameterAutomationEvents.
        ///
        /// This will be deprecated in favor of tokenByAddingParameterAutomationObserver in a future
        /// release.
        #[method(tokenByAddingParameterRecordingObserver:)]
        pub unsafe fn tokenByAddingParameterRecordingObserver(
            &self,
            observer: AUParameterRecordingObserver,
        ) -> AUParameterObserverToken;

        #[cfg(feature = "block2")]
        /// Add a recording observer for a parameter or all parameters in a group/tree.
        ///
        /// An audio unit host can use an AUParameterAutomationObserver or AUParameterRecordingObserver
        /// to capture a series of changes to a parameter value, including the timing of the events, as
        /// generated by a UI gesture in a view, for example.
        ///
        /// Unlike AUParameterObserver, these callbacks are not throttled.
        ///
        /// This block is called in an arbitrary thread context. It is responsible for thread-safety.
        /// It must not make any calls to add or remove other observers, including itself;
        /// this will deadlock.
        ///
        /// An audio unit's engine should interact with the parameter object via
        /// implementorValueObserver and implementorValueProvider.
        ///
        /// Parameter `observer`: A block to call to record the changing of a parameter value.
        ///
        /// Returns: A token which can be passed to removeParameterObserver: or to -[AUParameter
        /// setValue:originator:]
        #[method(tokenByAddingParameterAutomationObserver:)]
        pub unsafe fn tokenByAddingParameterAutomationObserver(
            &self,
            observer: AUParameterAutomationObserver,
        ) -> AUParameterObserverToken;

        /// Remove an observer created with tokenByAddingParameterObserver,
        /// tokenByAddingParameterRecordingObserver, or tokenByAddingParameterAutomationObserver.
        ///
        /// This call will remove the callback corresponding to the supplied token. Note that this
        /// will block until any callbacks currently in flight have completed.
        #[method(removeParameterObserver:)]
        pub unsafe fn removeParameterObserver(&self, token: AUParameterObserverToken);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AUParameterNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A group of related parameters.
    ///
    /// A parameter group is KVC-compliant for its children; e.g. valueForKey:
    /// "
    /// volume" will
    /// return a child parameter whose identifier is "volume".
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparametergroup?language=objc)
    #[unsafe(super(AUParameterNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AUParameterGroup;
);

unsafe impl NSCoding for AUParameterGroup {}

unsafe impl NSObjectProtocol for AUParameterGroup {}

unsafe impl NSSecureCoding for AUParameterGroup {}

extern_methods!(
    unsafe impl AUParameterGroup {
        /// The group's child nodes (AUParameterGroupNode).
        #[method_id(@__retain_semantics Other children)]
        pub unsafe fn children(&self) -> Retained<NSArray<AUParameterNode>>;

        /// Returns a flat array of all parameters in the group, including those in child groups.
        #[method_id(@__retain_semantics Other allParameters)]
        pub unsafe fn allParameters(&self) -> Retained<NSArray<AUParameter>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AUParameterGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// The top level group node, representing all of an audio unit's parameters.
    ///
    /// An audio unit's parameters are organized into a tree containing groups and parameters.
    /// Groups may be nested.
    ///
    /// The tree is KVO-compliant. For example, if the tree contains a group named "LFO" ,
    /// containing a parameter named rate, then "LFO.rate" refers to that parameter object, and
    /// "LFO.rate.value" refers to that parameter's value.
    ///
    /// An audio unit may choose to dynamically rearrange the tree. When doing so, it must
    /// issue a KVO notification on the audio unit's parameterTree property. The tree's elements are
    /// mostly immutable (except for values and implementor hooks); the only way to modify them
    /// is to publish a new tree.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparametertree?language=objc)
    #[unsafe(super(AUParameterGroup, AUParameterNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AUParameterTree;
);

unsafe impl NSCoding for AUParameterTree {}

unsafe impl NSObjectProtocol for AUParameterTree {}

unsafe impl NSSecureCoding for AUParameterTree {}

extern_methods!(
    unsafe impl AUParameterTree {
        /// Search a tree for a parameter with a specific address.
        ///
        /// Returns: The parameter corresponding to the supplied address, or nil if no such parameter exists.
        #[method_id(@__retain_semantics Other parameterWithAddress:)]
        pub unsafe fn parameterWithAddress(
            &self,
            address: AUParameterAddress,
        ) -> Option<Retained<AUParameter>>;

        #[cfg(feature = "AUComponent")]
        /// Search a tree for a specific v2 audio unit parameter.
        ///
        /// V2 audio units publish parameters identified by a parameter ID, scope, and element.
        /// A host that knows that it is dealing with a v2 unit can locate parameters using this method,
        /// for example, for the Apple-supplied system audio units.
        ///
        /// Returns: The parameter corresponding to the supplied ID/scope/element, or nil if no such parameter
        /// exists, or if the audio unit is not a v2 unit.
        #[method_id(@__retain_semantics Other parameterWithID:scope:element:)]
        pub unsafe fn parameterWithID_scope_element(
            &self,
            param_id: AudioUnitParameterID,
            scope: AudioUnitScope,
            element: AudioUnitElement,
        ) -> Option<Retained<AUParameter>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AUParameterTree {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A node representing a single parameter.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/auparameter?language=objc)
    #[unsafe(super(AUParameterNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AUParameter;
);

unsafe impl NSCoding for AUParameter {}

unsafe impl NSObjectProtocol for AUParameter {}

unsafe impl NSSecureCoding for AUParameter {}

extern_methods!(
    unsafe impl AUParameter {
        /// The parameter's minimum value.
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> AUValue;

        /// The parameter's maximum value.
        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> AUValue;

        #[cfg(feature = "AudioUnitProperties")]
        /// The parameter's unit of measurement.
        #[method(unit)]
        pub unsafe fn unit(&self) -> AudioUnitParameterUnit;

        /// A localized name for the parameter's unit. Supplied by the AU if kAudioUnitParameterUnit_CustomUnit; else by the framework.
        #[method_id(@__retain_semantics Other unitName)]
        pub unsafe fn unitName(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "AudioUnitProperties")]
        /// Various details of the parameter.
        #[method(flags)]
        pub unsafe fn flags(&self) -> AudioUnitParameterOptions;

        /// The parameter's address.
        #[method(address)]
        pub unsafe fn address(&self) -> AUParameterAddress;

        /// For parameters with kAudioUnitParameterUnit_Indexed, localized strings corresponding
        /// to the values.
        #[method_id(@__retain_semantics Other valueStrings)]
        pub unsafe fn valueStrings(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Parameters whose values may change as a side effect of this parameter's value
        /// changing.
        ///
        /// Each array value is an NSNumber representing AUParameterAddress.
        #[method_id(@__retain_semantics Other dependentParameters)]
        pub unsafe fn dependentParameters(&self) -> Option<Retained<NSArray<NSNumber>>>;

        /// The parameter's current value.
        #[method(value)]
        pub unsafe fn value(&self) -> AUValue;

        /// Setter for [`value`][Self::value].
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: AUValue);

        /// Set the parameter's value, avoiding redundant notifications to the originator.
        ///
        /// Bridged to the v2 function AudioUnitSetParameter.
        #[method(setValue:originator:)]
        pub unsafe fn setValue_originator(
            &self,
            value: AUValue,
            originator: AUParameterObserverToken,
        );

        /// Convenience for setValue:originator:atHostTime:eventType:
        ///
        /// Bridged to the v2 function AudioUnitSetParameter.
        #[method(setValue:originator:atHostTime:)]
        pub unsafe fn setValue_originator_atHostTime(
            &self,
            value: AUValue,
            originator: AUParameterObserverToken,
            host_time: u64,
        );

        /// Set the parameter's value, preserving the host time of the gesture that initiated the
        /// change, and associating an event type such as touch/release.
        ///
        /// In general, this method should only be called from a user interface. It initiates a change
        /// to a parameter in a way that captures the gesture such that it can be recorded later --
        /// any AUParameterAutomationObservers will receive the host time and event type associated
        /// with the parameter change.
        ///
        /// From an audio playback engine, a host should schedule automated parameter changes through
        /// AUAudioUnit's scheduleParameterBlock.
        ///
        /// Bridged to the v2 function AudioUnitSetParameter.
        #[method(setValue:originator:atHostTime:eventType:)]
        pub unsafe fn setValue_originator_atHostTime_eventType(
            &self,
            value: AUValue,
            originator: AUParameterObserverToken,
            host_time: u64,
            event_type: AUParameterAutomationEventType,
        );

        /// Get a textual representation of a value for the parameter. Use value==nil to use the
        /// current value. Bridged to the v2 property kAudioUnitProperty_ParameterStringFromValue.
        ///
        /// This is currently only supported for parameters whose flags include
        /// kAudioUnitParameterFlag_ValuesHaveStrings.
        #[method_id(@__retain_semantics Other stringFromValue:)]
        pub unsafe fn stringFromValue(&self, value: *const AUValue) -> Retained<NSString>;

        /// Convert a textual representation of a value to a numeric one.
        ///
        /// This is currently only supported for parameters whose flags include
        /// kAudioUnitParameterFlag_ValuesHaveStrings.
        #[method(valueFromString:)]
        pub unsafe fn valueFromString(&self, string: &NSString) -> AUValue;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AUParameter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);