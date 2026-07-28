# setTextureView(texture:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltextureviewpool/settextureview(texture:index:)>

Copies a default texture view to a slot in this texture view pool at an index provided.

## Declaration

```swift
func setTextureView(texture: any MTLTexture, index: Int) -> MTLResourceID
```

## Parameters

- **texture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance for which to copy its texture view.
- **index** — An index of a slot in this texture pool into which this method copies the texture view.

## Return Value

The [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid) of a newly created texture view in this pool.
