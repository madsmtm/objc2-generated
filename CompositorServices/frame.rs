//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/compositorservices/cp_frame?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct cp_frame {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for cp_frame {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("cp_frame", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/compositorservices/cp_frame_t?language=objc)
pub type cp_frame_t = *mut cp_frame;

extern "C-unwind" {
    /// Returns the sequential index number of the specified frame.
    ///
    /// - Parameters:
    /// - frame: The frame to query.
    /// - Returns: The sequential index of the frame, which is always a
    /// positive integer.
    ///
    /// The layer assigns a unique index number to each frame, starting at
    /// the first frame and incrementing the index by 1 for each new frame.
    #[cfg(feature = "cp_types")]
    pub fn cp_frame_get_frame_index(frame: cp_frame_t) -> cp_layer_frame_index_t;
}

extern "C-unwind" {
    /// Computes and returns the predicted timing information for the specified frame.
    ///
    /// - Parameters:
    /// - frame: The frame you're preparing to draw.
    /// - Returns: The predicted timing information for the specified frame, or `NULL`
    /// if the layer is in the ``cp_layer_renderer/cp_layer_renderer_state_paused`` or
    /// ``cp_layer_renderer/cp_layer_renderer_state_invalidated`` state.
    ///
    /// Use the returned timing information in any functions that return
    /// frame-related deadlines. For example, pass the timing information
    /// to the ``cp_frame_timing_get_optimal_input_time`` function to
    /// determine when to start encoding the frame. This function updates
    /// the frame-specific timing information using the latest data from
    /// the compositor before it returns it.
    ///
    /// Don’t call this function after you call ``cp_frame_query_drawable``
    /// for the specified frame. If you need the timing information after you
    /// retrieve the frame's ``cp_drawable_t`` type, save the return value
    /// of this function, or call ``cp_drawable_get_frame_timing`` to get
    /// the information from the drawable instead.
    #[cfg(feature = "frame_timing")]
    pub fn cp_frame_predict_timing(frame: cp_frame_t) -> cp_frame_timing_t;
}

extern "C-unwind" {
    /// Returns the drawable type you use to retrieve the textures and
    /// drawing environment for the frame.
    ///
    /// - Parameters:
    /// - frame: The frame to query.
    /// - Returns: The drawable type, or `NULL` if the layer is in the
    /// ``cp_layer_renderer/cp_layer_renderer_state_paused`` or
    /// ``cp_layer_renderer/cp_layer_renderer_state_invalidated`` state.
    ///
    /// Call this function when you're ready to encode the drawing commands
    /// for the frame. The ``cp_drawable_t`` type contains the textures and
    /// other information you need to set up your render descriptor in Metal.
    ///
    /// Note: This function isn't safe to be called concurrently. Always ensure a
    /// single thread call this function at a time.
    #[cfg(feature = "drawable")]
    pub fn cp_frame_query_drawable(frame: cp_frame_t) -> cp_drawable_t;
}

extern "C-unwind" {
    /// Notifies the compositor that you started updating the app-specific
    /// content you need to render the frame.
    ///
    /// - Parameters:
    /// - frame: The frame you are ready to prepare.
    ///
    /// This function helps you optimize your app’s rendering efficiency.
    /// Before you render a frame, you might need to respond to
    /// interactions and update your app's data structures before you can
    /// render items in your scene.  Call this function immediately
    /// before you start that work, and call the ``cp_frame_end_update``
    /// function as soon as you finish. The compositor uses the time difference
    /// to improve its predictions for when to start the frame encoding process.
    ///
    /// Move as much work as possible into the update phase to minimize encoding
    /// time. Don't do any work that relies on the current pose information during
    /// the update phase. Instead, make any pose-related changes during the
    /// encoding phase.
    pub fn cp_frame_start_update(frame: cp_frame_t);
}

extern "C-unwind" {
    /// Notifies the compositor that you finished updating the app-specific
    /// content you need to render the frame.
    ///
    /// - Parameters:
    /// - frame: The frame you finished preparing.
    ///
    /// This function helps you optimize your app’s rendering efficiency.
    /// Before you render a frame, you might need to respond to
    /// interactions and update your app's data structures before you can
    /// render items in your scene. Call the ``cp_frame_start_update``
    /// function immediately before you start that work, and call this function
    /// as soon as you finish. Compositor uses the frame update time to improve
    /// its predictions for when to start the frame encoding process.
    ///
    /// Move as much work as possible into the update phase to minimize encoding
    /// time. Don't do any work that relies on the current pose information during
    /// the update phase. Instead, make any pose-related changes during the
    /// encoding phase.
    pub fn cp_frame_end_update(frame: cp_frame_t);
}

extern "C-unwind" {
    /// Notifies the compositor that you're ready to generate the
    /// GPU commands to render the specified frame.
    ///
    /// - Parameters:
    /// - frame: The frame you're ready to encode and send to the GPU.
    ///
    /// This function helps you optimize your app’s rendering efficiency.
    /// Call this function function before you start any of the GPU work that depends
    /// on the input data. Call the ``cp_frame_end_submission`` function after
    /// you finish your drawing commands and are ready to commit the frame to
    /// the GPU. Compositor uses the time difference to improve its predictions
    /// for when to start the frame submission process. Those predictions help
    /// you schedule the encoding process at a more optimal time for the
    /// system.
    pub fn cp_frame_start_submission(frame: cp_frame_t);
}

extern "C-unwind" {
    /// Notifies the compositor that you finished generating the GPU
    /// commands to render the specified frame.
    ///
    /// - Parameters:
    /// - frame: The frame you're ready to submit to the GPU.
    ///
    /// This function helps you optimize your app’s rendering efficiency.
    /// Call the ``cp_frame_start_submission`` function before you start any of the GPU
    /// work that depends on the input data. Call this function after
    /// you finish your drawing commands and are ready to commit the frame to
    /// the GPU. Compositor uses the time difference to improve its predictions
    /// for when to start the frame submission process. Those predictions help
    /// you schedule the encoding process at a more optimal time for the
    /// system.
    pub fn cp_frame_end_submission(frame: cp_frame_t);
}