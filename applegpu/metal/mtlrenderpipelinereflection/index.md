# MTLRenderPipelineReflection

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection>

Information about the arguments of a graphics function.

## Declaration

```swift
class MTLRenderPipelineReflection
```

## Overview

The [MTLRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection) class is an interface that represents the parameters for the shaders in a render pipeline state (see [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate)). Each pipeline state can include object, mesh, vertex, fragment, and tile shaders.

You create a reflection instance at the same time as the pipeline state that it represents by calling the appropriate [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) method. For example, the [makeRenderPipelineState(descriptor:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:reflection:)) and [makeRenderPipelineState(descriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:completionhandler:)-5gdww) methods create the pipeline state and the reflection instances at the same time.

> **Important:**
>  Only create reflection instances if you need them because each one can require a significant amount of memory.

For more information, see [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation).

## Topics

### Inspecting a shader’s parameter
- [fragmentBindings](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/fragmentbindings) — An array of binding instances, each of which represents a parameter of the pipeline state’s fragment shader.
- [meshBindings](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/meshbindings) — An array of binding instances, each of which represents a parameter of the pipeline state’s mesh shader.
- [objectBindings](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/objectbindings) — An array of binding instances, each of which represents a parameter of the pipeline state’s object shader.
- [tileBindings](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/tilebindings) — An array of binding instances, each of which represents a parameter of the pipeline state’s tile shader.
- [vertexBindings](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/vertexbindings) — An array of binding instances, each of which represents a parameter of the pipeline state’s vertex shader.

### Deprecated
- [vertexArguments](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/vertexarguments) — An array of argument instances, each of which represent a parameter of the pipeline state’s vertex shader.
- [fragmentArguments](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/fragmentarguments) — An array of argument instances, each of which represent a parameter of the pipeline state’s fragment shader.
- [tileArguments](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection/tilearguments) — An array of argument instances, each of which represent a parameter of the pipeline state’s tile shader.

## See also

### Introspection data
- [MTLComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection) — Information about the arguments of a compute function.
- [MTLAutoreleasedComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedcomputepipelinereflection) — A convenience type alias for an autoreleased compute pipeline reflection object.
- [MTLAutoreleasedRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedrenderpipelinereflection) — A convenience type alias for an autoreleased pipeline reflection instance.
- [MTLBindingType](https://developer.apple.com/documentation/metal/mtlbindingtype)
- [MTLBinding](https://developer.apple.com/documentation/metal/mtlbinding)
- [MTLBindingAccess](https://developer.apple.com/documentation/metal/mtlbindingaccess)
- [MTLBufferBinding](https://developer.apple.com/documentation/metal/mtlbufferbinding)
- [MTLTextureBinding](https://developer.apple.com/documentation/metal/mtltexturebinding)
- [MTLThreadgroupBinding](https://developer.apple.com/documentation/metal/mtlthreadgroupbinding)
- [MTLObjectPayloadBinding](https://developer.apple.com/documentation/metal/mtlobjectpayloadbinding)
