//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

/// The scatter mode.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphscattermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSGraphScatterMode(pub NSInteger);
impl MPSGraphScatterMode {
    /// Add
    #[doc(alias = "MPSGraphScatterModeAdd")]
    pub const Add: Self = Self(0);
    /// Sub
    #[doc(alias = "MPSGraphScatterModeSub")]
    pub const Sub: Self = Self(1);
    /// Multiply
    #[doc(alias = "MPSGraphScatterModeMul")]
    pub const Mul: Self = Self(2);
    /// Divide
    #[doc(alias = "MPSGraphScatterModeDiv")]
    pub const Div: Self = Self(3);
    /// Minimum
    #[doc(alias = "MPSGraphScatterModeMin")]
    pub const Min: Self = Self(4);
    /// Maximum
    #[doc(alias = "MPSGraphScatterModeMax")]
    pub const Max: Self = Self(5);
    /// Set
    #[doc(alias = "MPSGraphScatterModeSet")]
    pub const Set: Self = Self(6);
}

unsafe impl Encode for MPSGraphScatterMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPSGraphScatterMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// ScatterNDOps.
#[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
impl MPSGraph {
    extern_methods!(
        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a ScatterND operation and returns the result tensor.
        ///
        /// Scatters the slices in updatesTensor to the result tensor along the indices in indicesTensor.
        /// The scatter is defined as
        /// ```md
        /// B = batchDims
        /// U = updates.rank - B
        /// P = res.rank - B
        /// Q = inds.rank - B
        /// K = inds.shape[-1]
        /// index_slice = indices[i_{b0},...,i_{bB},i_{0},..,i_{Q-1}]
        /// res[i_{b0},...,i_{bB},index_slice[0],...,index_slice[K-1]] = updates[i_{b0},...,i_{bB},i_{0},...,i_{Q-1}]
        /// ```
        /// Collisions will be summed, and slices not set by indices are set to 0. The tensors have the following shape requirements
        /// ```md
        /// K
        /// <
        /// = P
        /// U = (P-K) + Q-1
        /// indices.shape[0:Q-1] = updates.shape[0:Q-1]
        /// updates.shape[Q:U] = res.shape[K:P]
        /// ```
        ///
        /// - Parameters:
        /// - updatesTensor: Tensor containing slices to be inserted into the result tensor.
        /// - indicesTensor: Tensor containg the result indices to insert slices at
        /// - shape: The shape of the result tensor.
        /// - batchDimensions: The number of batch dimensions
        /// - mode: The type of update to use on the destination
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(scatterNDWithUpdatesTensor:indicesTensor:shape:batchDimensions:mode:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scatterNDWithUpdatesTensor_indicesTensor_shape_batchDimensions_mode_name(
            &self,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            shape: &MPSShape,
            batch_dimensions: NSUInteger,
            mode: MPSGraphScatterMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a ScatterND operation and returns the result tensor.
        ///
        /// Scatters the slices in updatesTensor to the result tensor along the indices in indicesTensor.
        /// The scatter is defined as
        /// ```md
        /// B = batchDims
        /// U = updates.rank - B
        /// P = res.rank - B
        /// Q = inds.rank - B
        /// K = inds.shape[-1]
        /// index_slice = indices[i_{b0},...,i_{bB},i_{0},..,i_{Q-1}]
        /// res[i_{b0},...,i_{bB},index_slice[0],...,index_slice[K-1]] = updates[i_{b0},...,i_{bB},i_{0},...,i_{Q-1}]
        /// ```
        /// Collisions will be summed, and slices not set by indices are set to 0. The tensors have the following shape requirements
        /// ```md
        /// K
        /// <
        /// = P
        /// U = (P-K) + Q-1
        /// indices.shape[0:Q-1] = updates.shape[0:Q-1]
        /// updates.shape[Q:U] = res.shape[K:P]
        /// ```
        ///
        /// - Parameters:
        /// - updatesTensor: Tensor containing slices to be inserted into the result tensor.
        /// - indicesTensor: Tensor containg the result indices to insert slices at
        /// - shape: The shape of the result tensor.
        /// - batchDimensions: The number of batch dimensions
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(scatterNDWithUpdatesTensor:indicesTensor:shape:batchDimensions:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scatterNDWithUpdatesTensor_indicesTensor_shape_batchDimensions_name(
            &self,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            shape: &MPSShape,
            batch_dimensions: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a ScatterND operation and returns the result tensor.
        ///
        /// Scatters the slices in updatesTensor to the result tensor along the indices in indicesTensor, on top of dataTensor.
        /// The scatter is defined as
        /// ```md
        /// B = batchDims
        /// U = updates.rank - B
        /// P = res.rank - B
        /// Q = inds.rank - B
        /// K = inds.shape[-1]
        /// index_slice = indices[i_{b0},...,i_{bB},i_{0},..,i_{Q-1}]
        /// res[...] = data[...]
        /// res[i_{b0},...,i_{bB},index_slice[0],...,index_slice[K-1]] += updates[i_{b0},...,i_{bB},i_{0},...,i_{Q-1}] // Note += is used but this depends on mode
        /// ```
        /// Collisions will be updated according to mode, and slices not set by indices are set to 0. The tensors have the following shape requirements
        /// ```md
        /// K
        /// <
        /// = P
        /// U = (P-K) + Q-1
        /// data.shape = res.shape
        /// indices.shape[0:Q-1] = updates.shape[0:Q-1]
        /// updates.shape[Q:U] = res.shape[K:P]
        /// ```
        ///
        /// - Parameters:
        /// - dataTensor: Tensor containing inital values of same shape as result tensor
        /// - updatesTensor: Tensor containing slices to be inserted into the result tensor.
        /// - indicesTensor: Tensor containg the result indices to insert slices at
        /// - batchDimensions: The number of batch dimensions
        /// - mode: The type of update to use on the destination
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(scatterNDWithDataTensor:updatesTensor:indicesTensor:batchDimensions:mode:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scatterNDWithDataTensor_updatesTensor_indicesTensor_batchDimensions_mode_name(
            &self,
            data_tensor: &MPSGraphTensor,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            batch_dimensions: NSUInteger,
            mode: MPSGraphScatterMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    );
}

/// MPSGraphScatterOps.
#[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
impl MPSGraph {
    extern_methods!(
        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a Scatter operation and returns the result tensor.
        ///
        /// Scatters the slices in updatesTensor to the result tensor along the indices in indicesTensor.
        /// The scatter is defined as
        /// ```md
        /// U = updates.rank
        /// P = res.rank
        /// res[i_{0},...,i_{axis-1},indices[i_{axis}],i_{axis+1},...,i_{U-1}] = updates[i_{0},...,i_{axis-1},i_{axis},i_{axis+1},...,i_{U-1}]
        /// ```
        /// Collisions will be updated according to mode. The tensors have the following shape requirements
        /// ```md
        /// U = P
        /// indices.rank = 1
        /// updates.shape[0:axis-1] = res.shape[0:axis-1]
        /// updates.shape[axis] = indices.shape[0]
        /// updates.shape[axis+1:U] = res.shape[0:P]
        /// ```
        ///
        /// - Parameters:
        /// - updatesTensor: Tensor containing values to be inserted into the result tensor.
        /// - indicesTensor: Tensor containg the result indices to insert values at.
        /// - shape: The shape of the result tensor.
        /// - axis: The axis of the result tensor to scatter values along.
        /// - mode: The type of update to use on the destination.
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(scatterWithUpdatesTensor:indicesTensor:shape:axis:mode:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scatterWithUpdatesTensor_indicesTensor_shape_axis_mode_name(
            &self,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            shape: &MPSShape,
            axis: NSInteger,
            mode: MPSGraphScatterMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a Scatter operation and returns the result tensor.
        ///
        /// Scatters the slices in updatesTensor to the result tensor along the indices in indicesTensor, on top of dataTensor.
        /// The scatter is defined as
        /// ```md
        /// U = updates.rank
        /// P = res.rank
        /// res[...] = data[...]
        /// res[i_{0},...,i_{axis-1},indices[i_{axis}],i_{axis+1},...,i_{U-1}] += updates[i_{0},...,i_{axis-1},i_{axis},i_{axis+1},...,i_{U-1}] // Note += is used but this depends on mode
        /// ```
        /// Collisions will be updated according to mode. The tensors have the following shape requirements
        /// ```md
        /// U = P
        /// indices.rank = 1
        /// data.shape = res.shape
        /// updates.shape[0:axis-1] = res.shape[0:axis-1]
        /// updates.shape[axis] = indices.shape[0]
        /// updates.shape[axis+1:U] = res.shape[0:P]
        /// ```
        ///
        /// - Parameters:
        /// - dataTensor: Tensor containing inital values of same shape as result tensor
        /// - updatesTensor: Tensor containing values to be inserted into the result tensor.
        /// - indicesTensor: Tensor containg the result indices to insert values at
        /// - axis: The axis of the result tensor to scatter values along
        /// - mode: The type of update to use on the destination
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(scatterWithDataTensor:updatesTensor:indicesTensor:axis:mode:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scatterWithDataTensor_updatesTensor_indicesTensor_axis_mode_name(
            &self,
            data_tensor: &MPSGraphTensor,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            axis: NSInteger,
            mode: MPSGraphScatterMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    );
}

/// MPSGraphScatterAlongAxisOps.
#[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
impl MPSGraph {
    extern_methods!(
        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a ScatterAlongAxis operation and returns the result tensor.
        ///
        /// Scatter values from `updatesTensor` along the specified `axis` at indices in `indicesTensor` into a result tensor.
        /// Values are updated following `mode`. See MPSGraphScatterMode.
        /// The shape of `updatesTensor` and `indicesTensor` must match. `shape` must match except at `axis`.
        /// The shape of the result tensor is equal to `shape` and initialized with an initial value corresponding to `mode`.
        /// If an index is out of bounds of `shape` along `axis` the update value is skipped.
        ///
        /// - Parameters:
        /// - axis: The axis to scatter to. Negative values wrap around
        /// - updatesTensor: The input tensor to scatter values from
        /// - indicesTensor: Int32 or Int64 tensor used to index the result tensor.
        /// - mode: The type of update to use
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(scatterAlongAxis:withUpdatesTensor:indicesTensor:shape:mode:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scatterAlongAxis_withUpdatesTensor_indicesTensor_shape_mode_name(
            &self,
            axis: NSInteger,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            shape: &MPSShape,
            mode: MPSGraphScatterMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a ScatterAlongAxis operation and returns the result tensor.
        ///
        /// Scatter values from `updatesTensor` along the specified `axis` at indices in `indicesTensor` into a result tensor.
        /// Values are updated following `mode`. See MPSGraphScatterMode.
        /// The shape of `updatesTensor` and `indicesTensor` must match. `shape` must match except at `axis`.
        /// The shape of the result tensor is equal to `shape` and initialized with an initial value corresponding to `mode`.
        /// If an index is out of bounds of `shape` along `axis` the update value is skipped.
        ///
        /// - Parameters:
        /// - axisTensor: Scalar Int32 tensor. The axis to scatter to. Negative values wrap around
        /// - updatesTensor: The input tensor to scatter values from
        /// - indicesTensor: Int32 or Int64 tensor used to index the result tensor.
        /// - mode: The type of update to use
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(scatterAlongAxisTensor:withUpdatesTensor:indicesTensor:shape:mode:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scatterAlongAxisTensor_withUpdatesTensor_indicesTensor_shape_mode_name(
            &self,
            axis_tensor: &MPSGraphTensor,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            shape: &MPSShape,
            mode: MPSGraphScatterMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a ScatterAlongAxis operation and returns the result tensor.
        ///
        /// Scatter values from `updatesTensor` along the specified `axis` at indices in `indicesTensor` onto `dataTensor`.
        /// Values in `dataTensor` are updated following `mode`. See MPSGraphScatterMode.
        /// The shape of `updatesTensor` and `indicesTensor` must match. The shape of `dataTensor` must match except at `axis`.
        /// If an index is out of bounds of `shape` along `axis` the update value is skipped.
        /// For example,
        /// ```md
        /// data = [ [0, 0, 0],
        /// [1, 1, 1],
        /// [2, 2, 2],
        /// [3, 3, 3] ]
        /// updates = [ [1, 2, 3],
        /// [4, 5, 6] ]
        /// indices = [ [2, 1, 0],
        /// [1, 3, 2] ]
        /// axis = 0
        /// result = scatterAlongAxis(axis, data, updates, indices, MPSGraphScatterModeAdd, "scatter")
        /// result = [ [0, 0, 3],
        /// [5, 3, 1],
        /// [3, 2, 8],
        /// [3, 8, 3] ]
        /// ```
        ///
        /// - Parameters:
        /// - axis: The axis to scatter to. Negative values wrap around
        /// - dataTensor: The input tensor to scatter values onto
        /// - updatesTensor: The input tensor to scatter values from
        /// - indicesTensor: Int32 or Int64 tensor used to index the result tensor.
        /// - mode: The type of update to use
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(scatterAlongAxis:withDataTensor:updatesTensor:indicesTensor:mode:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scatterAlongAxis_withDataTensor_updatesTensor_indicesTensor_mode_name(
            &self,
            axis: NSInteger,
            data_tensor: &MPSGraphTensor,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            mode: MPSGraphScatterMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a ScatterAlongAxis operation and returns the result tensor.
        ///
        /// Scatter values from `updatesTensor` along the specified `axis` at indices in `indicesTensor` onto `dataTensor`.
        /// Values in `dataTensor` are updated following `mode`. See MPSGraphScatterMode.
        /// The shape of `updatesTensor` and `indicesTensor` must match. The shape of `dataTensor` must match except at `axis`.
        /// If an index is out of bounds of `shape` along `axis` the update value is skipped.
        /// For example,
        /// ```md
        /// data = [ [0, 0, 0],
        /// [1, 1, 1],
        /// [2, 2, 2],
        /// [3, 3, 3] ]
        /// updates = [ [1, 2, 3],
        /// [4, 5, 6] ]
        /// indices = [ [2, 1, 0],
        /// [1, 3, 2] ]
        /// axis = 0
        /// result = scatterAlongAxis(axis, data, updates, indices, MPSGraphScatterModeAdd, "scatter")
        /// result = [ [0, 0, 3],
        /// [5, 3, 1],
        /// [3, 2, 8],
        /// [3, 8, 3] ]
        /// ```
        ///
        /// - Parameters:
        /// - axisTensor: Scalar Int32 tensor. The axis to scatter to. Negative values wrap around
        /// - dataTensor: The input tensor to scatter values onto
        /// - updatesTensor: The input tensor to scatter values from
        /// - indicesTensor: Int32 or Int64 tensor used to index the result tensor.
        /// - mode: The type of update to use
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(scatterAlongAxisTensor:withDataTensor:updatesTensor:indicesTensor:mode:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn scatterAlongAxisTensor_withDataTensor_updatesTensor_indicesTensor_mode_name(
            &self,
            axis_tensor: &MPSGraphTensor,
            data_tensor: &MPSGraphTensor,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            mode: MPSGraphScatterMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    );
}
