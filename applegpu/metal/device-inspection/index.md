# Device inspection

*API Collection*

<https://developer.apple.com/documentation/metal/device-inspection>

Locate and identify a GPU and the features it supports, and sample its counters.

## Topics

### Checking a GPU device’s feature support
- [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)) — Returns a Boolean value that indicates whether the GPU device supports the feature set of a specific GPU family.
- [MTLGPUFamily](https://developer.apple.com/documentation/metal/mtlgpufamily) — Represents the functionality for families of GPUs.
- [supportsFeatureSet(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfeatureset(_:)) — Returns a Boolean value that indicates whether the GPU device supports a specific feature set.
- [MTLFeatureSet](https://developer.apple.com/documentation/metal/mtlfeatureset) — The device feature sets that define specific platform, hardware, and software configurations.

### Checking compute support
- [maxThreadgroupMemoryLength](https://developer.apple.com/documentation/metal/mtldevice/maxthreadgroupmemorylength) — The maximum threadgroup memory available to a compute kernel, in bytes.
- [maxThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtldevice/maxthreadsperthreadgroup) — The maximum number of threads along each dimension of a threadgroup.

### Checking render support
- [supportsRaytracing](https://developer.apple.com/documentation/metal/mtldevice/supportsraytracing) — A Boolean value that indicates whether the GPU device supports ray tracing.
- [supportsPrimitiveMotionBlur](https://developer.apple.com/documentation/metal/mtldevice/supportsprimitivemotionblur) — A Boolean value that indicates whether the GPU device supports motion blur for ray tracing.
- [supportsRaytracingFromRender](https://developer.apple.com/documentation/metal/mtldevice/supportsraytracingfromrender) — A Boolean value that indicates whether you can call ray-tracing functions from a vertex or fragment shader.
- [supports32BitMSAA](https://developer.apple.com/documentation/metal/mtldevice/supports32bitmsaa) — A Boolean value that indicates whether the GPU can allocate 32-bit integer texture formats and resolve to 32-bit floating-point texture formats.
- [supportsPullModelInterpolation](https://developer.apple.com/documentation/metal/mtldevice/supportspullmodelinterpolation) — A Boolean value that indicates whether the GPU can compute multiple interpolations of a fragment function’s input.
- [supportsShaderBarycentricCoordinates](https://developer.apple.com/documentation/metal/mtldevice/supportsshaderbarycentriccoordinates) — A Boolean value that indicates whether the GPU supports barycentric coordinates.
- [supportsVertexAmplificationCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsvertexamplificationcount(_:)) — Returns a Boolean value that indicates whether the GPU supports an amplification factor.
- [areProgrammableSamplePositionsSupported](https://developer.apple.com/documentation/metal/mtldevice/areprogrammablesamplepositionssupported) — A Boolean value that indicates whether the GPU supports programmable sample positions.
- [areRasterOrderGroupsSupported](https://developer.apple.com/documentation/metal/mtldevice/arerasterordergroupssupported) — A Boolean value that indicates whether the GPU supports raster order groups.
- [areBarycentricCoordsSupported](https://developer.apple.com/documentation/metal/mtldevice/arebarycentriccoordssupported) — A Boolean value that indicates whether the GPU supports barycentric coordinates.

### Checking texture and sampler support
- [supports32BitFloatFiltering](https://developer.apple.com/documentation/metal/mtldevice/supports32bitfloatfiltering) — A Boolean value that indicates whether the GPU can filter a texture with a 32-bit floating-point format.
- [supportsBCTextureCompression](https://developer.apple.com/documentation/metal/mtldevice/supportsbctexturecompression) — A Boolean value that indicates whether you can use textures that use BC compression.
- [isDepth24Stencil8PixelFormatSupported](https://developer.apple.com/documentation/metal/mtldevice/isdepth24stencil8pixelformatsupported) — A Boolean value that indicates whether a device supports a packed depth-and-stencil pixel format.
- [supportsQueryTextureLOD](https://developer.apple.com/documentation/metal/mtldevice/supportsquerytexturelod) — A Boolean value that indicates whether you can query the texture level of detail from within a shader.
- [readWriteTextureSupport](https://developer.apple.com/documentation/metal/mtldevice/readwritetexturesupport) — The GPU device’s texture support tier.

### Checking function pointer support
- [supportsFunctionPointers](https://developer.apple.com/documentation/metal/mtldevice/supportsfunctionpointers) — A Boolean value that indicates whether the device supports function pointers in compute kernel functions.
- [supportsFunctionPointersFromRender](https://developer.apple.com/documentation/metal/mtldevice/supportsfunctionpointersfromrender) — A Boolean value that indicates whether the device supports function pointers in render functions.

### Checking a GPU device’s memory
- [currentAllocatedSize](https://developer.apple.com/documentation/metal/mtldevice/currentallocatedsize) — The total amount of memory, in bytes, the GPU device is using for all of its resources.
- [recommendedMaxWorkingSetSize](https://developer.apple.com/documentation/metal/mtldevice/recommendedmaxworkingsetsize) — An approximation of how much memory, in bytes, this GPU device can allocate without affecting its runtime performance.
- [hasUnifiedMemory](https://developer.apple.com/documentation/metal/mtldevice/hasunifiedmemory) — A Boolean value that indicates whether the GPU shares all of its memory with the CPU.
- [maxTransferRate](https://developer.apple.com/documentation/metal/mtldevice/maxtransferrate) — The highest theoretical rate, in bytes per second, the system can copy between system memory and the GPU’s dedicated memory (VRAM).

### Sampling a GPU device’s counters
- [counterSets](https://developer.apple.com/documentation/metal/mtldevice/countersets) — The counter sets supported by the device object.
- [supportsCounterSampling(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportscountersampling(_:)) — Returns a Boolean value that indicates whether you can read GPU counters at the specified command boundary.
- [MTLCounterSamplingPoint](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint) — Options for different times when you can sample GPU counters.
- [makeCounterSampleBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecountersamplebuffer(descriptor:)) — Creates a counter sample buffer.

### Sampling GPU and CPU timestamps simultaneously
- [sampleTimestamps()](https://developer.apple.com/documentation/metal/mtldevice/sampletimestamps()) — Captures and returns a CPU timestamp and a GPU timestamp from the same moment in time.

### Identifying a GPU device
- [name](https://developer.apple.com/documentation/metal/mtldevice/name) — The full name of the GPU device.
- [architecture](https://developer.apple.com/documentation/metal/mtldevice/architecture) — The architectural details of the GPU device.
- [MTLArchitecture](https://developer.apple.com/documentation/metal/mtlarchitecture) — A class that contains the architectural details of a GPU device.
- [registryID](https://developer.apple.com/documentation/metal/mtldevice/registryid) — The GPU device’s registry identifier.
- [location](https://developer.apple.com/documentation/metal/mtldevice/location) — The physical location of the GPU relative to the system.
- [MTLDeviceLocation](https://developer.apple.com/documentation/metal/mtldevicelocation) — Indicates the location of the GPU relative to the system it’s connect to.
- [locationNumber](https://developer.apple.com/documentation/metal/mtldevice/locationnumber) — A specific GPU position based on its general location.
- [isLowPower](https://developer.apple.com/documentation/metal/mtldevice/islowpower) — A Boolean value that indicates whether the GPU lowers its performance to conserve energy.
- [isRemovable](https://developer.apple.com/documentation/metal/mtldevice/isremovable) — A Boolean value that indicates whether the GPU is removable.
- [isHeadless](https://developer.apple.com/documentation/metal/mtldevice/isheadless) — A Boolean value that indicates whether a GPU device doesn’t have a connection to a display.
- [peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid) — The peer group ID the GPU belongs to, if applicable.
- [peerCount](https://developer.apple.com/documentation/metal/mtldevice/peercount) — The total number of GPUs in the peer group, if applicable.
- [peerIndex](https://developer.apple.com/documentation/metal/mtldevice/peerindex) — The unique identifier for a GPU in a peer group.

## See also

### Working with GPU devices
- [Work submission](https://developer.apple.com/documentation/metal/work-submission) — Create queues that submit work to the GPU or load assets into GPU resources, and indirect command buffers that group your frequent commands together.
- [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation) — Create pipeline states for render and compute passes, samplers, depth and stencil states, and indirect command buffers.
- [Resource creation](https://developer.apple.com/documentation/metal/resource-creation) — Load assets with input/output queues and make various resource instances, such as buffers, textures, acceleration structures, and memory heaps.
- [Shader library and archive creation](https://developer.apple.com/documentation/metal/shader-library-and-archive-creation) — Create static and dynamic shader libraries, and binary shader archives.
