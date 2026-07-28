# dispatchThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)

*Instance Method · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(indirectbuffer:indirectbufferoffset:threadsperthreadgroup:)>

Encodes a dispatch call for a compute pass, using an indirect buffer that defines the size of a grid that aligns to threadgroup boundaries.

## Declaration

```swift
func dispatchThreadgroups(indirectBuffer: any MTLBuffer, indirectBufferOffset: Int, threadsPerThreadgroup: MTLSize)
```

## Parameters

- **indirectBuffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance providing compute parameters. Lay out the data in this buffer as described in the [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) structure.
- **indirectBufferOffset** — Where the data begins, in bytes, from the start of the buffer. This value needs to be a multiple of `4`.
- **threadsPerThreadgroup** — The number of threads in one threadgroup, in each dimension.

## Discussion

The GPU fetches parameters from the indirect buffer just before the thread grid starts. This process lets the compute function run based on GPU feedback, without latency from data transfer between the CPU and the GPU.

## See also

### Dispatching from indirect command buffers
- [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:range:)) — Encodes an instruction to run commands from an indirect buffer.
- [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
- [executeCommands(in:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:indirectbuffer:indirectbufferoffset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
- [executeCommands(in:with:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:with:)) — Encodes an instruction to run commands from an indirect buffer.
