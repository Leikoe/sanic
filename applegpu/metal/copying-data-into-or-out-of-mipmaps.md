# Copying data into or out of mipmaps

*Article*

<https://developer.apple.com/documentation/metal/copying-data-into-or-out-of-mipmaps>

Specify which mipmaps that the data transfer affects.

## Overview

When you copy data between resources, and the source or destination is a texture, specify which mipmaps that the data transfer affects.

### Copy data from system memory to a mipmap

When you copy data from system memory into a texture, using the [replace(region:mipmapLevel:withBytes:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:withbytes:bytesperrow:)) or similar method, state which mipmap is the destination of that copy.

```swift
// Create a 3D region, where image is a CGContext instance.
let image = <#CGContext#>
let region: MTLRegion = MTLRegionMake3D(0, 0, 0, image.width, image.height, 1)

// Replace the region in the texture.
texture.replace(region: region, mipmapLevel: 0, withBytes: image.data!, bytesPerRow: image.bytesPerRow)
```

```objective-c
MTLRegion region = {
    { 0, 0, 0 },                   // MTLOrigin
    {image.width, image.height, 1} // MTLSize
};

[texture replaceRegion:region
           mipmapLevel:0
             withBytes:image.data.bytes
           bytesPerRow:bytesPerRow];
```

Call this routine once for each mipmap you want to fill, changing the region to match the size of the mipmap level you’re writing to.

### Copy mipmap data between Metal resources

If you already have data in Metal resources, use an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) to copy data to and from different mipmaps in a texture.

To copy all matching data between two textures, encode a command using the [copy(from:to:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:to:)): method. The two textures need to have the same pixel format and type. Metal copies all matching mipmap sizes to the destination texture.

To copy a selection of mipmaps from one texture to another, use the [copy(from:sourceSlice:sourceLevel:to:destinationSlice:destinationLevel:sliceCount:levelCount:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:to:destinationslice:destinationlevel:slicecount:levelcount:)) method. Specify the first source mipmap level and first destination mipmap level, both of which need to have the same dimensions. Also specify the number of mipmap levels you want to copy.

For example, the following code assumes that the destination texture is twice as large in both dimensions as the source texture. Mipmap `1` in the destination matches the size of the source mipmap `0`, so the code passes `0` as the source level and `1` as the destination level. It also passes `5` as the level count to copy `5` mipmaps.

```swift
// Copy mipmap data between MTLTexture instances.
let source = <#MTLTexture#>, destination = <#MTLTexture#>
            
encoder.copy(from: source, sourceSlice: 0, sourceLevel: 0, to: destination, destinationSlice: 0,
             destinationLevel: 1, sliceCount: 1, levelCount: 5)
```

```objective-c
[encoder copyFromTexture: source
    sourceSlice: 0
    sourceLevel: 0
    toTexture: destination
    destinationSlice: 0
    destinationLevel: 1
    sliceCount: 1
    levelCount: 5];
```

If you need to copy data between buffers and textures, encode a separate blit command for each mipmap level to copy. See [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) for other methods that copy data to and from textures.

## See also

### Texture mipmapping
- [Improving texture sampling quality and performance with mipmaps](https://developer.apple.com/documentation/metal/improving-texture-sampling-quality-and-performance-with-mipmaps) — Avoid texture-rendering artifacts and reduce the GPU’s workload by creating smaller versions of a texture.
- [Creating a mipmapped texture](https://developer.apple.com/documentation/metal/creating-a-mipmapped-texture) — Decide whether a texture that you’re creating needs mipmaps.
- [Generating mipmap data](https://developer.apple.com/documentation/metal/generating-mipmap-data) — Create your mipmaps either when you author content or at runtime.
- [Adding mipmap filtering to samplers](https://developer.apple.com/documentation/metal/adding-mipmap-filtering-to-samplers) — Specify how the GPU samples mipmaps in your textures.
- [Restricting access to specific mipmaps](https://developer.apple.com/documentation/metal/restricting-access-to-specific-mipmaps) — Set the range of mipmap levels that a sampler can access.
- [Predicting which mips the GPU samples with level-of-detail queries](https://developer.apple.com/documentation/metal/predicting-which-mips-the-gpu-samples-with-level-of-detail-queries) — Determine in advance which mipmap levels the GPU requires to sample a texture.
- [Dynamically adjusting texture level of detail](https://developer.apple.com/documentation/metal/dynamically-adjusting-texture-level-of-detail) — Defer generating or loading larger mipmaps until that level of detail is needed.
