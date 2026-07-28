# sparseTileSize(with:pixelFormat:sampleCount:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(with:pixelformat:samplecount:)>

Returns the dimensions of a sparse tile for a texture.

## Declaration

```swift
func sparseTileSize(with textureType: MTLTextureType, pixelFormat: MTLPixelFormat, sampleCount: Int) -> MTLSize
```

## Parameters

- **textureType** — An [MTLTextureType](https://developer.apple.com/documentation/metal/mtltexturetype) instance.
- **pixelFormat** — An [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat) instance.
- **sampleCount** — The number of samples for each pixel.

## Return Value

A new [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance.

## Discussion

The size of a sparse tile, in bytes, is the same for all sparse textures on a GPU device object. Because the size of pixels may vary, the actual dimensions of a sparse tile vary based on the texture and the pixel format. Use this method to get the dimensions of the tile for a particular format. Use these dimensions when converting regions from pixel-based units to sparse tile units and vice versa.

## See also

### Working with sparse textures
- [sparseTileSize(textureType:pixelFormat:sampleCount:sparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(texturetype:pixelformat:samplecount:sparsepagesize:)) — Returns the dimensions of a sparse tile for a texture that has a specific sparse page size.
- [sparseTileSizeInBytes(sparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes(sparsepagesize:)) — Returns the size, in bytes, of a sparse tile the GPU device creates with a specific page size.
- [sparseTileSizeInBytes](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes) — Returns the size, in bytes, of a sparse tile the GPU device creates using a default page size.
- [convertSparsePixelRegions(_:toTileRegions:withTileSize:alignmentMode:numRegions:)](https://developer.apple.com/documentation/metal/mtldevice/convertsparsepixelregions(_:totileregions:withtilesize:alignmentmode:numregions:)) — Converts a list of sparse pixel regions to tile regions.
- [convertSparseTileRegions(_:toPixelRegions:withTileSize:numRegions:)](https://developer.apple.com/documentation/metal/mtldevice/convertsparsetileregions(_:topixelregions:withtilesize:numregions:)) — Converts a list of sparse tile regions to pixel regions.
- [MTLSparsePageSize](https://developer.apple.com/documentation/metal/mtlsparsepagesize) — The page size options, in kilobytes, for sparse textures.
- [MTLSparseTextureRegionAlignmentMode](https://developer.apple.com/documentation/metal/mtlsparsetextureregionalignmentmode) — Options used when converting between a pixel-based region within a texture to a tile-based region.
