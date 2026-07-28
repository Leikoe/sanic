# MTLStencilDescriptor

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstencildescriptor>

An object that defines the front-facing or back-facing stencil operations of a depth and stencil state object.

## Declaration

```swift
class MTLStencilDescriptor
```

## Overview

A stencil test is a comparison between a masked reference value and a masked value stored in a stencil attachment. (A value is *masked* by performing a logical AND operation on it with the [readMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/readmask) value.) The [MTLStencilDescriptor](https://developer.apple.com/documentation/metal/mtlstencildescriptor) object defines how to update the contents of the stencil attachment, based on the results of the stencil test and the depth test.

The [stencilCompareFunction](https://developer.apple.com/documentation/metal/mtlstencildescriptor/stencilcomparefunction) property defines the stencil test. The [stencilFailureOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/stencilfailureoperation), [depthFailureOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/depthfailureoperation), and [depthStencilPassOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/depthstencilpassoperation) properties specify what to do to a stencil value stored in the stencil attachment for three different test outcomes: if the stencil test fails, if the stencil test passes and the depth test fails, or if both stencil and depth tests succeed, respectively. [writeMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/writemask) determines which stencil bits can be modified as the result of a stencil operation.

## Topics

### Configuring stencil functions and operations
- [stencilFailureOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/stencilfailureoperation) — The operation that is performed to update the values in the stencil attachment when the stencil test fails.
- [depthFailureOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/depthfailureoperation) — The operation that is performed to update the values in the stencil attachment when the stencil test passes, but the depth test fails.
- [depthStencilPassOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/depthstencilpassoperation) — The operation that is performed to update the values in the stencil attachment when both the stencil test and the depth test pass.
- [stencilCompareFunction](https://developer.apple.com/documentation/metal/mtlstencildescriptor/stencilcomparefunction) — The comparison that is performed between the masked reference value and a masked value in the stencil attachment.
- [MTLStencilOperation](https://developer.apple.com/documentation/metal/mtlstenciloperation) — The operation performed on a currently stored stencil value when a comparison test passes or fails.

### Configuring stencil bit mask properties
- [readMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/readmask) — A bitmask that determines from which bits that stencil comparison tests can read.
- [writeMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/writemask) — A bitmask that determines to which bits that stencil operations can write.

## See also

### Depth testing
- [Calculating primitive visibility using depth testing](https://developer.apple.com/documentation/metal/calculating-primitive-visibility-using-depth-testing) — Determine which pixels are visible in a scene by using a depth texture.
- [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) — A depth and stencil state instance that specifies the depth and stencil configuration and operations used in a render pass.
- [MTLDepthStencilDescriptor](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor) — An instance that configures new [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) instances.
