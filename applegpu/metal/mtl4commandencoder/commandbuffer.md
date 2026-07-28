# commandBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandencoder/commandbuffer>

Returns the command buffer that is currently encoding commands.

## Declaration

```swift
var commandBuffer: (any MTL4CommandBuffer)? { get }
```

## Discussion

This property may return undefined results if you call it after calling [endEncoding()](https://developer.apple.com/documentation/metal/mtl4commandencoder/endencoding()).
