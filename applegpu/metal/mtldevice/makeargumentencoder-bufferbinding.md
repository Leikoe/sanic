# makeArgumentEncoder(bufferBinding:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(bufferbinding:)>

Creates a new argument encoder for a buffer binding.

## Declaration

```swift
func makeArgumentEncoder(bufferBinding: any MTLBufferBinding) -> any MTLArgumentEncoder
```

## Parameters

- **bufferBinding** — An [MTLBufferBinding](https://developer.apple.com/documentation/metal/mtlbufferbinding) instance.

## See also

### Creating argument buffer encoders
- [argumentBuffersSupport](https://developer.apple.com/documentation/metal/mtldevice/argumentbufferssupport) — Returns the GPU device’s support tier for argument buffers.
- [maxArgumentBufferSamplerCount](https://developer.apple.com/documentation/metal/mtldevice/maxargumentbuffersamplercount) — The maximum number of unique argument buffer samplers per app.
- [makeArgumentEncoder(arguments:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(arguments:)) — Creates a new argument encoder for an array of arguments.
