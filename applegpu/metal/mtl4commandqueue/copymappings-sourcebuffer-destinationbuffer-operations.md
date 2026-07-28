# copyMappings(sourceBuffer:destinationBuffer:operations:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/copymappings(sourcebuffer:destinationbuffer:operations:)>

Copies multiple offsets within a source placement sparse buffer to a destination placement sparse buffer.

## Declaration

```swift
func copyMappings(sourceBuffer: any MTLBuffer, destinationBuffer: any MTLBuffer, operations: [MTL4CopySparseBufferMappingOperation])
```

## Parameters

- **sourceBuffer** — The source placement sparse [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer).
- **destinationBuffer** — The destination placement sparse [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer).
- **operations** — An array of [MTL4CopySparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsebuffermappingoperation) instances to perform.

## Discussion

You are responsible for ensuring the source destination sparse buffers have the same `placementSparsePageSize` when you create them via [makeBuffer(length:options:placementSparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:placementsparsepagesize:)).

Additionally, you are responsible for ensuring both the source and destination sparse buffers don’t use the same aliased tiles at the same time.

> **Note:**
> If a sparse texture and a sparse buffer share the same backing tiles, these don’t provide you with meaningful views of the other resource’s data.
