# makeSharedTextureHandle()

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.14, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture/makesharedtexturehandle()>

Creates a new texture handle from a shareable texture.

## Declaration

```swift
func makeSharedTextureHandle() -> MTLSharedTextureHandle?
```

## Discussion

If the texture is not shareable, this method returns `nil`.
