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
    /// [Apple's documentation](https://developer.apple.com/documentation/arkit/arpointcloud?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2")]
    pub struct ARPointCloud;
);

#[cfg(feature = "objc2")]
unsafe impl Send for ARPointCloud {}

#[cfg(feature = "objc2")]
unsafe impl Sync for ARPointCloud {}

#[cfg(all(feature = "objc2", feature = "objc2-foundation"))]
unsafe impl NSCoding for ARPointCloud {}

#[cfg(feature = "objc2")]
unsafe impl NSObjectProtocol for ARPointCloud {}

#[cfg(all(feature = "objc2", feature = "objc2-foundation"))]
unsafe impl NSSecureCoding for ARPointCloud {}

#[cfg(feature = "objc2")]
extern_methods!(
    #[cfg(feature = "objc2")]
    unsafe impl ARPointCloud {
        /// The number of points in the point cloud.
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        /// The 3D point identifiers comprising the point cloud.
        #[method(identifiers)]
        pub unsafe fn identifiers(&self) -> NonNull<u64>;

        /// Unavailable
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
