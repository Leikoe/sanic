# firstMipmapInTail

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture/firstmipmapintail>

The index of the first mipmap in the tail.

## Declaration

```swift
var firstMipmapInTail: Int { get }
```

```swift
optional var firstMipmapInTail: Int { get }
```

## Discussion

In a sparse texture, the *tail* is a collection of mipmaps at higher index values that are mapped as a single block of memory. When you map this mipmap into your sparse texture, Metal also maps mipmap levels with larger index values.

## See also

### Querying sparse properties
- [isSparse](https://developer.apple.com/documentation/metal/mtltexture/issparse) — A Boolean value that indicates whether this is a sparse texture.
- [tailSizeInBytes](https://developer.apple.com/documentation/metal/mtltexture/tailsizeinbytes) — The size of the sparse texture tail, in bytes.
