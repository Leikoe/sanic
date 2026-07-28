# setTexture(_:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4argumenttable/settexture(_:index:)>

Binds a texture to a texture binding slot.

## Declaration

```swift
func setTexture(_ resourceID: MTLResourceID, index bindingIndex: Int)
```

## Parameters

- **resourceID** — The [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid) of the [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance to bind.
- **bindingIndex** — A valid binding index in the texture binding range. It is an error for this value to match or exceed the value of property [maxTextureBindCount](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor/maxtexturebindcount) on the descriptor from which you created this argument table.
