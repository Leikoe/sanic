# stencilFromDepthStencil

*Type Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitoption/stencilfromdepthstencil>

A blit option that copies the stencil portion of a combined depth and stencil texture to or from a buffer.

## Declaration

```swift
static var stencilFromDepthStencil: MTLBlitOption { get }
```

## Discussion

You can pass this option to some methods that copy data between a buffer and a texture, including the following:

- [copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:options:))

- [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:))

## See also

### Depth and stencil buffer options
- [depthFromDepthStencil](https://developer.apple.com/documentation/metal/mtlblitoption/depthfromdepthstencil) — A blit option that copies the depth portion of a combined depth and stencil texture to or from a buffer.
