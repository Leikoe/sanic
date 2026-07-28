# suspending

*Type Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/suspending>

Configures the render pass as *suspending*.

## Declaration

```swift
static var suspending: MTL4RenderEncoderOptions { get }
```

## Discussion

Pass this option to [makeRenderCommandEncoder(descriptor:options:)](https://developer.apple.com/documentation/metal/mtl4commandbuffer/makerendercommandencoder(descriptor:options:)) to specify that Metal can stitch the work a render command encoder encodes with a subsequent “resuming” render command encoder.
