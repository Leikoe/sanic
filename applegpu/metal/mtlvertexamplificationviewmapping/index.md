# MTLVertexAmplificationViewMapping

*Structure · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping>

An offset applied to a render target index and viewport index.

## Declaration

```swift
struct MTLVertexAmplificationViewMapping
```

## Topics

### Creating a view mapping
- [init()](https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping/init()) — Initializes a default view mapping.
- [init(viewportArrayIndexOffset:renderTargetArrayIndexOffset:)](https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping/init(viewportarrayindexoffset:rendertargetarrayindexoffset:)) — Initializes a new view mapping.

### Specifying mapping offsets
- [renderTargetArrayIndexOffset](https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping/rendertargetarrayindexoffset) — An offset into the list of render targets.
- [viewportArrayIndexOffset](https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping/viewportarrayindexoffset) — An offset into the list of viewports.

## See also

### Dynamic render pipeline states
- [MTLViewport](https://developer.apple.com/documentation/metal/mtlviewport) — A 3D rectangular region for the viewport clipping.
- [MTLScissorRect](https://developer.apple.com/documentation/metal/mtlscissorrect) — A rectangle for the scissor fragment test.
- [MTLQuadTessellationFactorsHalf](https://developer.apple.com/documentation/metal/mtlquadtessellationfactorshalf) — The per-patch tessellation factors for a quad patch.
- [MTLTriangleTessellationFactorsHalf](https://developer.apple.com/documentation/metal/mtltriangletessellationfactorshalf) — The per-patch tessellation factors for a triangle patch.
