# MTLFence

*Protocol · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfence>

A synchronization mechanism that orders memory operations between GPU passes.

## Declaration

```swift
protocol MTLFence : NSObjectProtocol, Sendable
```

## Overview

Create a fence by calling the [makeFence()](https://developer.apple.com/documentation/metal/mtldevice/makefence()) method.

A fence instructs the GPU to finish running specific stages of a pass before starting stages from another pass. This is useful when a pass needs to wait before loading data from a resource until after another pass stores data to that resource. For example, to synchronize two passes where one modifies a texture and another reads it, use a fence with the following steps:

1. Encode the producing pass and update a fence after the commands that modify the texture.

2. Encode the consuming pass and wait for the same fence before the commands that read from that texture.

Apple family GPUs can update and respond to fences on a per-stage basis. This means a GPU can delay running the commands for specific stages that need to wait for another pass while it runs other stages from the same pass. For example, a GPU can run the vertex stage of a pass while the fragment stage waits until another pass updates a fence. For more information about Apple family GPUs, see the [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)) method, and the [Metal feature set tables PDF](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) or the equivalent [Metal feature set tables spreadsheet](https://developer.apple.com/metal/Metal-Feature-Set-Tables.zip).

The following encoder types support the [updateFence(_:afterEncoderStages:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/updatefence(_:afterencoderstages:)) and [waitForFence(_:beforeEncoderStages:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/waitforfence(_:beforeencoderstages:)) methods by conforming to the [MTL4CommandEncoder](https://developer.apple.com/documentation/metal/mtl4commandencoder) protocol:

- [MTL4RenderCommandEncoder](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder)

- [MTL4ComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtl4computecommandencoder)

- [MTL4MachineLearningCommandEncoder](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder)

The encoder types that inherit the [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder) protocol each have methods for updating and waiting for fences.

| Encoder types | Update fence methods | Wait for fence methods |
|---|---|---|
| [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) | [updateFence(_:after:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/updatefence(_:after:)) | [waitForFence(_:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/waitforfence(_:before:)) |
| [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) | [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/updatefence(_:)) | [waitForFence(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/waitforfence(_:)) |
| [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) | [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/updatefence(_:)) | [waitForFence(_:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/waitforfence(_:)) |
| [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) | [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/updatefence(_:)) | [waitForFence(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/waitforfence(_:)) |
| [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) | [update(_:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/update(_:)) | [wait(for:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/wait(for:)) |

> **Note:**
>  Earlier versions of Metal support hazard tracking for work you encode and commit with [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder), [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer), and [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instances, which means you don’t need to synchronize memory operations for resources with a [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlresource/hazardtrackingmode) property that’s equal to [hazardTrackingModeTracked](https://developer.apple.com/documentation/metal/mtlresourceoptions/hazardtrackingmodetracked).

### Submit producing passes before consuming passes

Send producing passes that update a fence to a queue before submitting consuming passes that wait for a fence. When encoding the producing and consuming passes into the same command buffer, encode the producing passes before the consuming passes. When submitting the producing and consuming passes in different command buffers, commit the command buffers with the producing passes before those with the consuming passes.

> **Note:**
>  When submitting multiple command buffers to an [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) at the same time, such as with its [commit:count:options:](https://developer.apple.com/documentation/metal/mtl4commandqueue/commit:count:options:) method, the method commits the command buffers in array order.

Fences can synchronize passes you submit to different queues, including [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue), [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue), or a combination of both.

> **Tip:**
>  Consider synchronizing passes that you submit to different queues with an [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) instance instead.

## Topics

### Identifying a fence
- [device](https://developer.apple.com/documentation/metal/mtlfence/device) — The device object that created the fence.
- [label](https://developer.apple.com/documentation/metal/mtlfence/label) — A string that identifies the fence.

### Selecting render stages
- [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) — The stages in a render pass that triggers a synchronization command.

## See also

### Synchronizing with barriers and fences
- [Synchronizing stages within a pass](https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass) — Block GPU stages in the a pass from running until other stages in the same pass finish.
- [Synchronizing passes with a fence](https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence) — Block GPU stages in a pass until another pass unblocks it by signaling a fence.
- [Synchronizing passes with consumer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-consumer-barriers) — Block GPU stages in a pass, and all subsequent passes, from running until stages from earlier passes finish.
- [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers) — Block GPU stages in subsequent passes from running until stages in a pass, and earlier passes, finish.
- [Synchronizing CPU and GPU work](https://developer.apple.com/documentation/metal/synchronizing-cpu-and-gpu-work) — Avoid stalls between CPU and GPU work by using multiple instances of a resource.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) — The segments of command execution within the Metal pass types.
- [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) — The stages in a render pass that triggers a synchronization command.
- [MTLBarrierScope](https://developer.apple.com/documentation/metal/mtlbarrierscope) — Describes the types of resources that a barrier operates on.
- [MTL4VisibilityOptions](https://developer.apple.com/documentation/metal/mtl4visibilityoptions) — Memory consistency options for synchronization commands.
