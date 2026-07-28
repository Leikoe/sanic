# vertexInvocations

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommoncounter/vertexinvocations>

The common name for the counter that tracks the number of times a render pass calls any vertex shader.

## Declaration

```swift
static let vertexInvocations: MTLCommonCounter
```

## See also

### Common counter names
- [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounter/timestamp) — The common name for the counter that tracks the current time.
- [tessellationInputPatches](https://developer.apple.com/documentation/metal/mtlcommoncounter/tessellationinputpatches) — The common name for the counter that tracks the number of tessellation patches a render pass sends to the tessellation stage.
- [postTessellationVertexInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/posttessellationvertexinvocations) — The common name for the counter that tracks the number of vertices a render pass sends to a post-tessellation vertex shader.
- [clipperInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/clipperinvocations) — The common name for the counter that tracks the number of primitives a render pass sends to the clip stage.
- [clipperPrimitivesOut](https://developer.apple.com/documentation/metal/mtlcommoncounter/clipperprimitivesout) — The common name for the counter that tracks the number of primitives the clip stage produces during a render pass.
- [fragmentInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/fragmentinvocations) — The common name for the counter that tracks the number of times a render pass calls fragment shaders.
- [fragmentsPassed](https://developer.apple.com/documentation/metal/mtlcommoncounter/fragmentspassed) — The common name for the counter that tracks the number of fragments a render pass sends to the visibility and blend stages.
- [computeKernelInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/computekernelinvocations) — The common name for the counter that tracks the number of times a pass invokes any compute kernel.
- [totalCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/totalcycles) — The common name for the counter that tracks the total number of cycles the GPU uses to run a pass.
- [vertexCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/vertexcycles) — The common name for the counter that tracks the number of cycles the GPU uses to run vertex shaders during a pass.
- [postTessellationVertexCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/posttessellationvertexcycles) — The common name for the counter that tracks the number of cycles the GPU uses to run post-tessellation vertex shaders during a pass.
- [fragmentCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/fragmentcycles) — The common name for the counter that tracks the number of cycles the GPU uses to run fragment shaders during a pass.
- [tessellationCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/tessellationcycles) — The common name for the counter that tracks the number of cycles the GPU uses to run the tessellation stage during a pass.
- [renderTargetWriteCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/rendertargetwritecycles) — The common name for the counter that tracks the number of cycles the GPU uses to write data to render targets during a render pass.
