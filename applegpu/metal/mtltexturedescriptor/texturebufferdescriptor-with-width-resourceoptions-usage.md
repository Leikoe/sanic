# textureBufferDescriptor(with:width:resourceOptions:usage:)

*Type Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturebufferdescriptor(with:width:resourceoptions:usage:)>

Creates a texture descriptor object for a texture buffer.

## Declaration

```swift
class func textureBufferDescriptor(with pixelFormat: MTLPixelFormat, width: Int, resourceOptions: MTLResourceOptions = [], usage: MTLTextureUsage) -> MTLTextureDescriptor
```

## Parameters

- **pixelFormat** — The format describing how every pixel on the texture buffer is stored. The default value is [MTLPixelFormat.rgba8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8unorm).
- **width** — The width of the texture buffer. The value needs to be greater than or equal to `1`.
- **resourceOptions** — The access options to use for the new texture buffer.
- **usage** — The allowed usage of the new texture buffer.

## Return Value

A pointer to a texture descriptor object for a texture buffer.

## See also

### Creating texture descriptors
- [texture2DDescriptor(pixelFormat:width:height:mipmapped:)](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texture2ddescriptor(pixelformat:width:height:mipmapped:)) — Creates a texture descriptor object for a 2D texture.
- [textureCubeDescriptor(pixelFormat:size:mipmapped:)](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturecubedescriptor(pixelformat:size:mipmapped:)) — Creates a texture descriptor object for a cube texture.
