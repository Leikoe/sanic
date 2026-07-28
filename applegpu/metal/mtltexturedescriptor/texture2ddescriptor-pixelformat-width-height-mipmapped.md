# texture2DDescriptor(pixelFormat:width:height:mipmapped:)

*Type Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexturedescriptor/texture2ddescriptor(pixelformat:width:height:mipmapped:)>

Creates a texture descriptor object for a 2D texture.

## Declaration

```swift
class func texture2DDescriptor(pixelFormat: MTLPixelFormat, width: Int, height: Int, mipmapped: Bool) -> MTLTextureDescriptor
```

## Parameters

- **pixelFormat** — The format describing how every pixel on the texture image is stored. The default value is [MTLPixelFormat.rgba8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8unorm).
- **width** — The width of the 2D texture image. The value needs to be greater than or equal to `1`.
- **height** — The height of the 2D texture image. The value needs to be greater than or equal to `1`.
- **mipmapped** — A Boolean indicating whether the resulting image should be mipmapped. If [true](https://developer.apple.com/documentation/Swift/true), then the [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/mipmaplevelcount) property in the returned descriptor is computed from `width` and `height`. If [false](https://developer.apple.com/documentation/Swift/false), then [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/mipmaplevelcount) is `1`.

## Return Value

A pointer to a texture descriptor object for a 2D texture.

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Creating texture descriptors
- [textureCubeDescriptor(pixelFormat:size:mipmapped:)](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturecubedescriptor(pixelformat:size:mipmapped:)) — Creates a texture descriptor object for a cube texture.
- [textureBufferDescriptor(with:width:resourceOptions:usage:)](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturebufferdescriptor(with:width:resourceoptions:usage:)) — Creates a texture descriptor object for a texture buffer.
