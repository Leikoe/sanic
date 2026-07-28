# clipperInvocations

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/clipperinvocations>

The number of primitives a render pass sends to the clip stage.

## Declaration

```swift
var clipperInvocations: UInt64
```

## See also

### Statistics values
- [tessellationInputPatches](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/tessellationinputpatches) — The number of tessellation patches a render pass sends to the tessellation stage.
- [vertexInvocations](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/vertexinvocations) — The number of times a render pass calls any vertex shader.
- [postTessellationVertexInvocations](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/posttessellationvertexinvocations) — The number of vertices a render pass sends to a post-tessellation vertex shader.
- [clipperPrimitivesOut](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/clipperprimitivesout) — The number of primitives the clip stage produces during a render pass.
- [fragmentInvocations](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/fragmentinvocations) — The number of times a render pass calls fragment shaders.
- [fragmentsPassed](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/fragmentspassed) — The number of fragments a render pass sends to the visibility and blend stages because they pass the scissor, depth, and stencil tests.
- [computeKernelInvocations](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/computekernelinvocations) — The number of times a pass calls any compute kernel.
