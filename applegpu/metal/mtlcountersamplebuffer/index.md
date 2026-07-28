# MTLCounterSampleBuffer

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplebuffer>

A specialized memory buffer that stores a GPU’s counter set data.

## Declaration

```swift
protocol MTLCounterSampleBuffer : NSObjectProtocol
```

## Overview

Create a counter sample buffer by calling an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [makeCounterSampleBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecountersamplebuffer(descriptor:)) method. See [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass).

You can store a GPU device’s counter set data only with an [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) instance that you create from the same device. See [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers) for information about storing counter sample data in a counter sample buffer.

## Topics

### Resolving the counter sample buffer’s data
- [resolveCounterRange(_:)](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/resolvecounterrange(_:)) — Transforms samples of a GPU’s counter set from the driver’s internal format to a standard Metal data structure.

### Inspecting the counter sample buffer’s configuration
- [label](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/label) — A string that identifies the counter sample buffer.
- [device](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/device) — The GPU device instance that owns the counter sample buffer.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/samplecount) — The number of samples in the buffer.

## See also

### Counter sample buffers
- [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) — Make a buffer that provides a place for a GPU to save its runtime performance metrics as it runs a pass.
- [MTLCounterSampleBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor) — A group of properties that configures the counter sample buffers you create with it.
- [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers) — Retrieve a GPU’s counter data at a time the GPU supports.
- [MTLCounterDontSample](https://developer.apple.com/documentation/metal/mtlcounterdontsample) — A sentinel value that instructs an encoder to skip sampling a counter as the GPU runs the encoder’s pass.
