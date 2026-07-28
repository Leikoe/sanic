# renderTargetArrayLength

*Instance Property · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.11, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/rendertargetarraylength>

The number of active layers that all attachments need to have for layered rendering.

## Declaration

```swift
var renderTargetArrayLength: Int { get set }
```

## Discussion

The default value is `0`, indicating that the GPU does not use layered rendering on this render pass.

The table below gives typical values you might set, depending on the type of texture being used as attachments in the render pass. Your vertex shader need to select the render target array index between `0` and the array length minus `1`.

| Texture Type | Typical Length |
|---|---|
| [MTLTextureType.type1DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type1darray) or [MTLTextureType.type2DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type2darray) | The length of the texture array ([arrayLength](https://developer.apple.com/documentation/metal/mtltexture/arraylength)) |
| [MTLTextureType.typeCube](https://developer.apple.com/documentation/metal/mtltexturetype/typecube) | 6 |
| [MTLTextureType.typeCubeArray](https://developer.apple.com/documentation/metal/mtltexturetype/typecubearray) | 6 times the length of the texture array ([arrayLength](https://developer.apple.com/documentation/metal/mtltexture/arraylength)) |

## See also

### Layered rendering
- [renderTargetWidth](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/rendertargetwidth) — The width, in pixels, to constrain the render target to.
- [renderTargetHeight](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/rendertargetheight) — The height, in pixels, to constrain the render target to.
