//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MLState
    #[cfg(feature = "MLModel")]
    unsafe impl MLModel {
        #[cfg(feature = "MLState")]
        /// Creates a new state object.
        ///
        /// Core ML framework will allocate the state buffers declared in the model.
        ///
        /// The allocated state buffers are initialized to zeros. To initialize with different values, use `.withMultiArray(for:)` to get the mutable `MLMultiArray`-view to the state buffer.
        ///
        /// It returns an empty state when the model is stateless. One can use the empty state with stateful prediction functions such as `prediction(from:using:)` and those predictions will be stateless. This simplifies the call site which may or may not use a stateful model.
        ///
        /// ```swift
        /// // Create state that contains two state buffers: s1 and s2.
        /// // Then, initialize s1 to 1.0 and s2 to 2.0.
        /// let state = model.newState()
        /// state.withMultiArray(for: "s1") { stateMultiArray in
        /// stateMultiArray[0] = 1.0
        /// }
        /// state.withMultiArray(for: "s2") { stateMultiArray in
        /// stateMultiArray[0] = 2.0
        /// }
        /// ```
        #[unsafe(method_family(new))]
        #[method_id(newState)]
        pub unsafe fn newState(&self) -> Retained<MLState>;

        #[cfg(all(feature = "MLFeatureProvider", feature = "MLState"))]
        /// Run a stateful prediction synchronously.
        ///
        /// Use this method to run predictions on a stateful model.
        ///
        /// ```swift
        /// let state = model.newState()
        /// let prediction = try model.prediction(from: inputFeatures, using: state)
        /// ```
        ///
        /// - Parameters:
        /// - inputFeatures: The input features as declared in the model description.
        /// - state: The state object created by `newState()` method.
        /// - error: The output parameter to receive an error information on failure.
        #[unsafe(method_family(none))]
        #[method_id(predictionFromFeatures:usingState:error:_)]
        pub unsafe fn predictionFromFeatures_usingState_error(
            &self,
            input_features: &ProtocolObject<dyn MLFeatureProvider>,
            state: &MLState,
        ) -> Result<Retained<ProtocolObject<dyn MLFeatureProvider>>, Retained<NSError>>;

        #[cfg(all(
            feature = "MLFeatureProvider",
            feature = "MLPredictionOptions",
            feature = "MLState"
        ))]
        /// Run a stateful prediction synchronously with options.
        ///
        /// Use this method to run predictions on a stateful model.
        ///
        /// ```swift
        /// let state = model.newState()
        /// let prediction = try model.prediction(from: inputFeatures, using: state, options: predictionOptions)
        /// ```
        ///
        /// - Parameters:
        /// - inputFeatures: The input features as declared in the model description.
        /// - state: The state object created by `newState()` method.
        /// - options: The prediction options.
        /// - error: The output parameter to receive an error information on failure.
        #[unsafe(method_family(none))]
        #[method_id(predictionFromFeatures:usingState:options:error:_)]
        pub unsafe fn predictionFromFeatures_usingState_options_error(
            &self,
            input_features: &ProtocolObject<dyn MLFeatureProvider>,
            state: &MLState,
            options: &MLPredictionOptions,
        ) -> Result<Retained<ProtocolObject<dyn MLFeatureProvider>>, Retained<NSError>>;

        #[cfg(all(
            feature = "MLFeatureProvider",
            feature = "MLPredictionOptions",
            feature = "MLState",
            feature = "block2"
        ))]
        /// Run a stateful prediction asynchronously.
        ///
        /// Use this method to run predictions on a stateful model.
        ///
        /// Do not request a prediction while another prediction that shares the same state is in-flight, otherwise the behavior is undefined.
        ///
        /// ```swift
        /// let state = model.newState()
        /// let prediction = try await model.prediction(from: inputFeatures, using: state)
        /// ```
        ///
        /// - Parameters
        /// - input: The input features to make a prediction from.
        /// - state: The state object created by `newState()` method.
        /// - options: Prediction options to modify how the prediction is run.
        /// - completionHandler: A block that will be invoked once the prediction has completed successfully or unsuccessfully. On success, it is invoked with a valid model output. On failure, it is invoked with a nil output and NSError
        #[method(predictionFromFeatures:usingState:options:completionHandler:)]
        pub unsafe fn predictionFromFeatures_usingState_options_completionHandler(
            &self,
            input_features: &ProtocolObject<dyn MLFeatureProvider>,
            state: &MLState,
            options: &MLPredictionOptions,
            completion_handler: &block2::Block<
                dyn Fn(*mut ProtocolObject<dyn MLFeatureProvider>, *mut NSError),
            >,
        );
    }
);
