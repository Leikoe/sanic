# MTLDepthStencilDescriptor

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldepthstencildescriptor>

An instance that configures new [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) instances.

## Declaration

```swift
class MTLDepthStencilDescriptor
```

## Overview

An [MTLDepthStencilDescriptor](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor) instance is used to define a specific configuration of the depth and stencil stages of a rendering pipeline. To create an [MTLDepthStencilDescriptor](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor) instance, use standard allocation and initialization techniques.

To enable writing the depth value to a depth attachment, set the depthWriteEnabled property to [true](https://developer.apple.com/documentation/Swift/true).

The depthCompareFunction property specifies how the depth test is performed. If a fragment’s depth value fails the depth test, the fragment is discarded. [MTLCompareFunction.less](https://developer.apple.com/documentation/metal/mtlcomparefunction/less) is a commonly used value for [depthCompareFunction](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/depthcomparefunction), because fragment values that are farther away from the viewer than the pixel depth value (a previously written fragment) fail the depth test and are considered occluded by the earlier depth value.

The [frontFaceStencil](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/frontfacestencil) and [backFaceStencil](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/backfacestencil) properties define two independent stencil descriptors: one for front-facing primitives and the other for back-facing primitives, respectively. Both properties can be set to the same MTLStencilDescriptor instance.

## Topics

### Specifying depth operations
- [depthCompareFunction](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/depthcomparefunction) — The comparison that is performed between a fragment’s depth value and the depth value in the attachment, which determines whether to discard the fragment.
- [isDepthWriteEnabled](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/isdepthwriteenabled) — A Boolean value that indicates whether depth values can be written to the depth attachment.

### Specifying stencil descriptors for primitives
- [backFaceStencil](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/backfacestencil) — The stencil descriptor for back-facing primitives.
- [frontFaceStencil](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/frontfacestencil) — The stencil descriptor for front-facing primitives.

### Identifying properties
- [label](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/label) — A string that identifies this object.

## See also

### Depth testing
- [Calculating primitive visibility using depth testing](https://developer.apple.com/documentation/metal/calculating-primitive-visibility-using-depth-testing) — Determine which pixels are visible in a scene by using a depth texture.
- [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) — A depth and stencil state instance that specifies the depth and stencil configuration and operations used in a render pass.
- [MTLStencilDescriptor](https://developer.apple.com/documentation/metal/mtlstencildescriptor) — An object that defines the front-facing or back-facing stencil operations of a depth and stencil state object.
