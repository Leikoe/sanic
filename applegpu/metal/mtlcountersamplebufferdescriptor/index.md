# MTLCounterSampleBufferDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor>

A group of properties that configures the counter sample buffers you create with it.

## Declaration

```swift
class MTLCounterSampleBufferDescriptor
```

## Overview

To create a new counter sample buffer, create and configure an [MTLCounterSampleBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor) instance, and then call an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [makeCounterSampleBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecountersamplebuffer(descriptor:)) method. See [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass).

Each new sample counter buffer inherits the values of the descriptor’s properties when you create it. You can modify a descriptor and reuse it to create other counter sample buffers, which has no effect on existing counter sample buffers.

## Topics

### Configuring a descriptor for a counter sample buffer
- [counterSet](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/counterset) — A GPU device’s counter set instance that you want to sample.
- [label](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/label) — The name for the counter sample buffer you create with the descriptor.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/samplecount) — The number of instances of a counter set’s data that a counter sample buffer can store.
- [storageMode](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/storagemode) — The memory storage mode for the counter sample buffers you create with the descriptor.

## See also

### Counter sample buffers
- [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) — Make a buffer that provides a place for a GPU to save its runtime performance metrics as it runs a pass.
- [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) — A specialized memory buffer that stores a GPU’s counter set data.
- [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers) — Retrieve a GPU’s counter data at a time the GPU supports.
- [MTLCounterDontSample](https://developer.apple.com/documentation/metal/mtlcounterdontsample) — A sentinel value that instructs an encoder to skip sampling a counter as the GPU runs the encoder’s pass.
