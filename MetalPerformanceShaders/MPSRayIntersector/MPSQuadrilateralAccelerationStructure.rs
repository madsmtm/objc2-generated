//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsquadrilateralaccelerationstructure?language=objc)
    #[unsafe(super(
        MPSPolygonAccelerationStructure,
        MPSAccelerationStructure,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "MPSAccelerationStructure",
        feature = "MPSKernel",
        feature = "MPSPolygonAccelerationStructure"
    ))]
    #[deprecated]
    pub struct MPSQuadrilateralAccelerationStructure;
);

#[cfg(all(
    feature = "MPSAccelerationStructure",
    feature = "MPSKernel",
    feature = "MPSPolygonAccelerationStructure"
))]
unsafe impl NSCoding for MPSQuadrilateralAccelerationStructure {}

#[cfg(all(
    feature = "MPSAccelerationStructure",
    feature = "MPSKernel",
    feature = "MPSPolygonAccelerationStructure"
))]
unsafe impl NSCopying for MPSQuadrilateralAccelerationStructure {}

#[cfg(all(
    feature = "MPSAccelerationStructure",
    feature = "MPSKernel",
    feature = "MPSPolygonAccelerationStructure"
))]
unsafe impl CopyingHelper for MPSQuadrilateralAccelerationStructure {
    type Result = Self;
}

#[cfg(all(
    feature = "MPSAccelerationStructure",
    feature = "MPSKernel",
    feature = "MPSPolygonAccelerationStructure"
))]
unsafe impl NSObjectProtocol for MPSQuadrilateralAccelerationStructure {}

#[cfg(all(
    feature = "MPSAccelerationStructure",
    feature = "MPSKernel",
    feature = "MPSPolygonAccelerationStructure"
))]
unsafe impl NSSecureCoding for MPSQuadrilateralAccelerationStructure {}

extern_methods!(
    #[cfg(all(
        feature = "MPSAccelerationStructure",
        feature = "MPSKernel",
        feature = "MPSPolygonAccelerationStructure"
    ))]
    unsafe impl MPSQuadrilateralAccelerationStructure {
        #[deprecated]
        #[method(quadrilateralCount)]
        pub unsafe fn quadrilateralCount(&self) -> NSUInteger;

        #[deprecated]
        #[method(setQuadrilateralCount:)]
        pub unsafe fn setQuadrilateralCount(&self, quadrilateral_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSAccelerationStructure`
    #[cfg(all(
        feature = "MPSAccelerationStructure",
        feature = "MPSKernel",
        feature = "MPSPolygonAccelerationStructure"
    ))]
    unsafe impl MPSQuadrilateralAccelerationStructure {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MPSAccelerationStructureGroup")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithGroup:)]
        pub unsafe fn initWithGroup(
            this: Allocated<Self>,
            group: &MPSAccelerationStructureGroup,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSAccelerationStructureGroup")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCoder:group:)]
        pub unsafe fn initWithCoder_group(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            group: &MPSAccelerationStructureGroup,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(
        feature = "MPSAccelerationStructure",
        feature = "MPSKernel",
        feature = "MPSPolygonAccelerationStructure"
    ))]
    unsafe impl MPSQuadrilateralAccelerationStructure {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "MPSAccelerationStructure",
        feature = "MPSKernel",
        feature = "MPSPolygonAccelerationStructure"
    ))]
    unsafe impl MPSQuadrilateralAccelerationStructure {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
