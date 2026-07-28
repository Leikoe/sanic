# makeRenderCommandEncoder(descriptor:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makerendercommandencoder(descriptor:)>

Creates a render command encoder from a descriptor.

## Declaration

```swift
func makeRenderCommandEncoder(descriptor renderPassDescriptor: MTLRenderPassDescriptor) -> (any MTLRenderCommandEncoder)?
```

## Parameters

- **renderPassDescriptor** — An [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) instance that configures the [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) the method returns.

## Discussion

Use an [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instance’s methods to set up a single graphics-rendering pass.
