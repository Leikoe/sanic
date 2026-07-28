# MTLCommandBufferError.Code.stackOverflow

*Case · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/stackoverflow>

An error code that indicates the GPU terminated the command buffer because a kernel function of tile shader used too many stack frames.

## Declaration

```swift
case stackOverflow
```

## Discussion

You can set the largest number of stack frames your pipelines by configuring these properties:

- [MTLComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor)`.`[maxCallStackDepth](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/maxcallstackdepth) for kernel functions

- [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor)`.`[maxCallStackDepth](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/maxcallstackdepth) for tile shaders

## See also

### Error codes
- [MTLCommandBufferError.Code.none](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/none) — An error code that represents the absence of any problems.
- [MTLCommandBufferError.Code.timeout](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/timeout) — An error code that indicates the system interrupted and terminated the command buffer before it finished running.
- [MTLCommandBufferError.Code.pageFault](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/pagefault) — An error code that indicates the command buffer generated a page fault the GPU can’t service.
- [MTLCommandBufferError.Code.notPermitted](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/notpermitted) — An error code that indicates a process doesn’t have access to a GPU device.
- [MTLCommandBufferError.Code.outOfMemory](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/outofmemory) — An error code that indicates the GPU device doesn’t have sufficient memory to execute a command buffer.
- [MTLCommandBufferError.Code.invalidResource](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/invalidresource) — An error code that indicates the command buffer has an invalid reference to resource.
- [MTLCommandBufferError.Code.memoryless](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/memoryless) — An error code that indicates the GPU ran out of one or more of its internal resources that support memoryless render pass attachments.
- [MTLCommandBufferError.Code.deviceRemoved](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/deviceremoved) — An error code that indicates a person physically removed the GPU device before the command buffer finished running.
- [accessRevoked](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/accessrevoked) — An error code that indicates the system has revoked the Metal device’s access because it’s responsible for too many timeouts or hangs.
- [MTLCommandBufferError.Code.internal](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/internal) — An error code that indicates the Metal framework has an internal problem.
