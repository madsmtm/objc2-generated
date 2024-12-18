//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbuffernoerr?language=objc)
pub const kCMBlockBufferNoErr: OSStatus = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferstructureallocationfailederr?language=objc)
pub const kCMBlockBufferStructureAllocationFailedErr: OSStatus = -12700;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferblockallocationfailederr?language=objc)
pub const kCMBlockBufferBlockAllocationFailedErr: OSStatus = -12701;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferbadcustomblocksourceerr?language=objc)
pub const kCMBlockBufferBadCustomBlockSourceErr: OSStatus = -12702;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferbadoffsetparametererr?language=objc)
pub const kCMBlockBufferBadOffsetParameterErr: OSStatus = -12703;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferbadlengthparametererr?language=objc)
pub const kCMBlockBufferBadLengthParameterErr: OSStatus = -12704;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferbadpointerparametererr?language=objc)
pub const kCMBlockBufferBadPointerParameterErr: OSStatus = -12705;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferemptybbuferr?language=objc)
pub const kCMBlockBufferEmptyBBufErr: OSStatus = -12706;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferunallocatedblockerr?language=objc)
pub const kCMBlockBufferUnallocatedBlockErr: OSStatus = -12707;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferinsufficientspaceerr?language=objc)
pub const kCMBlockBufferInsufficientSpaceErr: OSStatus = -12708;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmblockbufferflags?language=objc)
pub type CMBlockBufferFlags = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferassurememorynowflag?language=objc)
pub const kCMBlockBufferAssureMemoryNowFlag: CMBlockBufferFlags = 1 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferalwayscopydataflag?language=objc)
pub const kCMBlockBufferAlwaysCopyDataFlag: CMBlockBufferFlags = 1 << 1;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferdontoptimizedepthflag?language=objc)
pub const kCMBlockBufferDontOptimizeDepthFlag: CMBlockBufferFlags = 1 << 2;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbufferpermitemptyreferenceflag?language=objc)
pub const kCMBlockBufferPermitEmptyReferenceFlag: CMBlockBufferFlags = 1 << 3;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmblockbufferref?language=objc)
pub type CMBlockBufferRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmblockbuffercustomblocksource?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMBlockBufferCustomBlockSource {
    pub version: u32,
    pub AllocateBlock: Option<unsafe extern "C-unwind" fn(*mut c_void, usize) -> *mut c_void>,
    pub FreeBlock: Option<unsafe extern "C-unwind" fn(*mut c_void, NonNull<c_void>, usize)>,
    pub refCon: *mut c_void,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CMBlockBufferCustomBlockSource {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u32>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(*mut c_void, usize) -> *mut c_void>>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(*mut c_void, NonNull<c_void>, usize)>>::ENCODING,
            <*mut c_void>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CMBlockBufferCustomBlockSource {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmblockbuffercustomblocksourceversion?language=objc)
pub const kCMBlockBufferCustomBlockSourceVersion: u32 = 0;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMBlockBufferCreateEmpty(
        structure_allocator: CFAllocatorRef,
        sub_block_capacity: u32,
        flags: CMBlockBufferFlags,
        block_buffer_out: NonNull<CMBlockBufferRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMBlockBufferCreateWithMemoryBlock(
        structure_allocator: CFAllocatorRef,
        memory_block: *mut c_void,
        block_length: usize,
        block_allocator: CFAllocatorRef,
        custom_block_source: *mut CMBlockBufferCustomBlockSource,
        offset_to_data: usize,
        data_length: usize,
        flags: CMBlockBufferFlags,
        block_buffer_out: NonNull<CMBlockBufferRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMBlockBufferCreateWithBufferReference(
        structure_allocator: CFAllocatorRef,
        buffer_reference: CMBlockBufferRef,
        offset_to_data: usize,
        data_length: usize,
        flags: CMBlockBufferFlags,
        block_buffer_out: NonNull<CMBlockBufferRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMBlockBufferCreateContiguous(
        structure_allocator: CFAllocatorRef,
        source_buffer: CMBlockBufferRef,
        block_allocator: CFAllocatorRef,
        custom_block_source: *mut CMBlockBufferCustomBlockSource,
        offset_to_data: usize,
        data_length: usize,
        flags: CMBlockBufferFlags,
        block_buffer_out: NonNull<CMBlockBufferRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMBlockBufferGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMBlockBufferAppendMemoryBlock(
        the_buffer: CMBlockBufferRef,
        memory_block: *mut c_void,
        block_length: usize,
        block_allocator: CFAllocatorRef,
        custom_block_source: *mut CMBlockBufferCustomBlockSource,
        offset_to_data: usize,
        data_length: usize,
        flags: CMBlockBufferFlags,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMBlockBufferAppendBufferReference(
        the_buffer: CMBlockBufferRef,
        target_b_buf: CMBlockBufferRef,
        offset_to_data: usize,
        data_length: usize,
        flags: CMBlockBufferFlags,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMBlockBufferAssureBlockMemory(the_buffer: CMBlockBufferRef) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMBlockBufferAccessDataBytes(
        the_buffer: CMBlockBufferRef,
        offset: usize,
        length: usize,
        temporary_block: NonNull<c_void>,
        returned_pointer_out: NonNull<*mut c_char>,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMBlockBufferCopyDataBytes(
        the_source_buffer: CMBlockBufferRef,
        offset_to_data: usize,
        data_length: usize,
        destination: NonNull<c_void>,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMBlockBufferReplaceDataBytes(
        source_bytes: NonNull<c_void>,
        destination_buffer: CMBlockBufferRef,
        offset_into_destination: usize,
        data_length: usize,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMBlockBufferFillDataBytes(
        fill_byte: c_char,
        destination_buffer: CMBlockBufferRef,
        offset_into_destination: usize,
        data_length: usize,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMBlockBufferGetDataPointer(
        the_buffer: CMBlockBufferRef,
        offset: usize,
        length_at_offset_out: *mut usize,
        total_length_out: *mut usize,
        data_pointer_out: *mut *mut c_char,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn CMBlockBufferGetDataLength(the_buffer: CMBlockBufferRef) -> usize;
}

extern "C-unwind" {
    pub fn CMBlockBufferIsRangeContiguous(
        the_buffer: CMBlockBufferRef,
        offset: usize,
        length: usize,
    ) -> Boolean;
}

extern "C-unwind" {
    pub fn CMBlockBufferIsEmpty(the_buffer: CMBlockBufferRef) -> Boolean;
}
