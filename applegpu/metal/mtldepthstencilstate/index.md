# MTLDepthStencilState

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldepthstencilstate>

A depth and stencil state instance that specifies the depth and stencil configuration and operations used in a render pass.

## Declaration

```swift
protocol MTLDepthStencilState : NSObjectProtocol, Sendable
```

## Overview

The [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) protocol defines the interface for a lightweight instance used to encode how a graphics rendering pass should perform depth and stencil operations. The [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) uses an [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) instance to set the depth and stencil state for a rendering pass.

The standard allocation and initialization techniques don’t apply when creating an [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) instance. Instead, you can apply the following steps:

1. Create an [MTLDepthStencilDescriptor](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor) instance that defines the operations you want the rendering pass to use.

2. Create an [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) instance by passing the descriptor to an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [makeDepthStencilState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makedepthstencilstate(descriptor:)) method.

Typically, you create [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) instances when your app is first initialized and then reuse them throughout the lifetime of your app.

## Topics

### Identifying properties
- [device](https://developer.apple.com/documentation/metal/mtldepthstencilstate/device) — The device from which this state object was created.
- [label](https://developer.apple.com/documentation/metal/mtldepthstencilstate/label) — A string that identifies this object.

### Instance Properties
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtldepthstencilstate/gpuresourceid)

## See also

### Depth testing
- [Calculating primitive visibility using depth testing](https://developer.apple.com/documentation/metal/calculating-primitive-visibility-using-depth-testing) — Determine which pixels are visible in a scene by using a depth texture.
- [MTLDepthStencilDescriptor](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor) — An instance that configures new [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) instances.
- [MTLStencilDescriptor](https://developer.apple.com/documentation/metal/mtlstencildescriptor) — An object that defines the front-facing or back-facing stencil operations of a depth and stencil state object.
