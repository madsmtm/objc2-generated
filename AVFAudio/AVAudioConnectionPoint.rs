//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A representation of either a source or destination connection point in AVAudioEngine.
    ///
    /// AVAudioConnectionPoint describes either a source or destination connection point (node, bus)
    /// in AVAudioEngine's graph.
    ///
    /// Instances of this class are immutable.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioconnectionpoint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioConnectionPoint;
);

unsafe impl NSObjectProtocol for AVAudioConnectionPoint {}

extern_methods!(
    unsafe impl AVAudioConnectionPoint {
        #[cfg(all(feature = "AVAudioNode", feature = "AVAudioTypes"))]
        /// Create a connection point object.
        ///
        /// Parameter `node`: the source or destination node
        ///
        /// Parameter `bus`: the output or input bus on the node
        ///
        /// If the node is nil, this method fails (returns nil).
        #[method_id(@__retain_semantics Init initWithNode:bus:)]
        pub unsafe fn initWithNode_bus(
            this: Allocated<Self>,
            node: &AVAudioNode,
            bus: AVAudioNodeBus,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "AVAudioNode")]
        /// Returns the node in the connection point.
        #[method_id(@__retain_semantics Other node)]
        pub unsafe fn node(&self) -> Option<Retained<AVAudioNode>>;

        #[cfg(feature = "AVAudioTypes")]
        /// Returns the bus on the node in the connection point.
        #[method(bus)]
        pub unsafe fn bus(&self) -> AVAudioNodeBus;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioConnectionPoint {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);