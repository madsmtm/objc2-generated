//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// An opaque reference to a play-through connection.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremidi/midithruconnectionref?language=objc)
#[cfg(feature = "MIDIServices")]
pub type MIDIThruConnectionRef = MIDIObjectRef;

/// A custom mapping function to transform MIDI 7-bit values,
/// as contained in note numbers, velocities, control values,
/// etc.  y = value[x], where x is the input MIDI value, y the
/// output.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremidi/midivaluemap?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDIValueMap {
    pub value: [u8; 128],
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MIDIValueMap {
    const ENCODING: Encoding = Encoding::Struct("MIDIValueMap", &[<[u8; 128]>::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MIDIValueMap {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Values specifying a type of MIDI transformation, as found in the transform member of MIDITransform.
///
///
/// no transformation (param unused)
///
/// filter out the specified event type (param unused)
///
/// transform one control number to another; param is destination control number
///
/// add param to values
///
/// multiple value by the fixed point number in param, which is in fixed point: bbbb.bbbb bbbb bbbb
///
/// the value's minimum value is param
///
/// the value's maximum value is param
///
/// transform the value using a map; param is the index of the map in the connection's array of maps.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremidi/miditransformtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MIDITransformType(pub u16);
impl MIDITransformType {
    #[doc(alias = "kMIDITransform_None")]
    pub const None: Self = Self(0);
    #[doc(alias = "kMIDITransform_FilterOut")]
    pub const FilterOut: Self = Self(1);
    #[doc(alias = "kMIDITransform_MapControl")]
    pub const MapControl: Self = Self(2);
    #[doc(alias = "kMIDITransform_Add")]
    pub const Add: Self = Self(8);
    #[doc(alias = "kMIDITransform_Scale")]
    pub const Scale: Self = Self(9);
    #[doc(alias = "kMIDITransform_MinValue")]
    pub const MinValue: Self = Self(10);
    #[doc(alias = "kMIDITransform_MaxValue")]
    pub const MaxValue: Self = Self(11);
    #[doc(alias = "kMIDITransform_MapValue")]
    pub const MapValue: Self = Self(12);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MIDITransformType {
    const ENCODING: Encoding = u16::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MIDITransformType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremidi/kmidithruconnection_maxendpoints?language=objc)
pub const kMIDIThruConnection_MaxEndpoints: c_uint = 8;

/// Specifies how control numbers are interpreted.
///
///
/// control numbers may be 0-127
///
/// control numbers may be 0-31
///
/// control numbers may be 0-16383
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremidi/miditransformcontroltype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MIDITransformControlType(pub u8);
impl MIDITransformControlType {
    #[doc(alias = "kMIDIControlType_7Bit")]
    pub const ControlType_7Bit: Self = Self(0);
    #[doc(alias = "kMIDIControlType_14Bit")]
    pub const ControlType_14Bit: Self = Self(1);
    #[doc(alias = "kMIDIControlType_7BitRPN")]
    pub const ControlType_7BitRPN: Self = Self(2);
    #[doc(alias = "kMIDIControlType_14BitRPN")]
    pub const ControlType_14BitRPN: Self = Self(3);
    #[doc(alias = "kMIDIControlType_7BitNRPN")]
    pub const ControlType_7BitNRPN: Self = Self(4);
    #[doc(alias = "kMIDIControlType_14BitNRPN")]
    pub const ControlType_14BitNRPN: Self = Self(5);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MIDITransformControlType {
    const ENCODING: Encoding = u8::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MIDITransformControlType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes how a single type of MIDI event is transformed.
///
/// This structure controls the transformation of various MIDI events other than control changes.
///
/// Field: transform   The type of transformation to be applied to the event values.
/// Field: param       An argument to the transformation method (see description of MIDITransformType).
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremidi/miditransform?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDITransform {
    pub transform: MIDITransformType,
    pub param: i16,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MIDITransform {
    const ENCODING: Encoding = Encoding::Struct(
        "MIDITransform",
        &[<MIDITransformType>::ENCODING, <i16>::ENCODING],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MIDITransform {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes a transformation of MIDI control change events.
///
/// A single MIDIThruConnectionParams may describe any number of transformations to control
/// events. It is important that multiple transformations are ordered correctly: filter out,
/// remap, then alter values.
///
/// All transformations are done internally using 14-bit values, so for example, when doing
/// an add/min/max transform on a 7-bit control value, the parameter must be a 14-bit value.
/// For example, to add 10 to a control value, param must be (10
/// <
/// <
/// 7) = 1280.
///
/// As per the MIDI specification, a number of controls are interpreted specially:
///
/// Control | Function
/// --------|---------
/// 32-63   | the LSBs of 0-31
/// 6/38    | data entry
/// 96, 97  | data increment, decrement
/// 98-101  | NRPN/RPN
///
/// Field: controlType         The type of control specified by controlNumber
/// Field: remappedControlType If transform is kMIDITransform_MapControl, the output control type
/// Field: controlNumber       The control number to be affected.
/// Field: transform           The type of transformation to be applied to the event values.
/// Field: param               An argument to the transformation method (see description of MIDITransformType).
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremidi/midicontroltransform?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDIControlTransform {
    pub controlType: MIDITransformControlType,
    pub remappedControlType: MIDITransformControlType,
    pub controlNumber: u16,
    pub transform: MIDITransformType,
    pub param: i16,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MIDIControlTransform {
    const ENCODING: Encoding = Encoding::Struct(
        "MIDIControlTransform",
        &[
            <MIDITransformControlType>::ENCODING,
            <MIDITransformControlType>::ENCODING,
            <u16>::ENCODING,
            <MIDITransformType>::ENCODING,
            <i16>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MIDIControlTransform {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes a source or destination in a MIDIThruConnection.
///
/// When creating one of these, you can leave uniqueID 0 if the endpoint exists and you are passing
/// its MIDIEndpointRef.
///
/// When obtaining one of these from CoreMIDI, endpointRef may be NULL if it doesn't exist, but the
/// uniqueID will always be non-zero.
///
/// Field: endpointRef     The endpoint specified as a MIDIEndpointRef.
/// Field: uniqueID        The endpoint specified by its uniqueID.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremidi/midithruconnectionendpoint?language=objc)
#[cfg(feature = "MIDIServices")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDIThruConnectionEndpoint {
    pub endpointRef: MIDIEndpointRef,
    pub uniqueID: MIDIUniqueID,
}

#[cfg(all(feature = "MIDIServices", feature = "objc2"))]
unsafe impl Encode for MIDIThruConnectionEndpoint {
    const ENCODING: Encoding = Encoding::Struct(
        "MIDIThruConnectionEndpoint",
        &[<MIDIEndpointRef>::ENCODING, <MIDIUniqueID>::ENCODING],
    );
}

#[cfg(all(feature = "MIDIServices", feature = "objc2"))]
unsafe impl RefEncode for MIDIThruConnectionEndpoint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes a set of MIDI routings and transformations.
///
/// The remainder of the structure is variably-sized. It contains numControlTransform instances of
/// MIDIControlTransform, followed by numMaps instances of MIDIValueMap.
///
/// Field: version     Version of this structure; must be 0.
/// Field: numSources  The number of valid sources in the following array.
/// Field: sources     All MIDI generated by these sources is routed into this connection for processing
/// and distribution to destinations.
/// Field: numDestinations The number of valid destinations in the following array.
/// Field: destinations    All MIDI output from the connection is routed to these destinations.
/// Field: channelMap      Maps each of the source 16 MIDI channels to channel 0-15 (1-16) or 0xFF when
/// MIDI from a channel is to be filtered out.
/// Field: lowVelocity     Note events with a velocity less than this value are filtered out.
/// Field: highVelocity    Note events with a velocity greater than this, if it is not 0, are filtered out.
/// Field: lowNote         See highNote.
/// Field: highNote        If highNote >= lowNote, then notes outside this range are filtered out.
/// If lowNote > highNote, then notes
/// <i>
/// inside
/// </i>
/// this range are filtered out.
/// This applies to note and polyphonic key pressure events.
/// These fields are ignored if a there is a MIDIValueMap applying to noteNumber.
/// Field: noteNumber      Specifies how MIDI note numbers are transformed.
/// Field: velocity        Specifies how MIDI note velocities are transformed.
/// Field: keyPressure     Specifies how MIDI polyphonic key pressure events are transformed.
/// Field: channelPressure Specifies how MIDI monophonic (channel) pressure events are transformed.
/// Field: programChange   Specifies how MIDI program change events are transformed.
/// Field: pitchBend       Specifies how MIDI pitch bend events are transformed.
/// Field: filterOutSysEx  If 1, specifies that system-exclusive messages are to be filtered out.
/// Field: filterOutMTC    If 1, specifies that MIDI Time Code messages are to be filtered out.
/// Field: filterOutBeatClock  If 1, specifies the MIDI clock, play, stop, and resume messages are to
/// be filtered out.
/// Field: filterOutTuneRequest    If 1, specifies that MIDI Tune Request messages are to be filtered out.
/// Field: reserved2       Must be 0.
/// Field: filterOutAllControls    If 1, specifies that all MIDI continuous control messages are to be filtered out.
/// Field: numControlTransforms    The number of control transformations in the variable-length portion of the struct.
/// Field: numMaps                 The number of MIDIValueMaps in the variable-length portion of the struct.
/// Field: reserved3       Must be 0.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremidi/midithruconnectionparams?language=objc)
#[cfg(feature = "MIDIServices")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDIThruConnectionParams {
    pub version: u32,
    pub numSources: u32,
    pub sources: [MIDIThruConnectionEndpoint; 8],
    pub numDestinations: u32,
    pub destinations: [MIDIThruConnectionEndpoint; 8],
    pub channelMap: [u8; 16],
    pub lowVelocity: u8,
    pub highVelocity: u8,
    pub lowNote: u8,
    pub highNote: u8,
    pub noteNumber: MIDITransform,
    pub velocity: MIDITransform,
    pub keyPressure: MIDITransform,
    pub channelPressure: MIDITransform,
    pub programChange: MIDITransform,
    pub pitchBend: MIDITransform,
    pub filterOutSysEx: u8,
    pub filterOutMTC: u8,
    pub filterOutBeatClock: u8,
    pub filterOutTuneRequest: u8,
    pub reserved2: [u8; 3],
    pub filterOutAllControls: u8,
    pub numControlTransforms: u16,
    pub numMaps: u16,
    pub reserved3: [u16; 4],
}

#[cfg(all(feature = "MIDIServices", feature = "objc2"))]
unsafe impl Encode for MIDIThruConnectionParams {
    const ENCODING: Encoding = Encoding::Struct(
        "MIDIThruConnectionParams",
        &[
            <u32>::ENCODING,
            <u32>::ENCODING,
            <[MIDIThruConnectionEndpoint; 8]>::ENCODING,
            <u32>::ENCODING,
            <[MIDIThruConnectionEndpoint; 8]>::ENCODING,
            <[u8; 16]>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <MIDITransform>::ENCODING,
            <MIDITransform>::ENCODING,
            <MIDITransform>::ENCODING,
            <MIDITransform>::ENCODING,
            <MIDITransform>::ENCODING,
            <MIDITransform>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <[u8; 3]>::ENCODING,
            <u8>::ENCODING,
            <u16>::ENCODING,
            <u16>::ENCODING,
            <[u16; 4]>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "MIDIServices", feature = "objc2"))]
unsafe impl RefEncode for MIDIThruConnectionParams {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "MIDIServices")]
impl MIDIThruConnectionParams {
    // TODO: pub fn MIDIThruConnectionParamsSize(ptr: NonNull<MIDIThruConnectionParams>,) -> usize;

    /// Fills a MIDIThruConnectionParams with default values.
    ///
    /// Parameter `inConnectionParams`: The struct to be initialized.
    ///
    /// This convenience function fills the connection structure with default values: no endpoints,
    /// no transformations (mostly zeroes except for the channel map). Then, just filling in the
    /// source and adding one destination will create a simple, unmodified thru connection.
    #[doc(alias = "MIDIThruConnectionParamsInitialize")]
    #[cfg(feature = "MIDIServices")]
    #[inline]
    pub unsafe fn initialize(in_connection_params: NonNull<MIDIThruConnectionParams>) {
        extern "C-unwind" {
            fn MIDIThruConnectionParamsInitialize(
                in_connection_params: NonNull<MIDIThruConnectionParams>,
            );
        }
        unsafe { MIDIThruConnectionParamsInitialize(in_connection_params) }
    }
}

extern "C-unwind" {
    /// Creates a thru connection.
    ///
    /// Parameter `inPersistentOwnerID`: If null, then the connection is marked as owned by the client
    /// and will be automatically disposed with the client.  if it is non-null, then it
    /// should be a unique identifier, e.g. "com.mycompany.MyCoolProgram".
    ///
    /// Parameter `inConnectionParams`: A MIDIThruConnectionParams contained in a CFDataRef.
    ///
    /// Parameter `outConnection`: On successful return, a reference to the newly-created connection.
    ///
    /// Returns: An OSStatus result code.
    #[cfg(all(feature = "MIDIServices", feature = "objc2-core-foundation"))]
    pub fn MIDIThruConnectionCreate(
        in_persistent_owner_id: Option<&CFString>,
        in_connection_params: &CFData,
        out_connection: NonNull<MIDIThruConnectionRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Disposes a thru connection.
    ///
    /// Parameter `connection`: The connection to be disposed
    ///
    /// Returns: An OSStatus result code.
    #[cfg(feature = "MIDIServices")]
    pub fn MIDIThruConnectionDispose(connection: MIDIThruConnectionRef) -> OSStatus;
}

extern "C-unwind" {
    /// Obtains a thru connection's MIDIThruConnectionParams.
    ///
    /// Parameter `connection`: The connection to be disposed.
    ///
    /// Parameter `outConnectionParams`: On successful return, the connection's MIDIThruConnectionParams in a CFDataRef
    ///
    /// Returns: An OSStatus result code.
    ///
    /// The returned CFDataRef contains a MIDIThruConnectionParams structure. The caller is responsible
    /// for releasing it.
    #[cfg(all(feature = "MIDIServices", feature = "objc2-core-foundation"))]
    pub fn MIDIThruConnectionGetParams(
        connection: MIDIThruConnectionRef,
        out_connection_params: NonNull<NonNull<CFData>>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Alters a thru connection's MIDIThruConnectionParams.
    ///
    /// Parameter `connection`: The connection to be modified.
    ///
    /// Parameter `inConnectionParams`: The connection's new MIDIThruConnectionParams in a CFDataRef
    ///
    /// Returns: An OSStatus result code.
    #[cfg(all(feature = "MIDIServices", feature = "objc2-core-foundation"))]
    pub fn MIDIThruConnectionSetParams(
        connection: MIDIThruConnectionRef,
        in_connection_params: &CFData,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Returns all of the persistent thru connections created by a client.
    ///
    /// Parameter `inPersistentOwnerID`: The ID of the owner whose connections are to be returned.
    ///
    /// Parameter `outConnectionList`: On successful return, a CFDataRef containing an array of MIDIThruConnectionRef's.
    ///
    /// Returns: An OSStatus result code.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn MIDIThruConnectionFind(
        in_persistent_owner_id: &CFString,
        out_connection_list: NonNull<NonNull<CFData>>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "MIDIServices")]
    #[deprecated = "renamed to `MIDIThruConnectionParams::initialize`"]
    pub fn MIDIThruConnectionParamsInitialize(
        in_connection_params: NonNull<MIDIThruConnectionParams>,
    );
}
