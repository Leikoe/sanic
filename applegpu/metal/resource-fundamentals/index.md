# Resource fundamentals

*API Collection*

<https://developer.apple.com/documentation/metal/resource-fundamentals>

Control the common attributes of all Metal memory resources, including buffers and textures, and how to configure their underlying memory.

## Overview

A *resource* is a memory asset, such as an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) or [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), that a GPU can access (see [Buffers](https://developer.apple.com/documentation/metal/buffers) and [Textures](https://developer.apple.com/documentation/metal/textures)).

You can either allocate a resource from an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance or an [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instance (see [Memory heaps](https://developer.apple.com/documentation/metal/memory-heaps)). Metal sets a resource’s [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlresource/hazardtrackingmode) property to [MTLHazardTrackingMode.default](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/default) if you don’t select another tracking mode. The default value depends on what Metal instance creates the resource.

> **Important:**
> The value of an [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) instance’s [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlresource/hazardtrackingmode) property has no effect on the work you submit to an [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) (see [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization)) or resources that commands access through an argument buffer.

Each resource your app creates typically uses one of these storage modes:

- **[MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private)** — Apps can only access resources in private storage from the GPU.

- **[MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared)** — Apps can access resources in shared storage from both the CPU and the GPU.

- **[MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed)** — Apps can access resources in managed storage from both the CPU and the GPU, just like shared storage. However, the GPU backs resources in managed mode with memory in private storage.

Private mode resources give your app optimization opportunities that shared mode resources don’t. Managed mode resources also give your app the same opportunities and allow your to app access them from the CPU.

## Topics

### Resource management
- [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes) — Set a storage mode that defines the memory location and access permissions of a resource.
- [Choosing a resource storage mode for Apple GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-apple-gpus) — Select an appropriate storage mode for your textures and buffers on Apple GPUs.
- [Choosing a resource storage mode for Intel and AMD GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-intel-and-amd-gpus) — Select an appropriate storage mode for your textures and buffers on AMD and Intel GPUs.
- [Copying data to a private resource](https://developer.apple.com/documentation/metal/copying-data-to-a-private-resource) — Use a blit command encoder to copy buffer or texture data to a private resource.
- [Synchronizing a managed resource in macOS](https://developer.apple.com/documentation/metal/synchronizing-a-managed-resource-in-macos) — Manually synchronize memory for a Metal resource in apps.
- [Transferring data between connected GPUs](https://developer.apple.com/documentation/metal/transferring-data-between-connected-gpus) — Use high-speed connections between GPUs to transfer data quickly.
- [Reducing the memory footprint of Metal apps](https://developer.apple.com/documentation/metal/reducing-the-memory-footprint-of-metal-apps) — Learn best practices for using memory efficiently in iOS and tvOS.

### Residency sets
- [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) — Organize your resources into groups and influence when they become accessible to the GPU.
- [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) — A collection of resource allocations that can move in and out of resident memory.
- [MTLResidencySetDescriptor](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor) — A configuration that customizes the behavior for a residency set.

### View pools
- [MTLResourceViewPool](https://developer.apple.com/documentation/metal/mtlresourceviewpool) — Contains views over resources of a specific type, and allows you to manage those views.
- [MTLResourceViewPoolDescriptor](https://developer.apple.com/documentation/metal/mtlresourceviewpooldescriptor) — Provides parameters for creating a resource view pool.
- [MTLTextureViewPool](https://developer.apple.com/documentation/metal/mtltextureviewpool) — A pool of lightweight texture views.
- [MTLTextureViewDescriptor](https://developer.apple.com/documentation/metal/mtltextureviewdescriptor)

### Tensors
- [MTLTensor](https://developer.apple.com/documentation/metal/mtltensor) — A resource representing a multi-dimensional array that you can use with machine learning workloads.
- [MTLTensorDescriptor](https://developer.apple.com/documentation/metal/mtltensordescriptor) — A configuration type for creating new tensor instances.
- [MTLTensorExtents](https://developer.apple.com/documentation/metal/mtltensorextents) — An integer array that holds per-dimension values such as tensor sizes, strides, or block factors
- [MTLTensorReferenceType](https://developer.apple.com/documentation/metal/mtltensorreferencetype) — An object that represents a tensor in the shading language in a struct or array.
- [MTLTensorUsage](https://developer.apple.com/documentation/metal/mtltensorusage) — The contexts in which you can use a tensor.
- [MTLTensorDomain](https://developer.apple.com/documentation/metal/mtltensordomain) — An error domain for errors that pertain to creating a tensor.
- [MTLTensorBinding](https://developer.apple.com/documentation/metal/mtltensorbinding) — An object that represents a tensor bound to a graphics or compute function or a machine learning function.
- [MTLTensorError](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct)
- [MTLTensorError.Code](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct/code) — The error codes that Metal can raise when you create a tensor.
- [MTLTensorDataType](https://developer.apple.com/documentation/metal/mtltensordatatype) — The possible data types for the elements of a tensor.
- [MTLTensorDomain](https://developer.apple.com/documentation/metal/mtltensordomain) — An error domain for errors that pertain to creating a tensor.
- [MTL_TENSOR_MAX_RANK](https://developer.apple.com/documentation/metal/mtl_tensor_max_rank)

### Sparse resources
- [MTLBufferSparseTier](https://developer.apple.com/documentation/metal/mtlbuffersparsetier) — Enumerates the different support levels for sparse buffers.
- [MTL4CopySparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation) — Groups together arguments for an operation to copy a sparse buffer mapping.
- [MTL4UpdateSparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation) — Groups together arguments for an operation to update a sparse buffer mapping.
- [MTLTextureSparseTier](https://developer.apple.com/documentation/metal/mtltexturesparsetier) — Enumerates the different support levels for sparse textures.
- [MTL4CopySparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation) — Groups together arguments for an operation to copy a sparse texture mapping.
- [MTL4UpdateSparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation) — Groups together arguments for an operation to update a sparse texture mapping.

### Common resource functionality
- [MTLGPUAddress](https://developer.apple.com/documentation/metal/mtlgpuaddress) — A 64-bit unsigned integer type appropriate for storing GPU addresses.
- [MTLAllocation](https://developer.apple.com/documentation/metal/mtlallocation) — A memory allocation from a Metal GPU device, such as a memory heap, texture, or data buffer.
- [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) — An allocation of memory accessible to a GPU.
- [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) — Optional arguments used to set the behavior of a resource.
- [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage) — Options that describe how a graphics or compute function uses an argument buffer’s resource.
- [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid)

## See also

### Resources
- [Buffers](https://developer.apple.com/documentation/metal/buffers) — Create and manage untyped data your app uses to exchange information with its shader functions.
- [Textures](https://developer.apple.com/documentation/metal/textures) — Create and manage typed data your app uses to exchange information with its shader functions.
- [Memory heaps](https://developer.apple.com/documentation/metal/memory-heaps) — Take control of your app’s GPU memory management by creating a large memory allocation for various buffers, textures, and other resources.
- [Resource loading](https://developer.apple.com/documentation/metal/resource-loading) — Load assets in your games and apps quickly by running a dedicated input/output queue alongside your GPU tasks.
- [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization) — Prevent multiple commands that can access the same resources simultaneously by coordinating those reads and writes with barriers, fences, or events.
