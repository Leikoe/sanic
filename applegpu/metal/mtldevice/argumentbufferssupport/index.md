# argumentBuffersSupport

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/argumentbufferssupport>

Returns the GPU device’s support tier for argument buffers.

## Declaration

```swift
var argumentBuffersSupport: MTLArgumentBuffersTier { get }
```

## Topics

### Argument buffer tiers
- [MTLArgumentBuffersTier](https://developer.apple.com/documentation/metal/mtlargumentbufferstier) — The values that determine the limits and capabilities of argument buffers.

## See also

### Creating argument buffer encoders
- [maxArgumentBufferSamplerCount](https://developer.apple.com/documentation/metal/mtldevice/maxargumentbuffersamplercount) — The maximum number of unique argument buffer samplers per app.
- [makeArgumentEncoder(arguments:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(arguments:)) — Creates a new argument encoder for an array of arguments.
- [makeArgumentEncoder(bufferBinding:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(bufferbinding:)) — Creates a new argument encoder for a buffer binding.
