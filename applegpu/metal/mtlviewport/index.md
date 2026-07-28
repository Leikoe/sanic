# MTLViewport

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlviewport>

A 3D rectangular region for the viewport clipping.

## Declaration

```swift
struct MTLViewport
```

## Topics

### Creating a viewport
- [init()](https://developer.apple.com/documentation/metal/mtlviewport/init()) — Returns a new viewport.
- [init(originX:originY:width:height:znear:zfar:)](https://developer.apple.com/documentation/metal/mtlviewport/init(originx:originy:width:height:znear:zfar:)) — Returns a new viewport of a specified size at a specified origin.

### Specifying viewport boundaries
- [originX](https://developer.apple.com/documentation/metal/mtlviewport/originx) — The x coordinate of the upper-left corner of the viewport.
- [originY](https://developer.apple.com/documentation/metal/mtlviewport/originy) — The y coordinate of the upper-left corner of the viewport.
- [width](https://developer.apple.com/documentation/metal/mtlviewport/width) — The width of the viewport, in pixels.
- [height](https://developer.apple.com/documentation/metal/mtlviewport/height) — The height of the viewport, in pixels.
- [znear](https://developer.apple.com/documentation/metal/mtlviewport/znear) — The z coordinate of the near clipping plane of the viewport.
- [zfar](https://developer.apple.com/documentation/metal/mtlviewport/zfar) — The z coordinate of the far clipping plane of the viewport.

## See also

### Dynamic render pipeline states
- [MTLScissorRect](https://developer.apple.com/documentation/metal/mtlscissorrect) — A rectangle for the scissor fragment test.
- [MTLVertexAmplificationViewMapping](https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping) — An offset applied to a render target index and viewport index.
- [MTLQuadTessellationFactorsHalf](https://developer.apple.com/documentation/metal/mtlquadtessellationfactorshalf) — The per-patch tessellation factors for a quad patch.
- [MTLTriangleTessellationFactorsHalf](https://developer.apple.com/documentation/metal/mtltriangletessellationfactorshalf) — The per-patch tessellation factors for a triangle patch.
