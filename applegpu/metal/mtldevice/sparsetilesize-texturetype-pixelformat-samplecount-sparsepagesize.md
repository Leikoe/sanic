# sparseTileSize(textureType:pixelFormat:sampleCount:sparsePageSize:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(texturetype:pixelformat:samplecount:sparsepagesize:)>

Returns the dimensions of a sparse tile for a texture that has a specific sparse page size.

## Declaration

```swift
func sparseTileSize(textureType: MTLTextureType, pixelFormat: MTLPixelFormat, sampleCount: Int, sparsePageSize: MTLSparsePageSize) -> MTLSize
```

## Parameters

- **textureType** — An [MTLTextureType](https://developer.apple.com/documentation/metal/mtltexturetype) instance.
- **pixelFormat** — An [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat) instance.
- **sampleCount** — The number of samples for each pixel.
- **sparsePageSize** — An [MTLSparsePageSize](https://developer.apple.com/documentation/metal/mtlsparsepagesize) instance.

## Return Value

A new [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance.

## See also

### Working with sparse textures
- [sparseTileSize(with:pixelFormat:sampleCount:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(with:pixelformat:samplecount:)) — Returns the dimensions of a sparse tile for a texture.
- [sparseTileSizeInBytes(sparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes(sparsepagesize:)) — Returns the size, in bytes, of a sparse tile the GPU device creates with a specific page size.
- [sparseTileSizeInBytes](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes) — Returns the size, in bytes, of a sparse tile the GPU device creates using a default page size.
- [convertSparsePixelRegions(_:toTileRegions:withTileSize:alignmentMode:numRegions:)](https://developer.apple.com/documentation/metal/mtldevice/convertsparsepixelregions(_:totileregions:withtilesize:alignmentmode:numregions:)) — Converts a list of sparse pixel regions to tile regions.
- [convertSparseTileRegions(_:toPixelRegions:withTileSize:numRegions:)](https://developer.apple.com/documentation/metal/mtldevice/convertsparsetileregions(_:topixelregions:withtilesize:numregions:)) — Converts a list of sparse tile regions to pixel regions.
- [MTLSparsePageSize](https://developer.apple.com/documentation/metal/mtlsparsepagesize) — The page size options, in kilobytes, for sparse textures.
- [MTLSparseTextureRegionAlignmentMode](https://developer.apple.com/documentation/metal/mtlsparsetextureregionalignmentmode) — Options used when converting between a pixel-based region within a texture to a tile-based region.
