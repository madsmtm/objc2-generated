//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caedrmetadata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAEDRMetadata;
);

extern_conformance!(
    unsafe impl NSCoding for CAEDRMetadata {}
);

extern_conformance!(
    unsafe impl NSCopying for CAEDRMetadata {}
);

unsafe impl CopyingHelper for CAEDRMetadata {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CAEDRMetadata {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CAEDRMetadata {}
);

impl CAEDRMetadata {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(HDR10MetadataWithDisplayInfo:contentInfo:opticalOutputScale:))]
        #[unsafe(method_family = none)]
        pub unsafe fn HDR10MetadataWithDisplayInfo_contentInfo_opticalOutputScale(
            display_data: Option<&NSData>,
            content_data: Option<&NSData>,
            scale: c_float,
        ) -> Retained<CAEDRMetadata>;

        #[unsafe(method(HDR10MetadataWithMinLuminance:maxLuminance:opticalOutputScale:))]
        #[unsafe(method_family = none)]
        pub unsafe fn HDR10MetadataWithMinLuminance_maxLuminance_opticalOutputScale(
            min_nits: c_float,
            max_nits: c_float,
            scale: c_float,
        ) -> Retained<CAEDRMetadata>;

        #[unsafe(method(HLGMetadataWithAmbientViewingEnvironment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn HLGMetadataWithAmbientViewingEnvironment(
            data: &NSData,
        ) -> Retained<CAEDRMetadata>;

        #[unsafe(method(HLGMetadata))]
        #[unsafe(method_family = none)]
        pub unsafe fn HLGMetadata() -> Retained<CAEDRMetadata>;

        #[unsafe(method(isAvailable))]
        #[unsafe(method_family = none)]
        pub unsafe fn isAvailable() -> bool;
    );
}
