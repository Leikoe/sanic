# sampleCounters(sampleBuffer:sampleIndex:barrier:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)>

Encodes a command to sample hardware counters, providing performance information.

## Declaration

```swift
func sampleCounters(sampleBuffer: any MTLCounterSampleBuffer, sampleIndex: Int, barrier: Bool)
```

## Parameters

- **sampleBuffer** — An [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) instance that stores the GPU hardware data.
- **sampleIndex** — An index within `sampleBuffer` the command stores the data to.
- **barrier** — Whether or not the command inserts a barrier before sampling the counter’s data. A barrier ensures that the commands you encode before this one complete before the GPU samples the hardware counters, but can negatively impact runtime performance. Running this command without a barrier means the GPU can sample counters concurrently with other commands from the encoder. The `barrier` parameter for the command has no impact on sampling commands from other passes.

## Discussion

> **Important:**
>  To use a sample buffer, it needs to be part of the [sampleBufferAttachments](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor/samplebufferattachments) on the compute pass descriptor.

See [GPU counters and counter sample buffers](https://developer.apple.com/documentation/metal/gpu-counters-and-counter-sample-buffers), [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers), and [MTLCounter](https://developer.apple.com/documentation/metal/mtlcounter) for more information.
