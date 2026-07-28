# makeRenderCommandEncoder(descriptor:options:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandbuffer/makerendercommandencoder(descriptor:options:)>

Creates a render command encoder from a render pass descriptor with additional options.

## Declaration

```swift
func makeRenderCommandEncoder(descriptor: MTL4RenderPassDescriptor, options: MTL4RenderEncoderOptions = []) -> (any MTL4RenderCommandEncoder)?
```

## Parameters

- **descriptor** — Descriptor for the render pass.
- **options** — [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions) instance that provide render pass options.

## Return Value

The created [MTL4RenderCommandEncoder](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder) instance, or `nil` if the function fails.

## Discussion

This method creates a render command encoder to encode a render pass, whilst providing you the option to define some render pass characteristics via an instance of [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions).

Use these options to configure suspending/resuming render command encoders, which allow you to encode render passes from multiple threads simultaneously.
