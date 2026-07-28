# MTLCounterDontSample

*Global Variable · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlcounterdontsample>

A sentinel value that instructs an encoder to skip sampling a counter as the GPU runs the encoder’s pass.

## Declaration

```swift
var MTLCounterDontSample: Int { get }
```

## Discussion

You can skip sampling at specific stages by assigning this sentinel value to the following properties instead of an offset to a counter sample buffer:

| Types | Properties |
|---|---|
| [MTLRenderPassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor) | [startOfVertexSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/startofvertexsampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfVertexSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/endofvertexsampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [startOfFragmentSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/startoffragmentsampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfFragmentSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/endoffragmentsampleindex) |
| [MTLComputePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor) | [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/startofencodersampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/endofencodersampleindex) |
| [MTLBlitPassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor) | [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/startofencodersampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/endofencodersampleindex) |
| [MTLResourceStatePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor) | [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/startofencodersampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/endofencodersampleindex) |
| [MTLAccelerationStructurePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptor) | [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptor/startofencodersampleindex) ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptor/endofencodersampleindex) |

## See also

### Counter sample buffers
- [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) — Make a buffer that provides a place for a GPU to save its runtime performance metrics as it runs a pass.
- [MTLCounterSampleBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor) — A group of properties that configures the counter sample buffers you create with it.
- [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) — A specialized memory buffer that stores a GPU’s counter set data.
- [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers) — Retrieve a GPU’s counter data at a time the GPU supports.
