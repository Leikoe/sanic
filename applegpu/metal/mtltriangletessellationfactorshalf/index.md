# MTLTriangleTessellationFactorsHalf

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtltriangletessellationfactorshalf>

The per-patch tessellation factors for a triangle patch.

## Declaration

```swift
struct MTLTriangleTessellationFactorsHalf
```

## Overview

Refer to the [Tessellation](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Tessellation/Tessellation.html#//apple_ref/doc/uid/TP40014221-CH15) chapter of the [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221) for further information.

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtltriangletessellationfactorshalf/init())
- [init(edgeTessellationFactor:insideTessellationFactor:)](https://developer.apple.com/documentation/metal/mtltriangletessellationfactorshalf/init(edgetessellationfactor:insidetessellationfactor:))

### Instance Properties
- [edgeTessellationFactor](https://developer.apple.com/documentation/metal/mtltriangletessellationfactorshalf/edgetessellationfactor) — The edge tessellation factors, with each index value providing the tessellation factor for a particular edge.
- [insideTessellationFactor](https://developer.apple.com/documentation/metal/mtltriangletessellationfactorshalf/insidetessellationfactor) — The inside tessellation factor.

## See also

### Dynamic render pipeline states
- [MTLViewport](https://developer.apple.com/documentation/metal/mtlviewport) — A 3D rectangular region for the viewport clipping.
- [MTLScissorRect](https://developer.apple.com/documentation/metal/mtlscissorrect) — A rectangle for the scissor fragment test.
- [MTLVertexAmplificationViewMapping](https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping) — An offset applied to a render target index and viewport index.
- [MTLQuadTessellationFactorsHalf](https://developer.apple.com/documentation/metal/mtlquadtessellationfactorshalf) — The per-patch tessellation factors for a quad patch.
