# maxArgumentBufferSamplerCount

*Instance Property · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/maxargumentbuffersamplercount>

The maximum number of unique argument buffer samplers per app.

## Declaration

```swift
var maxArgumentBufferSamplerCount: Int { get }
```

## Discussion

This limit only applies to samplers that support argument buffers (see [supportArgumentBuffers](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/supportargumentbuffers)). An [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance is only unique if the properties of the [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) instance that created it are unique. For example, two samplers with equal [minFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/minfilter) values but different [magFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/magfilter) values are unique.

See [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) for more information about argument buffer tiers, limits, and capabilities.

## See also

### Creating argument buffer encoders
- [argumentBuffersSupport](https://developer.apple.com/documentation/metal/mtldevice/argumentbufferssupport) — Returns the GPU device’s support tier for argument buffers.
- [makeArgumentEncoder(arguments:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(arguments:)) — Creates a new argument encoder for an array of arguments.
- [makeArgumentEncoder(bufferBinding:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(bufferbinding:)) — Creates a new argument encoder for a buffer binding.
