//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlsubmeshtopology?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLSubmeshTopology;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MDLSubmeshTopology {}
);

impl MDLSubmeshTopology {
    extern_methods!(
        /// create a topology object corresponding to the topology in the submesh
        #[unsafe(method(initWithSubmesh:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSubmesh(
            this: Allocated<Self>,
            submesh: &MDLSubmesh,
        ) -> Retained<Self>;

        #[cfg(feature = "MDLMeshBuffer")]
        /// A buffer of 8 bit unsigned integer values, where each entry corresponds
        /// to the number of vertices making up a face.
        ///
        ///
        /// A submesh containing two triangles, a four sided polygon, and a
        /// line, would contain the data 3 3 4 2.
        /// If geometryType is of a fixed type, such as triangles, the buffer
        /// is optional, and will be created on demand if read.
        ///
        /// Indices to the vertex buffer will be stored in the index buffer
        /// correspondingly. In the example above, the indices would be stored
        /// in order, three indices for the first triangle, followed by three
        /// for the second, followed by four for the polygon, and finally two
        /// indices for the line.
        #[unsafe(method(faceTopology))]
        #[unsafe(method_family = none)]
        pub unsafe fn faceTopology(&self) -> Option<Retained<ProtocolObject<dyn MDLMeshBuffer>>>;

        #[cfg(feature = "MDLMeshBuffer")]
        /// Setter for [`faceTopology`][Self::faceTopology].
        #[unsafe(method(setFaceTopology:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFaceTopology(
            &self,
            face_topology: Option<&ProtocolObject<dyn MDLMeshBuffer>>,
        );

        /// The number of faces encoded in faceTopologyBuffer
        #[unsafe(method(faceCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn faceCount(&self) -> NSUInteger;

        /// Setter for [`faceCount`][Self::faceCount].
        #[unsafe(method(setFaceCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFaceCount(&self, face_count: NSUInteger);

        #[cfg(feature = "MDLMeshBuffer")]
        /// A crease value at a vertex to be applied during subdivision. Vertex creases
        /// A zero value is smooth, a one value is peaked. It is intended to be used
        /// with an index buffer, where the index buffer entries are vertex indices.
        /// The corresponding values in the corner sharpness attribute indicate the
        /// corner sharpness of those vertices. The index buffer is sparse. If a mesh
        /// has three sharp vertices, then the index buffer will have three entries.
        /// Since the number of entries in this vertex buffer is likely to be different
        /// than the number of entries in any other vertex buffer, it shouldn't be
        /// interleaved with other data.
        #[unsafe(method(vertexCreaseIndices))]
        #[unsafe(method_family = none)]
        pub unsafe fn vertexCreaseIndices(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MDLMeshBuffer>>>;

        #[cfg(feature = "MDLMeshBuffer")]
        /// Setter for [`vertexCreaseIndices`][Self::vertexCreaseIndices].
        #[unsafe(method(setVertexCreaseIndices:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVertexCreaseIndices(
            &self,
            vertex_crease_indices: Option<&ProtocolObject<dyn MDLMeshBuffer>>,
        );

        #[cfg(feature = "MDLMeshBuffer")]
        #[unsafe(method(vertexCreases))]
        #[unsafe(method_family = none)]
        pub unsafe fn vertexCreases(&self) -> Option<Retained<ProtocolObject<dyn MDLMeshBuffer>>>;

        #[cfg(feature = "MDLMeshBuffer")]
        /// Setter for [`vertexCreases`][Self::vertexCreases].
        #[unsafe(method(setVertexCreases:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVertexCreases(
            &self,
            vertex_creases: Option<&ProtocolObject<dyn MDLMeshBuffer>>,
        );

        /// The number of vertex creases encoded in vertexCreases
        #[unsafe(method(vertexCreaseCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn vertexCreaseCount(&self) -> NSUInteger;

        /// Setter for [`vertexCreaseCount`][Self::vertexCreaseCount].
        #[unsafe(method(setVertexCreaseCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVertexCreaseCount(&self, vertex_crease_count: NSUInteger);

        #[cfg(feature = "MDLMeshBuffer")]
        /// A crease value at an edge to be applied during subdivision. Edge creases
        /// A zero value is smooth, a one value is peaked. It is intended to be used
        /// with an index buffer, where the index buffer entries are edge index pairs.
        /// Accordingly, there will be two index entries for each edge sharpness entry,
        /// and the sharpness entry corresponds to the edge itself.
        /// The corresponding values in the edge sharpness attribute indicate the
        /// edge sharpness of those edges.  The index buffer is sparse. If a mesh
        /// has three sharp edges, then the index buffer will have six entries.
        /// Since the number of entries in this vertex buffer is likely to be different
        /// than the number of entries in any other vertex buffer, it shouldn't be
        /// interleaved with other data.
        #[unsafe(method(edgeCreaseIndices))]
        #[unsafe(method_family = none)]
        pub unsafe fn edgeCreaseIndices(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MDLMeshBuffer>>>;

        #[cfg(feature = "MDLMeshBuffer")]
        /// Setter for [`edgeCreaseIndices`][Self::edgeCreaseIndices].
        #[unsafe(method(setEdgeCreaseIndices:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEdgeCreaseIndices(
            &self,
            edge_crease_indices: Option<&ProtocolObject<dyn MDLMeshBuffer>>,
        );

        #[cfg(feature = "MDLMeshBuffer")]
        #[unsafe(method(edgeCreases))]
        #[unsafe(method_family = none)]
        pub unsafe fn edgeCreases(&self) -> Option<Retained<ProtocolObject<dyn MDLMeshBuffer>>>;

        #[cfg(feature = "MDLMeshBuffer")]
        /// Setter for [`edgeCreases`][Self::edgeCreases].
        #[unsafe(method(setEdgeCreases:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEdgeCreases(
            &self,
            edge_creases: Option<&ProtocolObject<dyn MDLMeshBuffer>>,
        );

        /// The number of edge creases encoded in edgeCreases
        #[unsafe(method(edgeCreaseCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn edgeCreaseCount(&self) -> NSUInteger;

        /// Setter for [`edgeCreaseCount`][Self::edgeCreaseCount].
        #[unsafe(method(setEdgeCreaseCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEdgeCreaseCount(&self, edge_crease_count: NSUInteger);

        #[cfg(feature = "MDLMeshBuffer")]
        /// The hole attribute is a vertex attribute of single integer values where
        /// each integer is an index of a face that is to be used as a hole. If there
        /// are two holes in a mesh, then the vertex buffer will have two entries.
        /// Since the number of entries in this vertex buffer is likely to be different
        /// than the number of entries in any other vertex buffer, it shouldn't be
        /// interleaved with other data.
        #[unsafe(method(holes))]
        #[unsafe(method_family = none)]
        pub unsafe fn holes(&self) -> Option<Retained<ProtocolObject<dyn MDLMeshBuffer>>>;

        #[cfg(feature = "MDLMeshBuffer")]
        /// Setter for [`holes`][Self::holes].
        #[unsafe(method(setHoles:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHoles(&self, holes: Option<&ProtocolObject<dyn MDLMeshBuffer>>);

        /// The number of holes encoded in holes
        #[unsafe(method(holeCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn holeCount(&self) -> NSUInteger;

        /// Setter for [`holeCount`][Self::holeCount].
        #[unsafe(method(setHoleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHoleCount(&self, hole_count: NSUInteger);
    );
}

/// Methods declared on superclass `NSObject`.
impl MDLSubmeshTopology {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// A drawable subset of an MDLMesh, with its own material
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/modelio/mdlsubmesh?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MDLSubmesh;
);

#[cfg(feature = "MDLTypes")]
extern_conformance!(
    unsafe impl MDLNamed for MDLSubmesh {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MDLSubmesh {}
);

impl MDLSubmesh {
    extern_methods!(
        #[cfg(all(
            feature = "MDLMaterial",
            feature = "MDLMeshBuffer",
            feature = "MDLTypes"
        ))]
        /// Initialize submesh with all data necessary to make properties valid
        #[unsafe(method(initWithName:indexBuffer:indexCount:indexType:geometryType:material:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName_indexBuffer_indexCount_indexType_geometryType_material(
            this: Allocated<Self>,
            name: &NSString,
            index_buffer: &ProtocolObject<dyn MDLMeshBuffer>,
            index_count: NSUInteger,
            index_type: MDLIndexBitDepth,
            geometry_type: MDLGeometryType,
            material: Option<&MDLMaterial>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "MDLMaterial",
            feature = "MDLMeshBuffer",
            feature = "MDLTypes"
        ))]
        /// Initialize submesh with all data necessary to make properties valid
        #[unsafe(method(initWithIndexBuffer:indexCount:indexType:geometryType:material:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIndexBuffer_indexCount_indexType_geometryType_material(
            this: Allocated<Self>,
            index_buffer: &ProtocolObject<dyn MDLMeshBuffer>,
            index_count: NSUInteger,
            index_type: MDLIndexBitDepth,
            geometry_type: MDLGeometryType,
            material: Option<&MDLMaterial>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "MDLMaterial",
            feature = "MDLMeshBuffer",
            feature = "MDLTypes"
        ))]
        /// Initialize submesh with all data necessary to make properties valid
        ///
        ///
        /// The geometry type will typically be MDLGeometryTypeVariableTopology,
        /// if other types are used the faceTopologyBuffer contents should
        /// reflect that.
        #[unsafe(method(initWithName:indexBuffer:indexCount:indexType:geometryType:material:topology:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName_indexBuffer_indexCount_indexType_geometryType_material_topology(
            this: Allocated<Self>,
            name: &NSString,
            index_buffer: &ProtocolObject<dyn MDLMeshBuffer>,
            index_count: NSUInteger,
            index_type: MDLIndexBitDepth,
            geometry_type: MDLGeometryType,
            material: Option<&MDLMaterial>,
            topology: Option<&MDLSubmeshTopology>,
        ) -> Retained<Self>;

        #[cfg(feature = "MDLTypes")]
        /// Initialize submesh using another submesh as input.
        ///
        /// the resulting submesh will have a new index type if necessary.
        /// If a conversion from the source submesh's geometry type to the requested
        /// geometry type is possible, conversion will be performed. Otherwise nil will
        /// be returned.
        #[unsafe(method(initWithMDLSubmesh:indexType:geometryType:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithMDLSubmesh_indexType_geometryType(
            this: Allocated<Self>,
            submesh: &MDLSubmesh,
            index_type: MDLIndexBitDepth,
            geometry_type: MDLGeometryType,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MDLMeshBuffer")]
        /// Index data referencing vertex data in parent mesh
        #[unsafe(method(indexBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexBuffer(&self) -> Retained<ProtocolObject<dyn MDLMeshBuffer>>;

        #[cfg(all(feature = "MDLMeshBuffer", feature = "MDLTypes"))]
        #[unsafe(method(indexBufferAsIndexType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexBufferAsIndexType(
            &self,
            index_type: MDLIndexBitDepth,
        ) -> Retained<ProtocolObject<dyn MDLMeshBuffer>>;

        /// Number of indices in the indexBuffer
        #[unsafe(method(indexCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexCount(&self) -> NSUInteger;

        #[cfg(feature = "MDLTypes")]
        /// Data type of indices in indexBuffer
        ///
        /// Support 8, 16, and 32 bit unsigned integer values
        #[unsafe(method(indexType))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexType(&self) -> MDLIndexBitDepth;

        #[cfg(feature = "MDLTypes")]
        /// Type of primitive that vertices referenced by the indexBuffer are
        /// assembled into
        #[unsafe(method(geometryType))]
        #[unsafe(method_family = none)]
        pub unsafe fn geometryType(&self) -> MDLGeometryType;

        #[cfg(feature = "MDLMaterial")]
        /// Material to apply when rendering this object
        #[unsafe(method(material))]
        #[unsafe(method_family = none)]
        pub unsafe fn material(&self) -> Option<Retained<MDLMaterial>>;

        #[cfg(feature = "MDLMaterial")]
        /// Setter for [`material`][Self::material].
        #[unsafe(method(setMaterial:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaterial(&self, material: Option<&MDLMaterial>);

        /// Topology data structure for use with MDLGeometryTypeVariableTopology
        ///
        ///
        /// ignored for geometry types other than MDLGeometryTypeVariableTopology.
        /// A submesh of type MDLGeometryTypeVariableTopology with no topology
        /// data is an empty submesh.
        #[unsafe(method(topology))]
        #[unsafe(method_family = none)]
        pub unsafe fn topology(&self) -> Option<Retained<MDLSubmeshTopology>>;

        /// Setter for [`topology`][Self::topology].
        #[unsafe(method(setTopology:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTopology(&self, topology: Option<&MDLSubmeshTopology>);

        /// Identifying name for this object
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Setter for [`name`][Self::name].
        #[unsafe(method(setName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setName(&self, name: &NSString);
    );
}

/// Methods declared on superclass `NSObject`.
impl MDLSubmesh {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
