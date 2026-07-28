# MTLScissorRect

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlscissorrect>

A rectangle for the scissor fragment test.

## Declaration

```swift
struct MTLScissorRect
```

## Topics

### Creating a scissor rectangle
- [init()](https://developer.apple.com/documentation/metal/mtlscissorrect/init())
- [init(x:y:width:height:)](https://developer.apple.com/documentation/metal/mtlscissorrect/init(x:y:width:height:))

### Specifying scissor boundaries
- [height](https://developer.apple.com/documentation/metal/mtlscissorrect/height) — The height of the scissor rectangle, in pixels.
- [width](https://developer.apple.com/documentation/metal/mtlscissorrect/width) — The width of the scissor rectangle, in pixels.
- [x](https://developer.apple.com/documentation/metal/mtlscissorrect/x) — The x window coordinate of the upper-left corner of the scissor rectangle.
- [y](https://developer.apple.com/documentation/metal/mtlscissorrect/y) — The y window coordinate of the upper-left corner of the scissor rectangle.

## See also

### Dynamic render pipeline states
- [MTLViewport](https://developer.apple.com/documentation/metal/mtlviewport) — A 3D rectangular region for the viewport clipping.
- [MTLVertexAmplificationViewMapping](https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping) — An offset applied to a render target index and viewport index.
- [MTLQuadTessellationFactorsHalf](https://developer.apple.com/documentation/metal/mtlquadtessellationfactorshalf) — The per-patch tessellation factors for a quad patch.
- [MTLTriangleTessellationFactorsHalf](https://developer.apple.com/documentation/metal/mtltriangletessellationfactorshalf) — The per-patch tessellation factors for a triangle patch.
