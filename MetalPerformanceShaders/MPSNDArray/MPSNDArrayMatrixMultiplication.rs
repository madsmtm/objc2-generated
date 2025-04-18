//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// Dependencies: This depends on Metal.framework.
    ///
    ///
    /// A matrix multiplication kernel operating on MPSNDArray objects.
    ///
    ///
    /// A MPSNDArrayMatrixMultiplication object computes, for each 2-D matrix within
    /// a 4-D MPSNDArray object:
    ///
    /// D = alpha * A * B + beta * C
    ///
    /// A, B, C, and D are matrices which are represented by objects stored
    /// in the two most major dimensions of the MPSNDArray. alpha and beta
    /// are scalar values (of the same data type as values of D and C) which
    /// are applied as shown above.
    ///
    /// If an input's 3rd or 4th dimension is 1 its data will be broadcast as
    /// appropriate to the remaining input's 3rd or 4th dimension respectively.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsndarraymatrixmultiplication?language=objc)
    #[unsafe(super(MPSNDArrayMultiaryKernel, MPSNDArrayMultiaryBase, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "MPSCore",
        feature = "MPSKernel",
        feature = "MPSNDArrayKernel"
    ))]
    pub struct MPSNDArrayMatrixMultiplication;
);

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
extern_conformance!(
    unsafe impl NSCoding for MPSNDArrayMatrixMultiplication {}
);

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
extern_conformance!(
    unsafe impl NSCopying for MPSNDArrayMatrixMultiplication {}
);

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
unsafe impl CopyingHelper for MPSNDArrayMatrixMultiplication {
    type Result = Self;
}

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSNDArrayMatrixMultiplication {}
);

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
extern_conformance!(
    unsafe impl NSSecureCoding for MPSNDArrayMatrixMultiplication {}
);

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
impl MPSNDArrayMatrixMultiplication {
    extern_methods!(
        /// The scale factor to apply to the product.  Specified in double
        /// precision.  Will be converted to the appropriate precision in the
        /// implementation subject to rounding and/or clamping as necessary.
        /// Defaults to 1.0 at initialization time.
        #[unsafe(method(alpha))]
        #[unsafe(method_family = none)]
        pub unsafe fn alpha(&self) -> c_double;

        /// Setter for [`alpha`][Self::alpha].
        #[unsafe(method(setAlpha:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAlpha(&self, alpha: c_double);

        /// The scale factor to apply to the addend if available.  Specified in double
        /// precision.  Will be converted to the appropriate precision in the
        /// implementation subject to rounding and/or clamping as necessary.
        /// Defaults to 1.0 at initialization time.
        #[unsafe(method(beta))]
        #[unsafe(method_family = none)]
        pub unsafe fn beta(&self) -> c_double;

        /// Setter for [`beta`][Self::beta].
        #[unsafe(method(setBeta:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBeta(&self, beta: c_double);
    );
}

/// Methods declared on superclass `MPSNDArrayMultiaryKernel`.
#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
impl MPSNDArrayMatrixMultiplication {
    extern_methods!(
        #[unsafe(method(initWithDevice:sourceCount:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice_sourceCount(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            count: NSUInteger,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:device:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            coder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `MPSNDArrayMultiaryBase`.
#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
impl MPSNDArrayMatrixMultiplication {
    extern_methods!(
        #[unsafe(method(initWithDevice:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `MPSKernel`.
#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
impl MPSNDArrayMatrixMultiplication {
    extern_methods!(
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
impl MPSNDArrayMatrixMultiplication {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
