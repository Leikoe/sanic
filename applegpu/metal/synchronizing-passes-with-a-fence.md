# Synchronizing passes with a fence

*Article*

<https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence>

Block GPU stages in a pass until another pass unblocks it by signaling a fence.

## Overview

A fence resolves access conflicts between commands in different passes that you submit to the same command queue, including the passes you commit in other command buffers.

> **Note:**
>  [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) instances in Metal 3 work across multiple command queues that belong to the same device; to synchronize across multiple command queues in Metal 4, use [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) or [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) instances.

When your app encodes commands that access a resource from different passes — or different stages within a single pass — it creates an access conflict when at least one command modifies that resource. This conflict happens because the GPU can run multiple commands at the same time, including those from:

- Multiple passes

- Different stages of a pass, such as the [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) and [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) stages of a compute pass

- Multiple instances of a stage, such as two or more dispatch commands within a compute pass

For more information about resource access conflicts and GPU stages, see [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization) and [MTLStages](https://developer.apple.com/documentation/metal/mtlstages), respectively.

> **Important:**
> To synchronize stages within the same pass, use an *intrapass barrier* instead of a fence because fences can only synchronize between stages of different passes.

For more information about synchronizing within a single pass, see [Synchronizing stages within a pass](https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass).

Start by identifying which memory operations from different passes introduce a conflict and resolve it with a fence:

1. Update a fence in the producing pass.

2. Wait for that fence in the consuming pass.

> **Note:**
>  Create an [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) by calling the [makeFence()](https://developer.apple.com/documentation/metal/mtldevice/makefence()) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance.

### Identify access conflicts between two or more passes

The following code example encodes two compute passes. The first encoder creates a pass with a copy command and a dispatch command:

```swift
func encodeComputeWorkWithFence(fence: MTLFence,
                                commandBuffer: MTL4CommandBuffer,
                                argumentTable: MTL4ArgumentTable,
                                buffers: [MTLBuffer])
{
    // === Encode pass 1 ===

    // Create an encoder for the first compute pass.
    let computeEncoder1: MTL4ComputeCommandEncoder!
    computeEncoder1 = commandBuffer.makeComputeCommandEncoder()

    // Assign the argument table to the compute encoder.
    computeEncoder1.setArgumentTable(argumentTable)

    // Add the buffers to the argument table for the dispatch command.
    let bufferA = buffers[0]
    let bufferB = buffers[1]

    argumentTable.setAddress(bufferA.gpuAddress, index: 0)
    argumentTable.setAddress(bufferB.gpuAddress, index: 1)

    // Copy from `bufferA` to `bufferB`, which runs during the blit stage.
    computeEncoder1.copy(sourceBuffer: bufferA, sourceOffset: 0,
                        destinationBuffer: bufferB, destinationOffset: 0,
                        size: copySize)

    // Run a dispatch command that modifies `bufferC`,
    // which the GPU runs during the dispatch stage.
    let bufferC = buffers[2]
    argumentTable.setAddress(bufferC.gpuAddress, index: 2)
    computeEncoder1.setComputePipelineState(modifyBufferIndex2ComputePipeline)
    computeEncoder1.dispatchThreadgroups(threadgroupsPerGrid: threadgroupCount,
                                         threadsPerThreadgroup: threadsPerThreadgroup)

    // Pass 1 needs to update a fence here.

    // Finalize the first compute pass.
    computeEncoder1.endEncoding()
```

```objective-c
- (void)encodeComputeWorkWithFence:(id<MTLFence>)fence
                     commandBuffer:(id<MTL4CommandBuffer>)commandBuffer
                     argumentTable:(id<MTL4ArgumentTable>)argumentTable
                           buffers:(id<MTLBuffer> *)buffers
{
    // === Encode pass 1 ===

    // Create an encoder for the first compute pass.
    id<MTL4ComputeCommandEncoder> computeEncoder1;
    computeEncoder1 = [commandBuffer computeCommandEncoder];

    // Assign the argument table to the compute encoder.
    [computeEncoder1 setArgumentTable:argumentTable];

    // Add the buffers to the argument table for the dispatch command.
    id<MTLBuffer> bufferA = buffers[0];
    id<MTLBuffer> bufferB = buffers[1];

    [argumentTable setAddress:bufferA.gpuAddress atIndex:0];
    [argumentTable setAddress:bufferB.gpuAddress atIndex:1];

    // Copy from `bufferA` to `bufferB`, which runs during the blit stage.
    [computeEncoder1 copyFromBuffer:bufferA sourceOffset:0
                           toBuffer:bufferB destinationOffset:0
                               size:copySize];

    // Run a dispatch command that modifies `bufferC`,
    // which the GPU runs during the dispatch stage.
    id<MTLBuffer> bufferC = buffers[2];
    [argumentTable setAddress:bufferC.gpuAddress atIndex:2];
    [computeEncoder1 setComputePipelineState:modifyBufferIndex2ComputePipeline];
    [computeEncoder1 dispatchThreadgroups:threadgroupCount
                    threadsPerThreadgroup:threadsPerThreadgroup];

    // Pass 1 needs to update a fence here.

    // Finalize the first compute pass.
    [computeEncoder1 endEncoding];
```

The second encoder also creates a pass with a copy command and a dispatch command:

```swift
    // === Encode pass 2 ===

    // Create an encoder for the second compute pass.
    let computeEncoder2: MTL4ComputeCommandEncoder!
    computeEncoder2 = commandBuffer.makeComputeCommandEncoder()

    // Assign the argument table to the compute encoder.
    computeEncoder2.setArgumentTable(argumentTable)

    // Pass 2 needs to wait for a fence here.

    // Copy from `bufferC` to `bufferD`, which runs during the blit stage.
    let bufferD = buffers[3]
    argumentTable.setAddress(bufferD.gpuAddress, index: 3)
    computeEncoder2.copy(sourceBuffer: bufferC, sourceOffset: 0,
                         destinationBuffer: bufferD, destinationOffset: 0,
                         size: copySize)

    // Run a dispatch command that works with `bufferE`.
    let bufferE = buffers[4]
    argumentTable.setAddress(bufferE.gpuAddress, index: 4)
    computeEncoder2.setComputePipelineState(modifyBufferIndex4ComputePipeline)
    computeEncoder2.dispatchThreadgroups(threadgroupsPerGrid: threadgroupCount,
                                         threadsPerThreadgroup: threadsPerThreadgroup)

    // Finalize the second compute pass.
    computeEncoder2.endEncoding()
}
```

```objective-c
    // === Encode pass 2 ===

    // Create an encoder for the second compute pass.
    id<MTL4ComputeCommandEncoder> computeEncoder2;
    computeEncoder2 = [commandBuffer computeCommandEncoder];

    // Assign the argument table to the compute encoder.
    [computeEncoder2 setArgumentTable:argumentTable];

    // Pass 2 needs to wait for a fence here.

    // Copy from `bufferC` to `bufferD`, which runs during the blit stage.
    id<MTLBuffer> bufferD = buffers[3];
    [argumentTable setAddress:bufferD.gpuAddress atIndex:3];
    [computeEncoder2 copyFromBuffer:bufferC sourceOffset:0
                           toBuffer:bufferD destinationOffset:0
                               size:copySize];

    // Run a dispatch command that works with `bufferE`.
    id<MTLBuffer> bufferE = buffers[4];
    [argumentTable setAddress:bufferE.gpuAddress atIndex:4];
    [computeEncoder2 setComputePipelineState:modifyBufferIndex4ComputePipeline];
    [computeEncoder2 dispatchThreadgroups:threadgroupCount
                    threadsPerThreadgroup:threadsPerThreadgroup];

    // Finalize the second compute pass.
    [computeEncoder2 endEncoding];
}
```

The example has at least one access conflict because both passes access a common resource, `bufferC`:

- The dispatch command from the first pass stores to `bufferC`.

- The copy command from the second pass loads from `bufferC`.

![image](https://docs-assets.developer.apple.com/published/28d5ebee107bbb409190cbb05e26c1ed/synchronizing-passes-with-a-fence-1%402x.png)

Without synchronization, the GPU can run both passes and their stages in parallel, which can yield inconsistent results in resources with access conflicts.

![image](https://docs-assets.developer.apple.com/published/4d564ff96353f496ea9e9c3e0977a89d/synchronizing-passes-with-a-fence-2%402x.png)

### Resolve an access conflict between passes with a fence

Resolve access conflicts between passes from the same command queue with an [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) instance by:

- Instructing the producing pass to signal a pass that’s waiting for a fence by calling the encoder’s [updateFence(_:afterEncoderStages:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/updatefence(_:afterencoderstages:)) method.

- Instructing the consuming pass to wait for the fence by calling the encoder’s [waitForFence(_:beforeEncoderStages:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/waitforfence(_:beforeencoderstages:)) method.

The GPU pauses before running the commands you encode in the consuming pass after the wait command until the GPU runs all update commands you encode for the same fence in the other relevant, producing passes.

> **Tip:**
> To get the best runtime performance in passes that update or wait for a fence, encode them as close as possible to the commands that introduce resource access conflicts.

The following code example modifies the code for the first pass by adding a call that updates the fence:

```swift
    // Run a dispatch command that modifies `bufferC`,
    // which the GPU runs during the dispatch stage.
    let bufferC = buffers[2]
    argumentTable.setAddress(bufferC.gpuAddress, index: 2)
    computeEncoder1.setComputePipelineState(modifyBufferIndex2ComputePipeline)
    computeEncoder1.dispatchThreadgroups(threadgroupsPerGrid: threadgroupCount,
                                         threadsPerThreadgroup: threadsPerThreadgroup)

    // Unblock the pass 2 that's waiting on the fence by
    // updating it when the dispatch stage (of pass 1) is done.
    computeEncoder1.updateFence(fence, afterEncoderStages: .dispatch)

    // Finalize the first compute pass.
    computeEncoder1.endEncoding()
```

```objective-c
    // Run a dispatch command that modifies `bufferC`,
    // which the GPU runs during the dispatch stage.
    id<MTLBuffer> bufferC = buffers[2];
    [argumentTable setAddress:bufferC.gpuAddress atIndex:2];
    [computeEncoder1 setComputePipelineState:modifyBufferIndex2ComputePipeline];
    [computeEncoder1 dispatchThreadgroups:threadgroupCount
                    threadsPerThreadgroup:threadsPerThreadgroup];

    // Unblock the pass 2 that's waiting on the fence by
    // updating it when the dispatch stage (of pass 1) is done.
    [computeEncoder1 updateFence:fence afterEncoderStages:MTLStageDispatch];

    // Finalize the first compute pass.
    [computeEncoder1 endEncoding];
```

The following code example modifies the code for the second pass by adding a call that waits for the fence.

```swift
    // Assign the argument table to the compute encoder.
    computeEncoder2.setArgumentTable(argumentTable)

    // Wait for pass 1 to update the fence.
    computeEncoder2.waitForFence(fence, beforeEncoderStages: .blit)

    // Copy from `bufferC` to `bufferD`, which runs during the blit stage.
    let bufferD = buffers[3]
    argumentTable.setAddress(bufferD.gpuAddress, index: 3)
    computeEncoder2.copy(sourceBuffer: bufferC, sourceOffset: 0,
                         destinationBuffer: bufferD, destinationOffset: 0,
                         size: copySize)
```

```objective-c
    // Assign the argument table to the compute encoder.
    [computeEncoder2 setArgumentTable:argumentTable];

    // Wait for pass 1 to update the fence.
    [computeEncoder2 waitForFence:fence beforeEncoderStages:MTLStageBlit];

    // Copy from `bufferC` to `bufferD`, which runs during the blit stage.
    id<MTLBuffer> bufferD = buffers[3];
    [argumentTable setAddress:bufferD.gpuAddress atIndex:3];
    [computeEncoder2 copyFromBuffer:bufferC sourceOffset:0
                           toBuffer:bufferD destinationOffset:0
                               size:copySize];
```

The fence forces the GPU to wait before it runs the blit stage of the second pass until the dispatch stage of the first pass finishes storing its modifications to the underlying memory for `bufferC`.

![image](https://docs-assets.developer.apple.com/published/7d54c4f8d610a94a846e38d6536472c8/synchronizing-passes-with-a-fence-3%402x.png)

You can reuse a fence instance to resolve resource access conflicts in subsequent commands after encoding a wait command for a pass.

> **Important:**
>  To reuse a fence within the same pass, encode the wait command first, then encode the update command.

For more information about other synchronization mechanisms, see these articles in the series:

- [Synchronizing stages within a pass](https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass)

- [Synchronizing passes with consumer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-consumer-barriers)

- [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers)

## See also

### Synchronizing with barriers and fences
- [Synchronizing stages within a pass](https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass) — Block GPU stages in the a pass from running until other stages in the same pass finish.
- [Synchronizing passes with consumer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-consumer-barriers) — Block GPU stages in a pass, and all subsequent passes, from running until stages from earlier passes finish.
- [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers) — Block GPU stages in subsequent passes from running until stages in a pass, and earlier passes, finish.
- [Synchronizing CPU and GPU work](https://developer.apple.com/documentation/metal/synchronizing-cpu-and-gpu-work) — Avoid stalls between CPU and GPU work by using multiple instances of a resource.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) — The segments of command execution within the Metal pass types.
- [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) — A synchronization mechanism that orders memory operations between GPU passes.
- [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) — The stages in a render pass that triggers a synchronization command.
- [MTLBarrierScope](https://developer.apple.com/documentation/metal/mtlbarrierscope) — Describes the types of resources that a barrier operates on.
- [MTL4VisibilityOptions](https://developer.apple.com/documentation/metal/mtl4visibilityoptions) — Memory consistency options for synchronization commands.
