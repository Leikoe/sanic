# Converting a GPU’s counter data into a readable format

*Article*

<https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format>

Inspect and use the data within a GPU’s counter sample buffer by resolving it into a standard format.

## Overview

To use the data a GPU driver stores in an [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) instance (see [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers)), your app needs to *resolve* it. Resolving the data converts the counter data from the GPU’s internal data structure into a common format that Metal defines.

You can resolve the data in a counter sample buffer by creating a blit pass that converts the data as it copies it to an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer). If the CPU can access a counter sample buffer, you can also resolve the data after the GPU finishes running a command buffer. See [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) for information about making a CPU-accessible counter sample buffer.

### Resolve the counter sample buffer with the CPU

For an [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) instance that you create with shared memory (see [storageMode](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/storagemode) and [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared)), you can resolve the data by calling its [resolveCounterRange(_:)](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/resolvecounterrange(_:)) method.

```swift
/// Converts the contents of the counter sample buffer into an array of result timestamps.
func resolveSampleBuffer() {
    /// Represents the size of the counter sample buffer.
    let range = 0..<sampleCount

    // Convert the contents of the counter sample buffer into the standard data format.
    guard let data = try? counterSampleBuffer.resolveCounterRange(range) else {
        return
    }
    ...
}
```

```objective-c
/// Converts the contents of the counter sample buffer into an array of result timestamps.
- (void) resolveSampleBuffer
{
    /// Represents the size of the counter sample buffer.
    NSRange range = NSMakeRange(0, self.sampleCount);

    // Convert the contents of the counter sample buffer into the standard data format.
    NSData* data = [self.counterSampleBuffer resolveCounterRange:range];
    if (nil == data) {
        return;
    }
    ...
}
```

You can resolve a sample counter buffer with the CPU at any time after the GPU finishes running the pass that retrieves the counter’s data. To access the data as soon as possible (with the CPU), add a completion handler to the pass’s command buffer by calling its [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) method.

```swift
commandBuffer.addCompletedHandler { commandBuffer in
    let timestamps = func resolveSampleBuffer() {
    ...
}
```

```objective-c
[commandBuffer addCompletedHandler:^(id<MTLCommandBuffer> _Nonnull commandBuffer) {
    [self resolveSampleBuffer];
    ...
}];
```

### Resolve the counter sample buffer with a blit pass on the GPU

You can also resolve an [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) instance’s data into an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) by running a blit pass on the GPU. For some GPUs, this technique is the only way to resolve a counter sample buffer that uses private storage (see [storageMode](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/storagemode) and [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private)).

To resolve a sample counter buffer in a blit pass, create an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) instance and call its [resolveCounters(_:range:destinationBuffer:destinationOffset:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resolvecounters(_:range:destinationbuffer:destinationoffset:)) method.

```swift
func resolveSampleBuffer(_ sampleBuffer: MTLCounterSampleBuffer,
                         with blitEncoder: MTLBlitCommandEncoder,
                         toBufferWith resourceOptions: MTLResourceOptions) -> MTLBuffer? {

    let counterBufferLength = MemoryLayout<MTLCounterResultTimestamp>.size * sampleCount
    let counterDataBuffer = sampleBuffer.device.makeBuffer(length: counterBufferLength,
                                                           options: resourceOptions)

    guard let counterDataBuffer = counterDataBuffer else {
        return nil
    }

    let range = 0..<sampleCount
    blitEncoder.resolveCounters(sampleBuffer,
                                range: range,
                                destinationBuffer: counterDataBuffer,
                                destinationOffset: 0)

    if resourceOptions.contains(.storageModeManaged) {
        blitEncoder.synchronize(resource: counterDataBuffer)
    }

    return counterDataBuffer
}
```

```objective-c
(id<MTLBuffer>) resolveSampleBuffer:(id<MTLCounterSampleBuffer>)sampleBuffer
                      withBlitEncoder:(id<MTLBlitCommandEncoder>)blitEncoder
              toBufferWithStorageMode:(MTLResourceOptions)storageMode
{
    NSUInteger counterBufferLength = self.sampleCount * sizeof(MTLCounterResultTimestamp);
    id<MTLBuffer> counterDataBuffer = [sampleBuffer.device newBufferWithLength: counterBufferLength
                                                                       options: storageMode];

    if (nil == counterDataBuffer) {
        return nil;
    }

    NSRange range = NSMakeRange(0, self.sampleCount);

    [blitEncoder resolveCounters:sampleBuffer
                         inRange:range
               destinationBuffer:counterDataBuffer
               destinationOffset:0];


    if (storageMode & MTLStorageModeManaged) {
        [blitEncoder synchronizeResource:counterDataBuffer];
    }

    return counterDataBuffer;
}
```

### Cast the counter sample’s data to a result type

Your app can inspect and use the resolved data by casting it to the result type that corresponds to the counter set.

| Counter set names | Counter result types |
|---|---|
| [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounterset/timestamp) | [MTLCounterResultTimestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp) |
| [stageUtilization](https://developer.apple.com/documentation/metal/mtlcommoncounterset/stageutilization) | [MTLCounterResultStageUtilization](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization) |
| [statistic](https://developer.apple.com/documentation/metal/mtlcommoncounterset/statistic) | [MTLCounterResultStatistic](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic) |

For example, your app can cast the data it resolves from a [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounterset/timestamp) counter set as an [MTLCounterResultTimestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp) array.

```swift
/// Converts the contents of the counter sample buffer into an array of result timestamps.
func resolveSampleBuffer() {
    ...
 
    // Convert the contents of the counter sample buffer into the standard data format.
    guard let data = try? counterSampleBuffer.resolveCounterRange(range) else {
        return
    }

    // Declare the destination type for the `Data` instance's contents.
    let timestampSamples: [MTLCounterResultTimestamp]

    // Cast the resolved data into an array of the counter's result type.
    timestampSamples = Array(unsafeUninitializedCapacity: sampleCount) { buffer, initializedCount in
        // Save the size for each counter result timestamp instance.
        let elementSize = MemoryLayout<MTLCounterResultTimestamp>.size

        // Copy the data's bytes into the array's unsafe mutable buffer pointer.
        let bytesCopied = data.copyBytes(to: buffer)

        // Calculate how many complete counter result timestamp elements the method copies.
        initializedCount = bytesCopied / elementSize
    }

    // Check whether the number of samples is correct.
    guard timestampSamples.count == sampleCount else {
        print("Only \(timestampSamples.count) out of \(sampleCount) timestamps resolved.");
        return
    }

    ...
}
```

```objective-c
/// Converts the contents of the counter sample buffer into an array of result timestamps.
- (void) resolveSampleBuffer
    ...
 
    // Convert the contents of the counter sample buffer into the standard data format.
    NSData* data = [self.counterSampleBuffer resolveCounterRange:range];
    ...

    NSUInteger resolvedSampleCount = data.length / sizeof(MTLCounterResultTimestamp);
    if (resolvedSampleCount < sampleCount) {
        printf("Only %lui out of %ui timestamps resolved.", resolvedSampleCount, sampleCount);
        return;
    }

    // Cast the data's bytes property to the counter's result type.
    MTLCounterResultTimestamp* timestamps = (MTLCounterResultTimestamp *)(data.bytes);
    ...
}
```

The code example above also checks whether the result type array has the correct number of elements of the counter set for the app.

### Inspect the information and check for error values

You can also use the result type instances to check whether the GPU stores any error values. The following code example determines whether any of the timestamp samples are equal to `0` or a sentinel error value:

```swift
/// Converts the contents of the counter sample buffer into an array of result timestamps.
func resolveSampleBuffer() {
    ...

    for (index, sample) in timestampSamples.enumerated() {
        if sample.timestamp == MTLCounterErrorValue {
            print("Timestamp sample \(index + 1) (of \(sampleCount)) has an error value.")
            return
        }

        if sample.timestamp == 0 {
            print("Timestamp sample \(index + 1) (of \(sampleCount)) has a value of zero.")
            return
        }
    }

    ...
}

```

```objective-c
/// Converts the contents of the counter sample buffer into an array of result timestamps.
- (void) resolveSampleBuffer
    ...
 
    // Cast the data's bytes property to the counter's result type.
    MTLCounterResultTimestamp* timestamps = (MTLCounterResultTimestamp *)(data.bytes);

    // Check for invalid values within the (resolved) data from the counter sample buffer.
    for (int index = 0; index < resolvedSampleCount; index++) {
        MTLTimestamp timestamp = timestamps[index].timestamp;

        if (timestamp == MTLCounterErrorValue) {
            printf("Timestamp sample #%di (of %ui) has an error value.", index + 1, sampleCount);
            return;
        }

        if (timestamp == 0) {
            printf("Timestamp sample #%di (of %ui) has a value of zero.", index + 1, sampleCount);
            return;
        }
    }

    ...
}
```

Any time the GPU encounters a runtime error while sampling a counter, it sets the counter datum to the sentinel value [MTLCounterErrorValue](https://developer.apple.com/documentation/metal/mtlcountererrorvalue).

> **Note:**
>  A GPU typically stores timestamp values from its internal clock. You can convert those timestamps into more meaningful time values, in nanoseconds, with [sampleTimestamps()](https://developer.apple.com/documentation/metal/mtldevice/sampletimestamps()) — see [Converting GPU timestamps into CPU time](https://developer.apple.com/documentation/metal/converting-gpu-timestamps-into-cpu-time).

## See also

### Counter sample data output
- [MTLCounterResultTimestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp) — The data structure for storing the data you resolve from a timestamp counter set.
- [MTLCounterResultStatistic](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic) — The data structure for storing the data you resolve from a statistic counter set.
- [MTLCounterResultStageUtilization](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization) — The data structure for storing the data you resolve from a stage-utilization counter set.
- [MTLCounterErrorValue](https://developer.apple.com/documentation/metal/mtlcountererrorvalue) — A sentinel value for an entry in a counter sample buffer that indicates the entry’s data is invalid.
