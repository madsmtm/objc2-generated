//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// The MPSImageStatisticsMinAndMax computes the minimum and maximum pixel values for a given region of an image.
    /// The min and max values are written to the destination image at the following pixel locations:
    /// - min value is written at pixel location (0, 0)
    /// - max value is written at pixel location (1, 0)
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagestatisticsminandmax?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageStatisticsMinAndMax;
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageStatisticsMinAndMax {}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageStatisticsMinAndMax {}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageStatisticsMinAndMax {
    type Result = Self;
}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageStatisticsMinAndMax {}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageStatisticsMinAndMax {}

extern_methods!(
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageStatisticsMinAndMax {
        /// The source rectangle to use when reading data.
        ///
        /// A MTLRegion that indicates which part of the source to read. If the clipRectSource does not lie
        /// completely within the source image, the intersection of the image bounds and clipRectSource will
        /// be used. The clipRectSource replaces the MPSUnaryImageKernel offset parameter for this filter.
        /// The latter is ignored.   Default: MPSRectNoClip, use the entire source texture.
        ///
        /// The clipRect specified in MPSUnaryImageKernel is used to control the origin in the destination texture
        /// where the min, max values are written.  The clipRect.width must be >=2.  The clipRect.height must be >= 1.
        #[method(clipRectSource)]
        pub unsafe fn clipRectSource(&self) -> MTLRegion;

        /// Setter for [`clipRectSource`][Self::clipRectSource].
        #[method(setClipRectSource:)]
        pub unsafe fn setClipRectSource(&self, clip_rect_source: MTLRegion);

        /// Specifies information to apply the statistics min-max operation on an image.
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Returns: A valid MPSImageStatisticsMinAndMax object or nil, if failure.
        #[unsafe(method_family(init))]
        #[method_id(initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageStatisticsMinAndMax {
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageStatisticsMinAndMax {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// The MPSImageStatisticsMeanAndVariance computes the mean and variance for a given region of an image.
    /// The mean and variance values are written to the destination image at the following pixel locations:
    /// - mean value is written at pixel location (0, 0)
    /// - variance value is written at pixel location (1, 0)
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagestatisticsmeanandvariance?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageStatisticsMeanAndVariance;
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageStatisticsMeanAndVariance {}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageStatisticsMeanAndVariance {}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageStatisticsMeanAndVariance {
    type Result = Self;
}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageStatisticsMeanAndVariance {}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageStatisticsMeanAndVariance {}

extern_methods!(
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageStatisticsMeanAndVariance {
        /// The source rectangle to use when reading data.
        ///
        /// A MTLRegion that indicates which part of the source to read. If the clipRectSource does not lie
        /// completely within the source image, the intersection of the image bounds and clipRectSource will
        /// be used. The clipRectSource replaces the MPSUnaryImageKernel offset parameter for this filter.
        /// The latter is ignored.   Default: MPSRectNoClip, use the entire source texture.
        ///
        /// The clipRect specified in MPSUnaryImageKernel is used to control the origin in the destination texture
        /// where the mean value is written.
        #[method(clipRectSource)]
        pub unsafe fn clipRectSource(&self) -> MTLRegion;

        /// Setter for [`clipRectSource`][Self::clipRectSource].
        #[method(setClipRectSource:)]
        pub unsafe fn setClipRectSource(&self, clip_rect_source: MTLRegion);

        /// Specifies information to apply the statistics mean operation on an image.
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Returns: A valid MPSImageStatisticsMeanAndVariance object or nil, if failure.
        #[unsafe(method_family(init))]
        #[method_id(initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageStatisticsMeanAndVariance {
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageStatisticsMeanAndVariance {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// The MPSImageStatisticsMean computes the mean for a given region of an image.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagestatisticsmean?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageStatisticsMean;
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageStatisticsMean {}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageStatisticsMean {}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageStatisticsMean {
    type Result = Self;
}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageStatisticsMean {}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageStatisticsMean {}

extern_methods!(
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageStatisticsMean {
        /// The source rectangle to use when reading data.
        ///
        /// A MTLRegion that indicates which part of the source to read. If the clipRectSource does not lie
        /// completely within the source image, the intersection of the image bounds and clipRectSource will
        /// be used. The clipRectSource replaces the MPSUnaryImageKernel offset parameter for this filter.
        /// The latter is ignored.   Default: MPSRectNoClip, use the entire source texture.
        ///
        /// The clipRect specified in MPSUnaryImageKernel is used to control the origin in the destination texture
        /// where the mean value is written.
        #[method(clipRectSource)]
        pub unsafe fn clipRectSource(&self) -> MTLRegion;

        /// Setter for [`clipRectSource`][Self::clipRectSource].
        #[method(setClipRectSource:)]
        pub unsafe fn setClipRectSource(&self, clip_rect_source: MTLRegion);

        /// Specifies information to apply the statistics mean operation on an image.
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Returns: A valid MPSImageStatisticsMean object or nil, if failure.
        #[unsafe(method_family(init))]
        #[method_id(initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageStatisticsMean {
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageStatisticsMean {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
