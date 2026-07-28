# MTLComputePipelineReflection

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection>

Information about the arguments of a compute function.

## Declaration

```swift
class MTLComputePipelineReflection
```

## Overview

An [MTLComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection) object provides access to the arguments of the compute function used in an [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) object. An [MTLComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection) object can be created along with an [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) object. Don’t create an [MTLComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection) object directly. Instead, call either the [makeComputePipelineState(function:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:options:reflection:)) or [makeComputePipelineState(function:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:options:completionhandler:)) method of [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) to create both an [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) object and an [MTLComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection) object.

[MTLComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection) objects can use a significant amount of memory; release any strong references to them after you finish creating pipeline objects.

## Topics

### Obtaining the arguments of the compute function
- [arguments](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection/arguments) — An array of instances that describe the arguments of a compute function.

### Instance Properties
- [bindings](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection/bindings)

## See also

### Introspection data
- [MTLAutoreleasedComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedcomputepipelinereflection) — A convenience type alias for an autoreleased compute pipeline reflection object.
- [MTLRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection) — Information about the arguments of a graphics function.
- [MTLAutoreleasedRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedrenderpipelinereflection) — A convenience type alias for an autoreleased pipeline reflection instance.
- [MTLBindingType](https://developer.apple.com/documentation/metal/mtlbindingtype)
- [MTLBinding](https://developer.apple.com/documentation/metal/mtlbinding)
- [MTLBindingAccess](https://developer.apple.com/documentation/metal/mtlbindingaccess)
- [MTLBufferBinding](https://developer.apple.com/documentation/metal/mtlbufferbinding)
- [MTLTextureBinding](https://developer.apple.com/documentation/metal/mtltexturebinding)
- [MTLThreadgroupBinding](https://developer.apple.com/documentation/metal/mtlthreadgroupbinding)
- [MTLObjectPayloadBinding](https://developer.apple.com/documentation/metal/mtlobjectpayloadbinding)
