# Ray tracing with acceleration structures

*API Collection*

<https://developer.apple.com/documentation/metal/ray-tracing-with-acceleration-structures>

Build a representation of your scene’s geometry using triangles and bounding volumes to quickly trace rays through the scene.

## Overview

Ray tracing can improve your content’s realism by more accurately modeling the behavior of light than traditional rendering. You can also use ray tracing to implement similar techniques that rely on line-of-sight, such as sound obstruction or visually based AI functions.

To apply ray tracing in your app:

1. Create acceleration structures that represent objects in a scene.

2. Define a ray’s behavior when it collides into parts of an acceleration structure by creating either intersectors or intersection queries.

3. Generate rays into the scene from a new or existing shader.

An *intersector* uses a table of your intersection functions that define the custom behavior for each intersection type. An *intersection query* returns to your calling function to handle the custom behavior for all intersection types.

Intersectors work with compute kernels on all GPUs, and with render shaders only on Apple silicon GPUs. Alternatively, your app can use intersection queries on non-Apple GPUs, or for porting code from other graphics APIs.

## Topics

### Ray tracing samples
- [Accelerating ray tracing using Metal](https://developer.apple.com/documentation/metal/accelerating-ray-tracing-using-metal) — Implement ray-traced rendering using GPU-based parallel processing.
- [Control the ray tracing process using intersection queries](https://developer.apple.com/documentation/metal/control-the-ray-tracing-process-using-intersection-queries) — Explicitly enumerate a ray’s intersections with acceleration structures by creating an intersection query object.
- [Rendering reflections in real time using ray tracing](https://developer.apple.com/documentation/metal/rendering-reflections-in-real-time-using-ray-tracing) — Implement realistic real-time lighting by dynamically generating reflection maps by encoding a ray-tracing compute pass.
- [Rendering a curve primitive in a ray tracing scene](https://developer.apple.com/documentation/metal/rendering-a-curve-primitive-in-a-ray-tracing-scene) — Implement ray traced rendering using GPU-based parallel processing.

### Acceleration structures
- [Improving ray-tracing data access using per-primitive data](https://developer.apple.com/documentation/metal/improving-ray-tracing-data-access-using-per-primitive-data) — Simplify data access and improve GPU utilization by storing custom primitive data directly in the acceleration structure.
- [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) — A collection of model data for GPU-accelerated intersection of rays with the model.
- [MTL4AccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuredescriptor) — Base class for Metal 4 acceleration structure descriptors.
- [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) — A base class for classes that define the configuration for a new acceleration structure.
- [MTL4PrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor) — Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and bounding boxes.
- [MTLPrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor) — A description of an acceleration structure that contains geometry primitives.
- [MTL4InstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor) — Descriptor for an instance acceleration structure.
- [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that derives from instances of primitive acceleration structures.
- [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) — Encodes commands that build and refit acceleration structures for a single pass.
- [MTLAccelerationStructureUsage](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage) — Options that affect how Metal builds an acceleration structure and the behavior of that acceleration structure.
- [MTLAccelerationStructureRefitOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructurerefitoptions)

### Acceleration structures passes
- [MTLAccelerationStructurePassDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepassdescriptor)
- [MTLAccelerationStructurePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptor)
- [MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptorarray)

### Geometry descriptors
- [MTL4AccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor) — Base class for all Metal 4 acceleration structure geometry descriptors.
- [MTLAccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor) — A base class for descriptors that contain geometry data to convert into a ray-tracing acceleration structure.
- [MTL4AccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor) — Describes triangle geometry suitable for ray tracing.
- [MTLAccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor) — A description of a list of triangle primitives to turn into an acceleration structure.
- [MTL4AccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor) — Describes curve geometry suitable for ray tracing.
- [MTLAccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor) — A descriptor you configure with curve geometry for building acceleration structures.
- [MTLCurveType](https://developer.apple.com/documentation/metal/mtlcurvetype)
- [MTLCurveBasis](https://developer.apple.com/documentation/metal/mtlcurvebasis)
- [MTLCurveEndCaps](https://developer.apple.com/documentation/metal/mtlcurveendcaps)
- [MTL4AccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor) — Describes bounding-box geometry suitable for ray tracing.
- [MTLAccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor) — A description of a list of bounding boxes to turn into an acceleration structure.

### Motion geometry descriptors
- [MTL4AccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor) — Describes motion triangle geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor) — A description of a list of triangle primitives, as motion keyframe data, to turn into an acceleration structure.
- [MTL4AccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor) — Describes motion curve geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioncurvegeometrydescriptor)
- [MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor) — Describes motion bounding box geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor) — A description of a list of bounding boxes, as motion keyframe data, to turn into an acceleration structure.
- [MTLMotionKeyframeData](https://developer.apple.com/documentation/metal/mtlmotionkeyframedata) — Geometry data for a specific keyframe to use in a moving instance.

### Instance descriptors
- [MTLAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure.
- [MTLAccelerationStructureUserIDInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier for the instance.
- [MTLAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier and motion data for the instance.
- [MTLAccelerationStructureInstanceOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions) — Options for adjusting the behavior of an instanced acceleration structure.
- [MTL4IndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor) — Descriptor for an “indirect” instance acceleration structure that allows providing the instance count and motion transform count indirectly, through buffer references.
- [MTLIndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that Metal derives from instances of primitive acceleration structures that the GPU can populate.
- [MTLIndirectAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure that the GPU can populate.
- [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor) — A description of an instance in an acceleration structure that the GPU can populate, with motion data for the instance.

### Intersection function tables
- [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) — A table of intersection functions that Metal calls to perform ray-tracing intersection tests.
- [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) — A specification of how to create an intersection function table.
- [MTLIntersectionFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiondescriptor) — A description of an intersection function that performs an intersection test.
- [MTLIntersectionFunctionSignature](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature) — Constants for specifying different types of custom intersection functions.
- [MTLIntersectionFunctionBufferArguments](https://developer.apple.com/documentation/metal/mtlintersectionfunctionbufferarguments)

### Supporting types
- [MTLAxisAlignedBoundingBox](https://developer.apple.com/documentation/metal/mtlaxisalignedboundingbox-swift.typealias) — The bounds for an axis-aligned bounding box.
- [MTLPackedFloat3](https://developer.apple.com/documentation/metal/mtlpackedfloat3-swift.typealias) — }
- [MTLPackedFloat4x3](https://developer.apple.com/documentation/metal/mtlpackedfloat4x3-swift.typealias) — A structure that contains the top three rows of a 4x4 matrix of 32-bit floating-point values, in column-major order.
- [MTLPackedFloat3Make(_:_:_:)](https://developer.apple.com/documentation/metal/mtlpackedfloat3make(_:_:_:)) — Returns a new packed vector with three floating-point values.
- [MTL4BufferRange](https://developer.apple.com/documentation/metal/mtl4bufferrange)
- [MTL4BufferRangeMake(_:_:)](https://developer.apple.com/documentation/metal/mtl4bufferrangemake(_:_:))

## See also

### Command encoders
- [Render passes](https://developer.apple.com/documentation/metal/render-passes) — Encode a render pass to draw graphics into an image.
- [Compute passes](https://developer.apple.com/documentation/metal/compute-passes) — Encode a compute pass that runs computations in parallel on a thread grid, processing and manipulating Metal resource data on multiple cores of a GPU.
- [Machine learning passes](https://developer.apple.com/documentation/metal/machine-learning-passes) — Add machine learning model inference to your Metal app’s GPU workflow.
- [Blit passes](https://developer.apple.com/documentation/metal/blit-passes) — Encode a block information transfer pass to adjust and copy data to and from GPU resources, such as buffers and textures.
- [Indirect command encoding](https://developer.apple.com/documentation/metal/indirect-command-encoding) — Store draw commands in Metal buffers and run them at a later time on the GPU, either once or repeatedly.
