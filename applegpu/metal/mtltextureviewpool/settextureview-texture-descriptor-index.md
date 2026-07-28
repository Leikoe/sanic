# setTextureView(texture:descriptor:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltextureviewpool/settextureview(texture:descriptor:index:)>

Creates a new lightweight texture view.

## Declaration

```swift
func setTextureView(texture: any MTLTexture, descriptor: MTLTextureViewDescriptor, index: Int) -> MTLResourceID
```

## Parameters

- **texture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance for which to create a new lightweight texture view.
- **descriptor** — A descriptor specifying properties of the texture view to create.
- **index** — An index of a slot in the texture pool into which this method writes the new texture view.

## Return Value

The [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid) of a newly created texture view in this pool.

## Discussion

This method creates a lightweight texture view over a texture according to a descriptor you provide. It then associates the texture view with a slot in this texture view pool at the index you specify.
