# MTLIndirectRenderCommand

*Protocol · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectrendercommand>

A render command in an indirect command buffer.

## Declaration

```swift
protocol MTLIndirectRenderCommand : NSObjectProtocol
```

## Overview

Don’t implement this protocol; you get instances of this type by asking an [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) for them.

Use this instance to reset or encode a command. You need to reset a command before encoding a new command.

## Topics

### Setting command arguments
- [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setrenderpipelinestate(_:)) — Sets the render pipeline state for the command.
- [setVertexBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setvertexbuffer(_:offset:at:)) — Sets a vertex buffer argument for the command.
- [setFragmentBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setfragmentbuffer(_:offset:at:)) — Sets a fragment buffer argument for the command.

### Encoding a drawing command
- [drawPrimitives(_:vertexStart:vertexCount:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/drawprimitives(_:vertexstart:vertexcount:instancecount:baseinstance:)) — Encodes a command to render a number of instances of primitives using vertex data in contiguous array elements, starting from the base instance.
- [drawIndexedPrimitives(_:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/drawindexedprimitives(_:indexcount:indextype:indexbuffer:indexbufferoffset:instancecount:basevertex:baseinstance:)) — Encodes a command to render a number of instances of primitives using an index list specified in a buffer, starting from the base vertex of the base instance.
- [drawPatches(_:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/drawpatches(_:patchstart:patchcount:patchindexbuffer:patchindexbufferoffset:instancecount:baseinstance:tessellationfactorbuffer:tessellationfactorbufferoffset:tessellationfactorbufferinstancestride:)) — Encodes a command to render a number of instances of tessellated patches.
- [drawIndexedPatches(_:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/drawindexedpatches(_:patchstart:patchcount:patchindexbuffer:patchindexbufferoffset:controlpointindexbuffer:controlpointindexbufferoffset:instancecount:baseinstance:tessellationfactorbuffer:tessellationfactorbufferoffset:tessellationfactorbu-4mdz8) — Encodes a command to render a number of instances of tessellated patches, using a control point index buffer.

### Resetting a command
- [reset()](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/reset()) — Resets the command to its default state.

### Instance Methods
- [clearBarrier()](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/clearbarrier())
- [drawMeshThreadgroups(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/drawmeshthreadgroups(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:))
- [drawMeshThreads(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/drawmeshthreads(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:))
- [setBarrier()](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setbarrier())
- [setCullMode(_:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setcullmode(_:))
- [setDepthBias(_:slopeScale:clamp:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setdepthbias(_:slopescale:clamp:))
- [setDepthClipMode(_:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setdepthclipmode(_:))
- [setDepthStencilState(_:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setdepthstencilstate(_:))
- [setFrontFacing(_:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setfrontfacing(_:))
- [setMeshBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setmeshbuffer(_:offset:at:))
- [setObjectBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setobjectbuffer(_:offset:at:))
- [setObjectThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setobjectthreadgroupmemorylength(_:index:))
- [setTriangleFillMode(_:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/settrianglefillmode(_:))
- [setVertexBuffer(_:offset:attributeStride:at:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setvertexbuffer(_:offset:attributestride:at:))

## See also

### Render compute commands
- [MTLDrawPatchIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments) — The data layout required for drawing patches via indirect buffer calls.
- [MTLDrawPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments) — The data layout required for drawing primitives via indirect buffer calls.
- [MTLDrawIndexedPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments) — The data layout required for drawing indexed primitives via indirect buffer calls.
