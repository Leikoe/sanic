# makeParallelRenderCommandEncoder(descriptor:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeparallelrendercommandencoder(descriptor:)>

Creates a parallel render command encoder from a descriptor.

## Declaration

```swift
func makeParallelRenderCommandEncoder(descriptor renderPassDescriptor: MTLRenderPassDescriptor) -> (any MTLParallelRenderCommandEncoder)?
```

## Parameters

- **renderPassDescriptor** — An [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) instance that configures the [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) the method returns.

## Discussion

An [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) instance can create multiple, independent render command encoders that contribute to the same render pass on different threads.
