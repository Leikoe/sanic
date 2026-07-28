# resolveCounters(_:range:destinationBuffer:destinationOffset:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resolvecounters(_:range:destinationbuffer:destinationoffset:)>

Encodes a command that resolves the data from the samples in a sample counter buffer and stores the results into a buffer.

## Declaration

```swift
func resolveCounters(_ sampleBuffer: any MTLCounterSampleBuffer, range: Range<Int>, destinationBuffer: any MTLBuffer, destinationOffset: Int)
```

## Parameters

- **sampleBuffer** — A counter sample buffer source that contains the sample data.
- **range** — A range that indicates which of the buffer’s samples the command resolves.
- **destinationBuffer** — A destination buffer where the command stores the data it resolves.
- **destinationOffset** — A starting offset, in bytes, within `destinationBuffer` where the blit pass writes the first byte of the data it resolves.

## Discussion

For an example of how and when to use this method, see [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format).

> **Note:**
>  The GPU stores [MTLCounterErrorValue](https://developer.apple.com/documentation/metal/mtlcountererrorvalue) in `destinationBuffer` each time it encounters an error resolving a sample.

## See also

### Sampling counters
- [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) — Encodes a command that samples the GPU’s hardware counters during a blit pass and stores the data in a counter sample buffer.
