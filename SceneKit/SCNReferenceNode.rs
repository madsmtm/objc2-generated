//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Controls whenever to load the reference node.
///
/// When the load policy is set to SCNReferenceLoadingPolicyImmediately, the reference is loaded immediately when the SCNReferenceNode is unarchived.
/// When the load policy is set to SCNReferenceLoadingPolicyOnDemand the reference is never loaded until "load" is explicitly invoked.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnreferenceloadingpolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCNReferenceLoadingPolicy(pub NSInteger);
impl SCNReferenceLoadingPolicy {
    #[doc(alias = "SCNReferenceLoadingPolicyImmediate")]
    pub const Immediate: Self = Self(0);
    #[doc(alias = "SCNReferenceLoadingPolicyOnDemand")]
    pub const OnDemand: Self = Self(1);
}

unsafe impl Encode for SCNReferenceLoadingPolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCNReferenceLoadingPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// Node that references an external file.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnreferencenode?language=objc)
    #[unsafe(super(SCNNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "SCNNode")]
    pub struct SCNReferenceNode;
);

#[cfg(feature = "SCNNode")]
extern_conformance!(
    unsafe impl NSCoding for SCNReferenceNode {}
);

#[cfg(feature = "SCNNode")]
extern_conformance!(
    unsafe impl NSCopying for SCNReferenceNode {}
);

#[cfg(feature = "SCNNode")]
unsafe impl CopyingHelper for SCNReferenceNode {
    type Result = Self;
}

#[cfg(feature = "SCNNode")]
extern_conformance!(
    unsafe impl NSObjectProtocol for SCNReferenceNode {}
);

#[cfg(feature = "SCNNode")]
extern_conformance!(
    unsafe impl NSSecureCoding for SCNReferenceNode {}
);

#[cfg(all(feature = "SCNAction", feature = "SCNNode"))]
extern_conformance!(
    unsafe impl SCNActionable for SCNReferenceNode {}
);

#[cfg(all(feature = "SCNAnimation", feature = "SCNNode"))]
extern_conformance!(
    unsafe impl SCNAnimatable for SCNReferenceNode {}
);

#[cfg(all(feature = "SCNBoundingVolume", feature = "SCNNode"))]
extern_conformance!(
    unsafe impl SCNBoundingVolume for SCNReferenceNode {}
);

#[cfg(feature = "SCNNode")]
impl SCNReferenceNode {
    extern_methods!(
        /// Creates a reference node with a url.
        #[unsafe(method(initWithURL:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithURL(
            this: Allocated<Self>,
            reference_url: &NSURL,
        ) -> Option<Retained<Self>>;

        /// Support coding and decoding via NSKeyedArchiver.
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        /// Creates a reference node with a url.
        #[unsafe(method(referenceNodeWithURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn referenceNodeWithURL(reference_url: &NSURL) -> Option<Retained<Self>>;

        /// Specifies the url to resolve.
        #[unsafe(method(referenceURL))]
        #[unsafe(method_family = none)]
        pub unsafe fn referenceURL(&self) -> Retained<NSURL>;

        /// Setter for [`referenceURL`][Self::referenceURL].
        #[unsafe(method(setReferenceURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setReferenceURL(&self, reference_url: &NSURL);

        /// Specifies when to load the reference. see SCNReferenceLoadingPolicy above. Defaults to SCNReferenceLoadingPolicyImmediately.
        #[unsafe(method(loadingPolicy))]
        #[unsafe(method_family = none)]
        pub unsafe fn loadingPolicy(&self) -> SCNReferenceLoadingPolicy;

        /// Setter for [`loadingPolicy`][Self::loadingPolicy].
        #[unsafe(method(setLoadingPolicy:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLoadingPolicy(&self, loading_policy: SCNReferenceLoadingPolicy);

        /// Force the reference to be loaded if it hasn't been loaded already. The resolved nodes will be added
        /// as child nodes of the receiver.
        #[unsafe(method(load))]
        #[unsafe(method_family = none)]
        pub unsafe fn load(&self);

        /// Remove the child nodes and mark as unloaded.
        #[unsafe(method(unload))]
        #[unsafe(method_family = none)]
        pub unsafe fn unload(&self);

        /// Indicates whether the referenced URL has been loaded.
        #[unsafe(method(isLoaded))]
        #[unsafe(method_family = none)]
        pub unsafe fn isLoaded(&self) -> bool;
    );
}

/// Methods declared on superclass `SCNNode`.
#[cfg(feature = "SCNNode")]
impl SCNReferenceNode {
    extern_methods!(
        /// Creates and initializes a node instance.
        #[unsafe(method(node))]
        #[unsafe(method_family = none)]
        pub unsafe fn node() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "SCNNode")]
impl SCNReferenceNode {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
