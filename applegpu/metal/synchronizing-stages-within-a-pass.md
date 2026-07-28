# Synchronizing stages within a pass

*Article*

<https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass>

Block GPU stages in the a pass from running until other stages in the same pass finish.

## Overview

An intrapass barrier resolves access conflicts between commands within the same pass, without affecting any other passes. When your app encodes commands that access a resource from different passes — or different stages within a single pass — it creates an access conflict when at least one command modifies that resource. This conflict happens because the GPU can run multiple commands at the same time, including those from:

- Multiple passes

- Different stages of a pass, such as the [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) and [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) stages of a compute pass

- Multiple instances of a stage, such as two or more dispatch commands within a compute pass

For more information about resource access conflicts and GPU stages, see [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization) and [MTLStages](https://developer.apple.com/documentation/metal/mtlstages), respectively.

Start by identifying which memory operations from different stages within a pass introduce a conflict. Then resolve the conflict by adding an *intrapass barrier* to pause the GPU before running the consuming stage until it finishes running the producing stage.

> **Note:**
>  An intrapass barrier has no effect on any other pass. A GPU may still be running an earlier pass, or it may begin running the next pass, or both.

### Identify access conflicts within a single pass

The following code example encodes a compute pass that has an access conflict between its copy and dispatch commands.

```swift
func encodeComputeWorkWithIntrapassBarrier(computeEncoder: MTL4ComputeCommandEncoder,
                                           argumentTable: MTL4ArgumentTable,
                                           buffers: [MTLBuffer])
{
    // Assign the argument table to the compute encoder.
    computeEncoder.setArgumentTable(argumentTable)

    // Add the buffers to the argument table.
    let bufferA = buffers[0]
    let bufferB = buffers[1]

    argumentTable.setAddress(bufferA.gpuAddress, index: 0)
    argumentTable.setAddress(bufferB.gpuAddress, index: 1)

    // Encode a copy command, which the GPU runs during the blit stage.
    computeEncoder.copy(sourceBuffer: bufferA, sourceOffset: 0,
                        destinationBuffer: bufferB, destinationOffset: 0,
                        size: copySize)

    // This method needs a barrier here.

    // Run a dispatch command that works with `bufferB`,
    // which the GPU runs during the dispatch stage.
    computeEncoder.setComputePipelineState(modifyBufferIndex1ComputePipeline)
    computeEncoder.dispatchThreadgroups(threadgroupsPerGrid: threadgroupCount,
                                        threadsPerThreadgroup: threadsPerThreadgroup)
}
```

```objective-c
- (void)encodeComputeWorkWithIntrapassBarrier:(id<MTL4ComputeCommandEncoder>)computeEncoder
                                argumentTable:(id<MTL4ArgumentTable>)argumentTable
                                      buffers:(id<MTLBuffer> *)buffers
{
    // Assign the argument table to the compute encoder.
    [computeEncoder setArgumentTable:argumentTable];

    // Add the buffers to the argument table.
    id<MTLBuffer> bufferA = buffers[0];
    id<MTLBuffer> bufferB = buffers[1];

    [argumentTable setAddress:bufferA.gpuAddress atIndex:0];
    [argumentTable setAddress:bufferB.gpuAddress atIndex:1];

    // Encode a copy command, which the GPU runs during the blit stage.
    [computeEncoder copyFromBuffer:bufferA sourceOffset:0
                          toBuffer:bufferB destinationOffset:0
                              size:copySize];

    // This method needs a barrier here.

    // Run a dispatch command that works with `bufferB`,
    // which the GPU runs during the dispatch stage.
    [computeEncoder setComputePipelineState:modifyBufferIndex1ComputePipeline];
    [computeEncoder dispatchThreadgroups:threadgroupCount
                   threadsPerThreadgroup:threadsPerThreadgroup];
}
```

The example has at least one access conflict because the pass accesses two common resources — `bufferA` and `bufferB` — from different stages, and at least one command modifies one or more of those resources.

The copy command and the dispatch commands run during the blit and dispatch stages, respectively; both commands modify `bufferB`.

![image](https://docs-assets.developer.apple.com/published/c561aebb95e82f882e2699eec5fa477c/synchronizing-stages-within-a-pass-1%402x.png)

Without a barrier, the GPU can run the commands at any time relative to each other, including at the same time, which can yield inconsistent results in resources with access conflicts.

![image](https://docs-assets.developer.apple.com/published/d51f338f01e9acac15e205d1ba45ebed/synchronizing-stages-within-a-pass-2%402x.png)

### Resolve an intrapass conflict with a barrier

Resolve access conflicts between commands within the same pass by adding an intrapass barrier with the encoder’s [barrier(afterEncoderStages:beforeEncoderStages:visibilityOptions:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterencoderstages:beforeencoderstages:visibilityoptions:)) method.

The following code example modifies the previous one adding an intrapass barrier between the blit and dispatch stages within the pass.

```swift
    // Encode a copy command, which the GPU runs during the blit stage.
    computeEncoder.copy(sourceBuffer: bufferA, sourceOffset: 0,
                        destinationBuffer: bufferB, destinationOffset: 0,
                        size: copySize)

    // Add a barrier between the copy above and the dispatch below.
    computeEncoder.barrier(afterEncoderStages: .blit,
                           beforeEncoderStages: .dispatch,
                           visibilityOptions: .device)

    // Run a dispatch command that works with `bufferB`,
    // which the GPU runs during the dispatch stage.
    computeEncoder.setComputePipelineState(modifyBufferIndex1ComputePipeline)
    computeEncoder.dispatchThreadgroups(threadgroupsPerGrid: threadgroupCount,
                                        threadsPerThreadgroup: threadsPerThreadgroup)
```

```objective-c
    // Encode a copy command, which the GPU runs during the blit stage.
    [computeEncoder copyFromBuffer:bufferA sourceOffset:0
                          toBuffer:bufferB destinationOffset:0
                              size:copySize];

    // Add a barrier between the copy above and the dispatch below.
    [computeEncoder barrierAfterEncoderStages:MTLStageBlit
                          beforeEncoderStages:MTLStageDispatch
                            visibilityOptions:MTL4VisibilityOptionDevice];

    // Run a dispatch command that works with `bufferB`,
    // which the GPU runs during the dispatch stage.
    [computeEncoder setComputePipelineState:modifyBufferIndex1ComputePipeline];
    [computeEncoder dispatchThreadgroups:threadgroupCount
                   threadsPerThreadgroup:threadsPerThreadgroup];
```

The code example adds a barrier between the blit and dispatch stages because they both access `bufferB` with load or store operations. The barrier forces the GPU to wait until the blit command completes before starting the dispatch stage.

![image](https://docs-assets.developer.apple.com/published/5730800e64956f5a9090f6ea4451e8c8/synchronizing-stages-within-a-pass-3%402x.png)

The barrier makes it so that the store operations from the blit stage’s commands finish completely before the dispatch stage’s commands load from the same memory.

### Encode commands that rely on fragment or tile stage outputs

Metal doesn’t support intrapass barriers that wait for the [tile](https://developer.apple.com/documentation/metal/mtlstages/tile) or [fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) stages on devices that have a tile-based deferred rendering (TBDR) architecture, such as Apple silicon GPUs.

> **Note:**
>  For more information about TBDR architecture, see [Tailor your apps for Apple GPUs and tile-based deferred rendering](https://developer.apple.com/documentation/metal/tailor-your-apps-for-apple-gpus-and-tile-based-deferred-rendering).

You can encode a tile dispatch that depends on the results of a previous tile dispatch because tile compute dispatches can access data from anywhere within the same tile. Similarly, you can encode a draw command that depends on the results of a previous draw command’s fragment stage because fragment shaders can only access data at their specific pixel location. However, if a tile dispatch needs results from another tile, or a fragment shader needs results from another fragment, then start a new render pass and synchronize them with a barrier.

For example, to synchronize the two passes by adding a consumer-based queue barrier in the new pass:

1. End the current render pass by calling the encoder’s [endEncoding()](https://developer.apple.com/documentation/metal/mtl4commandencoder/endencoding()) method.

2. Start a new render pass by creating a new render encoder from the command buffer, or another bound for the queue.

3. Add a consumer barrier by calling the new encoder’s [barrier(afterQueueStages:beforeStages:visibilityOptions:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterqueuestages:beforestages:visibilityoptions:)) method, which synchronizes the results of the previous render pass.

Similarly, to create a producer-based queue barrier in a pass:

1. Add a producer barrier by calling the encoder’s [barrier(afterStages:beforeQueueStages:visibilityOptions:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterstages:beforequeuestages:visibilityoptions:)) method to synchronize the results of the current render pass.

2. End the current render pass by calling the encoder’s [endEncoding()](https://developer.apple.com/documentation/metal/mtl4commandencoder/endencoding()) method.

3. Start a new render pass by creating a new encoder from the command buffer, or another bound for the queue.

Alternatively, use an [MTLFence](https://developer.apple.com/documentation/metal/mtlfence):

1. Update a fence in the current render pass by calling the encoder’s [updateFence(_:afterEncoderStages:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/updatefence(_:afterencoderstages:)) method.

2. End the current render pass by calling the encoder’s [endEncoding()](https://developer.apple.com/documentation/metal/mtl4commandencoder/endencoding()) method.

3. Start a new render pass by creating a new encoder from the same command buffer.

4. Wait for the same fence instance in the new render pass by calling the new encoder’s [waitForFence(_:beforeEncoderStages:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/waitforfence(_:beforeencoderstages:)) method.

For more information about other synchronization mechanisms, see these articles in the series:

- [Synchronizing passes with a fence](https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence)

- [Synchronizing passes with consumer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-consumer-barriers)

- [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers)

## See also

### Synchronizing with barriers and fences
- [Synchronizing passes with a fence](https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence) — Block GPU stages in a pass until another pass unblocks it by signaling a fence.
- [Synchronizing passes with consumer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-consumer-barriers) — Block GPU stages in a pass, and all subsequent passes, from running until stages from earlier passes finish.
- [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers) — Block GPU stages in subsequent passes from running until stages in a pass, and earlier passes, finish.
- [Synchronizing CPU and GPU work](https://developer.apple.com/documentation/metal/synchronizing-cpu-and-gpu-work) — Avoid stalls between CPU and GPU work by using multiple instances of a resource.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) — The segments of command execution within the Metal pass types.
- [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) — A synchronization mechanism that orders memory operations between GPU passes.
- [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) — The stages in a render pass that triggers a synchronization command.
- [MTLBarrierScope](https://developer.apple.com/documentation/metal/mtlbarrierscope) — Describes the types of resources that a barrier operates on.
- [MTL4VisibilityOptions](https://developer.apple.com/documentation/metal/mtl4visibilityoptions) — Memory consistency options for synchronization commands.
