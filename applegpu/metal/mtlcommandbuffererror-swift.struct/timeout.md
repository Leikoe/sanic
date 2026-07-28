# timeout

*Type Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/timeout>

An error code that indicates the system interrupted and terminated the command buffer before it finished running.

## Declaration

```swift
static var timeout: MTLCommandBufferError.Code { get }
```

## Discussion

Possible causes include:

- The commands in the buffer took more time to run than the system allows.

- The command buffer timed out waiting for another workload to signal an [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent).

## See also

### Errors codes
- [none](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/none) — An error code that represents the absence of any problems.
- [pageFault](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/pagefault) — An error code that indicates the command buffer generated a page fault the GPU can’t service.
- [notPermitted](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/notpermitted) — An error code that indicates a process doesn’t have access to a GPU device.
- [outOfMemory](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/outofmemory) — An error code that indicates the GPU device doesn’t have sufficient memory to execute a command buffer.
- [invalidResource](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/invalidresource) — An error code that indicates the command buffer has an invalid reference to resource.
- [memoryless](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/memoryless) — An error code that indicates the GPU ran out of one or more of its internal resources that support memoryless render pass attachments.
- [deviceRemoved](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/deviceremoved) — An error code that indicates a person physically removed the GPU device before the command buffer finished running.
- [stackOverflow](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/stackoverflow) — An error code that indicates the GPU terminated the command buffer because a kernel function of tile shader used too many stack frames.
- [accessRevoked](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/accessrevoked) — An error code that indicates the system has revoked the Metal device’s access because it’s responsible for too many timeouts or hangs.
- [internal](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/internal) — An error code that indicates the Metal framework has an internal problem.
- [MTLCommandBufferError.Code](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code) — Error codes that indicate why a GPU is unable to finish running a command buffer.
