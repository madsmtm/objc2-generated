//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-foundation")]
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "objc2")]
extern_class!(
    /// A mutable Function Block object created by the client process.
    ///
    ///
    /// A Function Block created with this API may be used in the Function Block configuration
    /// of a client-created MIDIUMPMutableEndpoint.
    ///
    /// This API is not realtime-safe, all interaction with the function block should be done on the
    /// main thread.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremidi/midiumpmutablefunctionblock?language=objc)
    #[unsafe(super(MIDIUMPFunctionBlock, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MIDIUMPFunctionBlock", feature = "objc2"))]
    pub struct MIDIUMPMutableFunctionBlock;
);

#[cfg(all(feature = "MIDIUMPFunctionBlock", feature = "objc2"))]
unsafe impl NSObjectProtocol for MIDIUMPMutableFunctionBlock {}

#[cfg(feature = "objc2")]
extern_methods!(
    #[cfg(all(feature = "MIDIUMPFunctionBlock", feature = "objc2"))]
    unsafe impl MIDIUMPMutableFunctionBlock {
        #[cfg(all(feature = "MIDIUMPEndpoint", feature = "MIDIUMPMutableEndpoint"))]
        /// The UMP Endpoint to which this Function Block is registered.
        #[method_id(@__retain_semantics Other UMPEndpoint)]
        pub unsafe fn UMPEndpoint(&self) -> Option<Retained<MIDIUMPMutableEndpoint>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "MIDIMessages", feature = "objc2-foundation"))]
        /// The initializer for constructing a Function Block.
        ///
        ///
        /// Parameter `name`: The Function Block name.
        ///
        /// Parameter `direction`: The directionality of the Function Block.
        ///
        /// Parameter `firstGroup`: The first UMP Group supported by the Function Block.
        ///
        /// Parameter `totalGroupsSpanned`: The number of UMP groups spanned by the Function Block.
        ///
        /// Parameter `maxSysEx8Streams`: The maximum number of simultaneous Sysex8 streams.
        ///
        /// Parameter `MIDI1Info`: The MIDI 1.0 speed information for the Function Block.
        ///
        /// Parameter `UIHint`: A UI hint for the Function Block.
        ///
        /// Parameter `isEnabled`: The enable state of the Function Block.
        ///
        ///
        /// This operation will fail if virtual MIDI endpoint creation is not allowed
        /// (for example, on iOS, if your app doesn't list 'audio' in UIBackgroundModes).
        #[method_id(@__retain_semantics Init initWithName:direction:firstGroup:totalGroupsSpanned:maxSysEx8Streams:MIDI1Info:UIHint:isEnabled:)]
        pub unsafe fn initWithName_direction_firstGroup_totalGroupsSpanned_maxSysEx8Streams_MIDI1Info_UIHint_isEnabled(
            this: Allocated<Self>,
            name: &NSString,
            direction: MIDIUMPFunctionBlockDirection,
            first_group: MIDIUMPGroupNumber,
            total_groups_spanned: MIDIUInteger7,
            max_sys_ex8_streams: MIDIUInteger7,
            midi1_info: MIDIUMPFunctionBlockMIDI1Info,
            ui_hint: MIDIUMPFunctionBlockUIHint,
            is_enabled: bool,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-foundation")]
        /// Set whether this Function Block is enabled or disabled.
        ///
        ///
        /// Parameter `isEnabled`: The new state of the Function Block.
        ///
        /// Parameter `error`: The out-error used if an error occurred.
        ///
        ///
        /// Returns: YES for success. NO in the event of a failure, in which case the error is returned in error.
        ///
        ///
        /// If a Function Block is registered to UMP Endpoint as part of a static configuration,
        /// the state must always be enabled and may not change. If registered to a UMP Endpoint,
        /// changes to the Function Block state are propagated to the system-wide cache.
        #[method(setEnabled:error:_)]
        pub unsafe fn setEnabled_error(&self, is_enabled: bool) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "objc2-foundation")]
        /// Set the function block name.
        ///
        ///
        /// Parameter `name`: A string representing the name of the function block.
        ///
        /// Parameter `error`: The out-error used if an error occurs.
        ///
        ///
        /// Returns: YES for success. NO in the event of a failure, in which case the error is returned in error.
        ///
        ///
        /// The Function Block name string. Updating the name of a Function Block will cause the
        /// updated name to be propagated to all local copies of the system-wide cache.
        #[method(setName:error:_)]
        pub unsafe fn setName_error(&self, name: &NSString) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "MIDIMessages", feature = "objc2-foundation"))]
        /// Reconfigure a Function Block.
        ///
        ///
        /// Parameter `firstGroup`: The new first Group to use for the Function Block..
        ///
        /// Parameter `direction`: The direction of the Function Block: input, output, or bidirectional.
        ///
        /// Parameter `MIDI1Info`: MIDI 1.0 speed information.
        ///
        /// Parameter `UIHint`: A hint for UI about the primary usage of this Function Block.
        ///
        ///
        /// If a mutable Function Block has not been registered to a CI device or was registered in
        /// a non-static Function Block configuration, the first Group can be changed if the final
        /// Group spanned by the Function Block is valid after the Function Block has been
        /// relocated.
        /// Returns YES if the first Group of the Function Block was changed.
        #[method(reconfigureWithFirstGroup:direction:MIDI1Info:UIHint:error:_)]
        pub unsafe fn reconfigureWithFirstGroup_direction_MIDI1Info_UIHint_error(
            &self,
            first_group: MIDIUMPGroupNumber,
            direction: MIDIUMPFunctionBlockDirection,
            midi1_info: MIDIUMPFunctionBlockMIDI1Info,
            ui_hint: MIDIUMPFunctionBlockUIHint,
        ) -> Result<(), Retained<NSError>>;
    }
);

#[cfg(feature = "objc2")]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MIDIUMPFunctionBlock", feature = "objc2"))]
    unsafe impl MIDIUMPMutableFunctionBlock {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);