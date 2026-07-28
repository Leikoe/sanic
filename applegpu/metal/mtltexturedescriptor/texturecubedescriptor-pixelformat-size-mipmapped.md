# textureCubeDescriptor(pixelFormat:size:mipmapped:)

*Type Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturecubedescriptor(pixelformat:size:mipmapped:)>

Creates a texture descriptor object for a cube texture.

## Declaration

```swift
class func textureCubeDescriptor(pixelFormat: MTLPixelFormat, size: Int, mipmapped: Bool) -> MTLTextureDescriptor
```

## Parameters

- **pixelFormat** — The format describing how every pixel on the texture image is stored. The default value is [MTLPixelFormat.rgba8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8unorm).
- **size** — The width and height of each slice of the cube texture. The value needs to be greater than or equal to `1`.
- **mipmapped** — A Boolean indicating whether the resulting image should be mipmapped. If [true](https://developer.apple.com/documentation/Swift/true), then the [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/mipmaplevelcount) property in the returned descriptor is computed from `width` and `height`. If [false](https://developer.apple.com/documentation/Swift/false), then [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/mipmaplevelcount) is `1`.

## Return Value

A pointer to a texture descriptor object for a cube texture.

## Discussion

For a cube texture, the property values describe one slice, which is any one of its six sides. Each slice is a square.

## See also

### Creating texture descriptors
- [texture2DDescriptor(pixelFormat:width:height:mipmapped:)](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texture2ddescriptor(pixelformat:width:height:mipmapped:)) — Creates a texture descriptor object for a 2D texture.
- [textureBufferDescriptor(with:width:resourceOptions:usage:)](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturebufferdescriptor(with:width:resourceoptions:usage:)) — Creates a texture descriptor object for a texture buffer.
