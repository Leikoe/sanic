# Sampling GPU data into counter sample buffers

*Article*

<https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers>

Retrieve a GPU’s counter data at a time the GPU supports.

## Overview

You can sample a GPU device’s performance counter data at different times, including:

- At pipeline stage boundaries

- Between different Metal commands

Typically, a GPU supports one of these boundary types. For example, Apple silicon supports sampling at the stage boundary because it processes fragments after processing every primitive for a render pass. However, a typical immediate-mode GPU supports sampling between commands.

Before you can sample a GPU counter, implement the following prerequisite steps:

1. Identify which counters you can sample from an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance (see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports)).

2. Make an [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) instance to store the counter’s data (see [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass)).

The sections below explain how to identify when you can sample a GPU’s counters, and how to encode commands to retrieve their data.

Each GPU vendor defines its own private data format for its counter sample buffers, which means your app can’t read the contents of each buffer directly. Instead, your app can transform the data from the vendor’s internal format to the standard Metal formats by *resolving* each sample buffer. See [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format) for the next steps that resolve the data within a counter sample buffer.

### Check which boundaries a GPU supports

You can inspect a GPU device instance to see whether it supports a specific sample boundary by calling its [supportsCounterSampling(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportscountersampling(_:)) method with each [MTLCounterSamplingPoint](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint) case.

```swift
func samplingBoundariesFor(_ device: MTLDevice) -> [MTLCounterSamplingPoint] {
    let boundaryNames = ["atStageBoundary",
                         "atDrawBoundary",
                         "atBlitBoundary",
                         "atDispatchBoundary",
                         "atTileDispatchBoundary"]

    let allBoundaries: [MTLCounterSamplingPoint] = [.atStageBoundary,
                                                   .atDrawBoundary,
                                                   .atBlitBoundary,
                                                   .atDispatchBoundary,
                                                   .atTileDispatchBoundary]

    print("The GPU device supports the following sampling boundary/ies: [", terminator: "")
    var boundaries = [MTLCounterSamplingPoint]()

    for index in 0..<boundaryNames.count {
        let boundary = allBoundaries[index]
        if device.supportsCounterSampling(boundary) {
            if boundaries.count >= 1 {
                // Prefix the boundary's name with a comma and a space.
                print(", ", terminator: "")
            }

            // Print the boundary's name.
            print("\(boundaryNames[index])", terminator: "")

            // Add the boundary to the return-value array.
            boundaries.append(boundary)
        }
    }

    // Finish printing the line that lists the boundaries the GPU device supports.
    // Example: "The GPU device supports the following sampling boundaries: [atStageBoundary]"
    print("]")

    return boundaries
}
```

```objective-c
+ (NSArray<NSNumber*>*) samplingBoundariesFor:(id<MTLDevice>)device
{
    NSArray<NSString*>* boundaryNames = @[@"atStageBoundary",
                                          @"atDrawBoundary",
                                          @"atBlitBoundary",
                                          @"atDispatchBoundary",
                                          @"atTileDispatchBoundary"];

    NSUInteger allBoundaries[] = {
        MTLCounterSamplingPointAtStageBoundary,
        MTLCounterSamplingPointAtDrawBoundary,
        MTLCounterSamplingPointAtBlitBoundary,
        MTLCounterSamplingPointAtDispatchBoundary,
        MTLCounterSamplingPointAtTileDispatchBoundary};

    printf("The GPU device supports the following sampling boundary/ies: [");

    NSMutableArray<NSNumber*>* boundaries = [[NSMutableArray<NSNumber*> alloc] init];

    for (int index = 0; index < boundaryNames.count; index++) {
        if ([device supportsCounterSampling:allBoundaries[index]]) {
            if (boundaries.count >= 1) {
                // Prefix the boundary's name with a comma and a space.
                printf(", ");
            }

            // Print the boundary's name.
            printf("%s", boundaryNames[index].UTF8String);

            // Add the boundary to the return-value array.
            NSNumber* boundaryNumber = [NSNumber numberWithUnsignedLong:allBoundaries[index]];
            [boundaries addObject: boundaryNumber];
        }
    }

    // Finish printing the line that lists the boundaries the GPU device supports.
    // Example: "The GPU device supports these sampling boundaries: [atStageBoundary]"
    printf("]\n");

    return boundaries;
}
```

This method checks for multiple sample boundaries and returns those the GPU supports in an array.

### Sample counters at stage boundaries

For a GPU device that can sample counters at stage boundaries ( [MTLCounterSamplingPoint.atStageBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atstageboundary)), you can sample its counters between the stages of a pass. When the GPU starts or finishes a stage, it samples the counters and copies the results into a counter sample buffer.

> **Note:**
>  By default, a pass doesn’t sample any GPU counters.

You tell the GPU which counters to sample by configuring a pass descriptor’s [sampleBufferAttachments](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor/samplebufferattachments) property. For example, you can sample the timestamp counters before and after the vertex and fragment stages by configuring an [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) instance’s [sampleBufferAttachments](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/samplebufferattachments) property.

```swift
func configureRenderPass(_ renderPass: MTLRenderPassDescriptor, attachmentIndex: Int = 0) {
    guard let sampleAttachment = renderPass.sampleBufferAttachments[attachmentIndex] else {
        return
    }

    sampleAttachment.sampleBuffer = self.counterSampleBuffer
    sampleAttachment.startOfVertexSampleIndex = 0
    sampleAttachment.endOfVertexSampleIndex = 1
    sampleAttachment.startOfFragmentSampleIndex = 2
    sampleAttachment.endOfFragmentSampleIndex = 3
}
```

```objective-c
- (void) configureRenderPass:(MTLRenderPassDescriptor *)renderPass
             attachmentIndex: (int)index
{
    MTLRenderPassSampleBufferAttachmentDescriptor *sampleAttachment;
    sampleAttachment = renderPass.sampleBufferAttachments[index];

    sampleAttachment.sampleBuffer = self.counterSampleBuffer;
    sampleAttachment.startOfVertexSampleIndex = 0;
    sampleAttachment.endOfVertexSampleIndex = 1;
    sampleAttachment.startOfFragmentSampleIndex = 2;
    sampleAttachment.endOfFragmentSampleIndex = 3;
}
```

Each index value tells the GPU where to put a specific counter value within a counter sample buffer. You can skip specific counters by setting an index to [MTLCounterDontSample](https://developer.apple.com/documentation/metal/mtlcounterdontsample). For example, you can alter the code example above so that the GPU only samples before and after a fragment stage.

```swift
    ...
    sampleAttachment.sampleBuffer = self.counterSampleBuffer;
    sampleAttachment.startOfVertexSampleIndex = MTLCounterDontSample;
    sampleAttachment.endOfVertexSampleIndex = MTLCounterDontSample;
    sampleAttachment.startOfFragmentSampleIndex = 2;
    sampleAttachment.endOfFragmentSampleIndex = 3;
}
```

This example still stores the fragment counter data in the third and fourth positions within the counter sample buffer (at indexes 2 and 3, respectively). However, it doesn’t sample the vertex stage timestamps, which leaves that part of the counter sample buffer unaltered.

Each type of pass has different boundary types and corresponding properties in their descriptor types.

| Pass descriptor type | Attachment type | Attachment descriptor properties |
|---|---|---|
| [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) | [MTLRenderPassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor) | [sampleBuffer](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/samplebuffer) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [startOfVertexSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/startofvertexsampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfVertexSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/endofvertexsampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [startOfFragmentSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/startoffragmentsampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfFragmentSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/endoffragmentsampleindex) |
| [MTLAccelerationStructurePassDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepassdescriptor) | [MTLAccelerationStructurePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptor) | [sampleBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptor/samplebuffer) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptor/startofencodersampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptor/endofencodersampleindex) |
| [MTLComputePassDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor) | [MTLComputePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor) | [sampleBuffer](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/samplebuffer) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/startofencodersampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/endofencodersampleindex) |
| [MTLBlitPassDescriptor](https://developer.apple.com/documentation/metal/mtlblitpassdescriptor) | [MTLBlitPassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor) | [sampleBuffer](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/samplebuffer) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/startofencodersampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/endofencodersampleindex) |
| [MTLResourceStatePassDescriptor](https://developer.apple.com/documentation/metal/mtlresourcestatepassdescriptor) | [MTLResourceStatePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor) | [sampleBuffer](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/samplebuffer) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/startofencodersampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/endofencodersampleindex) |

### Sample counters at command boundaries

You can encode specific commands to sample a counter’s data during a pass for a GPU that supports any of the following boundaries:

- [MTLCounterSamplingPoint.atDrawBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atdrawboundary)

- [MTLCounterSamplingPoint.atDispatchBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atdispatchboundary)

- [MTLCounterSamplingPoint.atBlitBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atblitboundary)

- [MTLCounterSamplingPoint.atTileDispatchBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/attiledispatchboundary)

You do this by calling an encoder’s [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) method.

```swift
renderEncoder.drawPrimitives(type: .triangle, vertexStart: 0, vertexCount: 6)

...


// Store the GPU counter data in the sample buffer.
renderEncoder.sampleCounters(sampleBuffer: self.counterSampleBuffer,
                             sampleIndex: 0,
                             barrier: false)

...

renderEncoder.drawPrimitives(type: .triangle,
                             vertexStart: entity.offset,
                             vertexCount: entity.count)
```

```objective-c
[renderEncoder drawPrimitives:MTLPrimitiveTypeTriangle
                  vertexStart:0
                  vertexCount: 6];

...

// Store the GPU counter data in the sample buffer.
[renderEncoder sampleCountersInBuffer: self.counterSampleBuffer
                        atSampleIndex: 0
                          withBarrier: NO];

...

[renderEncoder drawPrimitives: MTLPrimitiveTypeTriangle
                  vertexStart: entity.start
                  vertexCount: entity.count];
```

The code example above encodes a sample command between two draw commands. When the GPU reaches the sample command, it samples the counters and copies the results into a counter sample buffer.

Each pass encoder type has its own version of the method.

| Pass encoder type | Sample method |
|---|---|
| [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) | [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) |
| [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) | [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) |
| [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) | [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) |
| [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) | [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) |

The `barrier` parameter for these methods controls whether the pass waits for the GPU to complete all the previous commands in the buffer before sampling the counters (see [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization)). Each barrier typically reduces performance, but can be useful during development to get accurate and consistent data across multiple runs.

## See also

### Counter sample buffers
- [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) — Make a buffer that provides a place for a GPU to save its runtime performance metrics as it runs a pass.
- [MTLCounterSampleBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor) — A group of properties that configures the counter sample buffers you create with it.
- [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) — A specialized memory buffer that stores a GPU’s counter set data.
- [MTLCounterDontSample](https://developer.apple.com/documentation/metal/mtlcounterdontsample) — A sentinel value that instructs an encoder to skip sampling a counter as the GPU runs the encoder’s pass.
