# MTLSparseTextureRegionAlignmentMode

*Enumeration · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsparsetextureregionalignmentmode>

Options used when converting between a pixel-based region within a texture to a tile-based region.

## Declaration

```swift
enum MTLSparseTextureRegionAlignmentMode
```

## Topics

### Specifying the alignment mode
- [MTLSparseTextureRegionAlignmentMode.outward](https://developer.apple.com/documentation/metal/mtlsparsetextureregionalignmentmode/outward) — The tile region includes any partially covered tiles.
- [MTLSparseTextureRegionAlignmentMode.inward](https://developer.apple.com/documentation/metal/mtlsparsetextureregionalignmentmode/inward) — The tile region ignores partially covered tiles.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlsparsetextureregionalignmentmode/init(rawvalue:))

## See also

### Working with sparse textures
- [sparseTileSize(textureType:pixelFormat:sampleCount:sparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(texturetype:pixelformat:samplecount:sparsepagesize:)) — Returns the dimensions of a sparse tile for a texture that has a specific sparse page size.
- [sparseTileSize(with:pixelFormat:sampleCount:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(with:pixelformat:samplecount:)) — Returns the dimensions of a sparse tile for a texture.
- [sparseTileSizeInBytes(sparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes(sparsepagesize:)) — Returns the size, in bytes, of a sparse tile the GPU device creates with a specific page size.
- [sparseTileSizeInBytes](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes) — Returns the size, in bytes, of a sparse tile the GPU device creates using a default page size.
- [convertSparsePixelRegions(_:toTileRegions:withTileSize:alignmentMode:numRegions:)](https://developer.apple.com/documentation/metal/mtldevice/convertsparsepixelregions(_:totileregions:withtilesize:alignmentmode:numregions:)) — Converts a list of sparse pixel regions to tile regions.
- [convertSparseTileRegions(_:toPixelRegions:withTileSize:numRegions:)](https://developer.apple.com/documentation/metal/mtldevice/convertsparsetileregions(_:topixelregions:withtilesize:numregions:)) — Converts a list of sparse tile regions to pixel regions.
- [MTLSparsePageSize](https://developer.apple.com/documentation/metal/mtlsparsepagesize) — The page size options, in kilobytes, for sparse textures.
