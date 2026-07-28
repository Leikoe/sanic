# MTLDrawPatchIndirectArguments

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments>

The data layout required for drawing patches via indirect buffer calls.

## Declaration

```swift
struct MTLDrawPatchIndirectArguments
```

## Overview

See also the following methods:

- [drawPatches(numberOfPatchControlPoints:patchIndexBuffer:patchIndexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawpatches(numberofpatchcontrolpoints:patchindexbuffer:patchindexbufferoffset:indirectbuffer:indirectbufferoffset:))

- [drawIndexedPatches(numberOfPatchControlPoints:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedpatches(numberofpatchcontrolpoints:patchindexbuffer:patchindexbufferoffset:controlpointindexbuffer:controlpointindexbufferoffset:indirectbuffer:indirectbufferoffset:))

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments/init()) — Returns a new data layout for drawing patches via indirect buffer calls.
- [init(patchCount:instanceCount:patchStart:baseInstance:)](https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments/init(patchcount:instancecount:patchstart:baseinstance:)) — Returns a new data layout for drawing patches via indirect buffer calls, with specified parameters.

### Instance Properties
- [baseInstance](https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments/baseinstance) — The first instance to draw.
- [instanceCount](https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments/instancecount) — The number of instances to draw.
- [patchCount](https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments/patchcount) — The number of patches in each instance.
- [patchStart](https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments/patchstart) — The patch start index.

## See also

### Render compute commands
- [MTLIndirectRenderCommand](https://developer.apple.com/documentation/metal/mtlindirectrendercommand) — A render command in an indirect command buffer.
- [MTLDrawPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments) — The data layout required for drawing primitives via indirect buffer calls.
- [MTLDrawIndexedPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments) — The data layout required for drawing indexed primitives via indirect buffer calls.
