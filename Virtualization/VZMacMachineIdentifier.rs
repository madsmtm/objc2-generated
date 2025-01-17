//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An identifier to make a virtual machine unique.
    ///
    /// The Mac machine identifier is used by macOS guests to uniquely identify the virtual hardware.
    ///
    /// Two virtual machines running concurrently should not use the same identifier.
    ///
    /// If the virtual machine is serialized to disk, the identifier can be preserved in a binary representation through VZMacMachineIdentifier.dataRepresentation.
    /// The identifier can then be recreated with -[VZMacMachineIdentifier initWithDataRepresentation:] from the binary representation.
    ///
    /// The contents of two identifiers can be compared with -[VZMacMachineIdentifier isEqual:].
    ///
    /// See also: VZMacPlatformConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmacmachineidentifier?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMacMachineIdentifier;
);

unsafe impl NSCopying for VZMacMachineIdentifier {}

unsafe impl CopyingHelper for VZMacMachineIdentifier {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZMacMachineIdentifier {}

extern_methods!(
    unsafe impl VZMacMachineIdentifier {
        /// Create a new unique machine identifier.
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Get the machine identifier described by the specified data representation.
        ///
        /// Parameter `dataRepresentation`: The opaque data representation of the machine identifier to be obtained.
        ///
        /// Returns: A unique identifier identical to the one that generated the dataRepresentation, or nil if the data is invalid.
        ///
        /// See: VZMacMachineIdentifier.dataRepresentation
        #[unsafe(method_family(init))]
        #[method_id(initWithDataRepresentation:)]
        pub unsafe fn initWithDataRepresentation(
            this: Allocated<Self>,
            data_representation: &NSData,
        ) -> Option<Retained<Self>>;

        /// Opaque data representation of the machine identifier.
        ///
        /// This can be used to recreate the same machine identifier with -[VZMacMachineIdentifier initWithDataRepresentation:].
        ///
        /// See: -[VZMacMachineIdentifier initWithDataRepresentation:]
        #[unsafe(method_family(none))]
        #[method_id(dataRepresentation)]
        pub unsafe fn dataRepresentation(&self) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VZMacMachineIdentifier {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
