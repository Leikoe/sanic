# convertSparseTileRegions(_:toPixelRegions:withTileSize:numRegions:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/convertsparsetileregions(_:topixelregions:withtilesize:numregions:)>

Converts a list of sparse tile regions to pixel regions.

## Declaration

```swift
optional func convertSparseTileRegions(_ tileRegions: UnsafePointer<MTLRegion>, toPixelRegions pixelRegions: UnsafeMutablePointer<MTLRegion>, withTileSize tileSize: MTLSize, numRegions: Int)
```

## Parameters

- **tileRegions** — A pointer to a C array of tile [MTLRegion](https://developer.apple.com/documentation/metal/mtlregion) instances.
- **pixelRegions** — A pointer to a C array of pixel [MTLRegion](https://developer.apple.com/documentation/metal/mtlregion) instances.
- **tileSize** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents a sparse tile’s size, in pixels.
- **numRegions** — The number of regions you want the method to convert.

## See also

### Working with sparse textures
- [sparseTileSize(textureType:pixelFormat:sampleCount:sparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(texturetype:pixelformat:samplecount:sparsepagesize:)) — Returns the dimensions of a sparse tile for a texture that has a specific sparse page size.
- [sparseTileSize(with:pixelFormat:sampleCount:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(with:pixelformat:samplecount:)) — Returns the dimensions of a sparse tile for a texture.
- [sparseTileSizeInBytes(sparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes(sparsepagesize:)) — Returns the size, in bytes, of a sparse tile the GPU device creates with a specific page size.
- [sparseTileSizeInBytes](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes) — Returns the size, in bytes, of a sparse tile the GPU device creates using a default page size.
- [convertSparsePixelRegions(_:toTileRegions:withTileSize:alignmentMode:numRegions:)](https://developer.apple.com/documentation/metal/mtldevice/convertsparsepixelregions(_:totileregions:withtilesize:alignmentmode:numregions:)) — Converts a list of sparse pixel regions to tile regions.
- [MTLSparsePageSize](https://developer.apple.com/documentation/metal/mtlsparsepagesize) — The page size options, in kilobytes, for sparse textures.
- [MTLSparseTextureRegionAlignmentMode](https://developer.apple.com/documentation/metal/mtlsparsetextureregionalignmentmode) — Options used when converting between a pixel-based region within a texture to a tile-based region.
