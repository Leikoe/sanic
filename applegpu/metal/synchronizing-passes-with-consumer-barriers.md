# Synchronizing passes with consumer barriers

*Article*

<https://developer.apple.com/documentation/metal/synchronizing-passes-with-consumer-barriers>

Block GPU stages in a pass, and all subsequent passes, from running until stages from earlier passes finish.

## Overview

Consumer queue barriers are coarse synchronization primitives that resolve access conflicts between commands in different passes that you submit to the same command queue, including the passes from other command buffers you submit to the same queue. Consumer barriers are convenient for synchronizing passes that load from common resources that multiple, earlier passes modify in the same queue.

> **Note:**
>  You can also add consumer barriers with Metal 3 encoder types.

When your app encodes commands that access a resource from different passes — or different stages within a single pass — it creates an access conflict when at least one command modifies that resource. This conflict happens because the GPU can run multiple commands at the same time, including those from:

- Multiple passes

- Different stages of a pass, such as the [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) and [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) stages of a compute pass

- Multiple instances of a stage, such as two or more dispatch commands within a compute pass

For more information about resource access conflicts and GPU stages, see [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization) and [MTLStages](https://developer.apple.com/documentation/metal/mtlstages), respectively.

Start by identifying which memory operations from previous passes in the same queue introduce a conflict and resolve it with a consumer queue barrier in the consumer pass.

> **Tip:**
>  As an alternative for Metal 4 queues, create a single producer queue barrier in the producing pass that’s the equivalent of multiple consumer queue barriers for applicable scenarios. For more information, see [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers).

### Identify access conflicts on the same queue

The following code example encodes three compute passes. The first pass runs a single copy command:

```swift
func encodeComputeWorkWithConsumerBarrier(commandBuffer: MTL4CommandBuffer,
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

    // Finalize the first compute pass.
    computeEncoder1.endEncoding()
```

```objective-c
- (void)encodeComputeWorkWithConsumerBarrier:(id<MTL4CommandBuffer>)commandBuffer
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

    // Finalize the first compute pass.
    [computeEncoder1 endEncoding];
```

The second pass runs a copy command and a dispatch command:

```swift
    // === Encode pass 2 ===

    // Create an encoder for the second compute pass.
    let computeEncoder2: MTL4ComputeCommandEncoder!
    computeEncoder2 = commandBuffer.makeComputeCommandEncoder()

    // Assign the argument table to the compute encoder.
    computeEncoder2.setArgumentTable(argumentTable)

    // Copy from `bufferC` to `bufferD`, which runs during the blit stage.
    let bufferC = buffers[2]
    let bufferD = buffers[3]
    argumentTable.setAddress(bufferC.gpuAddress, index: 2)
    argumentTable.setAddress(bufferD.gpuAddress, index: 3)
    computeEncoder2.copy(sourceBuffer: bufferC, sourceOffset: 0,
                         destinationBuffer: bufferD, destinationOffset: 0,
                         size: copySize)

    // Pass 2 needs to add a consumer barrier here because the dispatch stage
    // in pass 2 and 3 need to wait for the blit stage in pass 1 to finish.

    // Run a dispatch command that works with `bufferB`,
    // which the GPU runs during the dispatch stage.
    computeEncoder2.setComputePipelineState(modifyBufferIndex1ComputePipeline)
    computeEncoder2.dispatchThreadgroups(threadgroupsPerGrid: threadgroupCount,
                                         threadsPerThreadgroup: threadsPerThreadgroup)

    // Finalize the second compute pass.
    computeEncoder2.endEncoding()
```

```objective-c
    // === Encode pass 2 ===

    // Create an encoder for the second compute pass.
    id<MTL4ComputeCommandEncoder> computeEncoder2;
    computeEncoder2 = [commandBuffer computeCommandEncoder];

    // Assign the argument table to the compute encoder.
    [computeEncoder2 setArgumentTable:argumentTable];

    // Copy from `bufferC` to `bufferD`, which runs during the blit stage.
    id<MTLBuffer> bufferC = buffers[2];
    id<MTLBuffer> bufferD = buffers[3];
    [argumentTable setAddress:bufferC.gpuAddress atIndex:2];
    [argumentTable setAddress:bufferD.gpuAddress atIndex:3];
    [computeEncoder2 copyFromBuffer:bufferC sourceOffset:0
                           toBuffer:bufferD destinationOffset:0
                               size:copySize];

    // Pass 2 needs to add a consumer barrier here because the dispatch stage
    // in pass 2 and 3 need to wait for the blit stage in pass 1 to finish.

    // Run a dispatch command that works with `bufferB`,
    // which the GPU runs during the dispatch stage.
    [computeEncoder2 setComputePipelineState:modifyBufferIndex1ComputePipeline];
    [computeEncoder2 dispatchThreadgroups:threadgroupCount
                    threadsPerThreadgroup:threadsPerThreadgroup];

    // Finalize the second compute pass.
    [computeEncoder2 endEncoding];
```

The third pass runs a single dispatch command:

```swift
    // === Encode pass 3 ===

    // Create an encoder for the third compute pass.
    let computeEncoder3: MTL4ComputeCommandEncoder!
    computeEncoder3 = commandBuffer.makeComputeCommandEncoder()

    // Assign the argument table to the compute encoder.
    computeEncoder3.setArgumentTable(argumentTable)

    // Run a dispatch command that works with `bufferE`,
    // which the GPU runs during the dispatch stage.
    let bufferE = buffers[4]
    argumentTable.setAddress(bufferE.gpuAddress, index: 4)
    computeEncoder3.setComputePipelineState(modifyBufferIndex4ComputePipeline)
    computeEncoder3.dispatchThreadgroups(threadgroupsPerGrid: threadgroupCount,
                                         threadsPerThreadgroup: threadsPerThreadgroup)

    // Finalize the third compute pass.
    computeEncoder3.endEncoding()
}
```

```objective-c
    // === Encode pass 3 ===

    // Create an encoder for the third compute pass.
    id<MTL4ComputeCommandEncoder> computeEncoder3;
    computeEncoder3 = [commandBuffer computeCommandEncoder];

    // Assign the argument table to the compute encoder.
    [computeEncoder3 setArgumentTable:argumentTable];

    // Run a dispatch command that works with `bufferE`,
    // which the GPU runs during the dispatch stage.
    id<MTLBuffer> bufferE = buffers[4];
    [argumentTable setAddress:bufferE.gpuAddress atIndex:4];
    [computeEncoder3 setComputePipelineState:modifyBufferIndex4ComputePipeline];
    [computeEncoder3 dispatchThreadgroups:threadgroupCount
                    threadsPerThreadgroup:threadsPerThreadgroup];

    // Finalize the third compute pass.
    [computeEncoder3 endEncoding];
}
```

The example has at least one access conflict because passes 1 and 2 both access a common resource, `bufferB`:

- The copy command from the first pass stores to `bufferB`.

- The dispatch command from the second pass loads from `bufferB`.

![image](https://docs-assets.developer.apple.com/published/a36dda87b8401978c2b0be48e734807e/synchronizing-passes-with-consumer-barriers-1%402x.png)

Without synchronization, the GPU can run all three passes and their stages in parallel, which can yield inconsistent results in resources with access conflicts.

![image](https://docs-assets.developer.apple.com/published/8cdfc94b96ac5fddce92708f21b537f6/synchronizing-passes-with-consumer-barriers-2%402x.png)

### Resolve access conflicts with a consumer barrier

Resolve access conflicts between passes from the same command queue with a consumer barrier by calling the encoder’s [barrier(afterQueueStages:beforeStages:visibilityOptions:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterqueuestages:beforestages:visibilityoptions:)) method.

Each consumer queue barrier temporarily blocks the GPU from running the specific stage types you pass to the `beforeStages` parameter in the current pass, and all subsequent passes in the same queue. The barrier unblocks those stages when all the stage types you pass to the `afterQueueStages` parameter finish running in all previous passes.

> **Important:**
>  The stages you pass to the `beforeStages` parameter of the [barrier(afterQueueStages:beforeStages:visibilityOptions:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterqueuestages:beforestages:visibilityoptions:)) method apply to the pass you’re encoding and all subsequent passes, but the stages of the `afterQueueStages` parameter only apply to previous passes.

The following example modifies the code that encodes the second pass by adding a consumer queue barrier just before the dispatch command stage in the second pass:

```swift
    // === Encode pass 2 ===

    // Create an encoder for the second compute pass.
    let computeEncoder2: MTL4ComputeCommandEncoder!
    computeEncoder2 = commandBuffer.makeComputeCommandEncoder()

    // Assign the argument table to the compute encoder.
    computeEncoder2.setArgumentTable(argumentTable)

    // Copy from `bufferC` to `bufferD`, which runs during the blit stage.
    let bufferC = buffers[2]
    let bufferD = buffers[3]
    argumentTable.setAddress(bufferC.gpuAddress, index: 2)
    argumentTable.setAddress(bufferD.gpuAddress, index: 3)
    computeEncoder2.copy(sourceBuffer: bufferC, sourceOffset: 0,
                         destinationBuffer: bufferD, destinationOffset: 0,
                         size: copySize)

    // Add a consumer queue barrier that blocks any dispatch stages in subsequent passes
    // in the queue, including this one, from running until blit stages in all
    // previous passes finish running, not counting this one.
    computeEncoder2.barrier(afterQueueStages: .blit,
                            beforeStages: .dispatch,
                            visibilityOptions: .device)

    // Run a dispatch command that works with `bufferB`,
    // which the GPU runs during the dispatch stage.
    computeEncoder2.setComputePipelineState(modifyBufferIndex1ComputePipeline)
    computeEncoder2.dispatchThreadgroups(threadgroupsPerGrid: threadgroupCount,
                                         threadsPerThreadgroup: threadsPerThreadgroup)

    // Finalize the second compute pass.
    computeEncoder2.endEncoding()
```

```objective-c
    // === Encode pass 2 ===

    // Create an encoder for the second compute pass.
    id<MTL4ComputeCommandEncoder> computeEncoder2;
    computeEncoder2 = [commandBuffer computeCommandEncoder];

    // Assign the argument table to the compute encoder.
    [computeEncoder2 setArgumentTable:argumentTable];

    // Copy from `bufferC` to `bufferD`, which runs during the blit stage.
    id<MTLBuffer> bufferC = buffers[2];
    id<MTLBuffer> bufferD = buffers[3];
    [argumentTable setAddress:bufferC.gpuAddress atIndex:2];
    [argumentTable setAddress:bufferD.gpuAddress atIndex:3];
    [computeEncoder2 copyFromBuffer:bufferC sourceOffset:0
                           toBuffer:bufferD destinationOffset:0
                               size:copySize];

    // Add a consumer queue barrier that blocks any dispatch stages in subsequent passes
    // in the queue, including this one, from running until blit stages in all
    // previous passes finish running, not counting this one.
    [computeEncoder2 barrierAfterQueueStages:MTLStageBlit
                                beforeStages:MTLStageDispatch
                           visibilityOptions:MTL4VisibilityOptionDevice];

    // Run a dispatch command that works with `bufferB`,
    // which the GPU runs during the dispatch stage.
    [computeEncoder2 setComputePipelineState:modifyBufferIndex1ComputePipeline];
    [computeEncoder2 dispatchThreadgroups:threadgroupCount
                    threadsPerThreadgroup:threadsPerThreadgroup];

    // Finalize the second compute pass.
    [computeEncoder2 endEncoding];
```

In this example, the barrier prevents the GPU from running the dispatch stage in both the second and third passes until the blit stage in the first pass finishes storing its modifications.

![image](https://docs-assets.developer.apple.com/published/62e47c0f7819872e5b41819b52cea7b5/synchronizing-passes-with-consumer-barriers-3%402x.png)

The barrier unblocks both dispatch stages when the blit stage from the first pass finishes running because it’s the only pass that applies to the `afterQueueStages` parameter.

For more information about other synchronization mechanisms, see these articles in the series:

- [Synchronizing stages within a pass](https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass)

- [Synchronizing passes with a fence](https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence)

- [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers)

## See also

### Synchronizing with barriers and fences
- [Synchronizing stages within a pass](https://developer.apple.com/documentation/metal/synchronizing-stages-within-a-pass) — Block GPU stages in the a pass from running until other stages in the same pass finish.
- [Synchronizing passes with a fence](https://developer.apple.com/documentation/metal/synchronizing-passes-with-a-fence) — Block GPU stages in a pass until another pass unblocks it by signaling a fence.
- [Synchronizing passes with producer barriers](https://developer.apple.com/documentation/metal/synchronizing-passes-with-producer-barriers) — Block GPU stages in subsequent passes from running until stages in a pass, and earlier passes, finish.
- [Synchronizing CPU and GPU work](https://developer.apple.com/documentation/metal/synchronizing-cpu-and-gpu-work) — Avoid stalls between CPU and GPU work by using multiple instances of a resource.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) — The segments of command execution within the Metal pass types.
- [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) — A synchronization mechanism that orders memory operations between GPU passes.
- [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) — The stages in a render pass that triggers a synchronization command.
- [MTLBarrierScope](https://developer.apple.com/documentation/metal/mtlbarrierscope) — Describes the types of resources that a barrier operates on.
- [MTL4VisibilityOptions](https://developer.apple.com/documentation/metal/mtl4visibilityoptions) — Memory consistency options for synchronization commands.
