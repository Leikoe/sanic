# makeArgumentEncoder(arguments:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(arguments:)>

Creates a new argument encoder for an array of arguments.

## Declaration

```swift
func makeArgumentEncoder(arguments: [MTLArgumentDescriptor]) -> (any MTLArgumentEncoder)?
```

## Parameters

- **arguments** — An array of [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instances that you need to sort by their [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) properties in monotonically increasing order.

## See also

### Creating argument buffer encoders
- [argumentBuffersSupport](https://developer.apple.com/documentation/metal/mtldevice/argumentbufferssupport) — Returns the GPU device’s support tier for argument buffers.
- [maxArgumentBufferSamplerCount](https://developer.apple.com/documentation/metal/mtldevice/maxargumentbuffersamplercount) — The maximum number of unique argument buffer samplers per app.
- [makeArgumentEncoder(bufferBinding:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(bufferbinding:)) — Creates a new argument encoder for a buffer binding.
