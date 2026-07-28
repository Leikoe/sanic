# destinationLevel

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/destinationlevel>

The index of the mipmap level in the destination texture.

## Declaration

```swift
var destinationLevel: Int
```

## Discussion

Provide a value between `0` and the destination texture’s [firstMipmapInTail](https://developer.apple.com/documentation/metal/mtltexture/firstmipmapintail).

When [sourceLevel](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/sourcelevel) is equal to the source texture’s [firstMipmapInTail](https://developer.apple.com/documentation/metal/mtltexture/firstmipmapintail), set [destinationLevel](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/destinationlevel) to the destination texture’s [firstMipmapInTail](https://developer.apple.com/documentation/metal/mtltexture/firstmipmapintail).
