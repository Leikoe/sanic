# updateTextureMappings(_:mode:regions:mipLevels:slices:numRegions:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemappings(_:mode:regions:miplevels:slices:numregions:)>

Encodes a command to update memory mappings for multiple regions inside a texture.

## Declaration

```swift
func updateTextureMappings(_ texture: any MTLTexture, mode: MTLSparseTextureMappingMode, regions: UnsafePointer<MTLRegion>, mipLevels: UnsafePointer<Int>, slices: UnsafePointer<Int>, numRegions: Int)
```

```swift
optional func updateTextureMappings(_ texture: any MTLTexture, mode: MTLSparseTextureMappingMode, regions: UnsafePointer<MTLRegion>, mipLevels: UnsafePointer<Int>, slices: UnsafePointer<Int>, numRegions: Int)
```

## Parameters

- **texture** — The sparse texture to update.
- **mode** — The change to make to the texture mapping.
- **regions** — A pointer to an array of regions to change. You need to provide as many regions as you specify in the `numRegions` parameter.
- **mipLevels** — A pointer to an array of mipmap levels to change. You need to provide as many entries as you specify in the `numRegions` parameter.
- **slices** — A pointer to an array of slices to change. You need to provide as many entries as you specify in the `numRegions` parameter.
- **numRegions** — The number of regions to update.

## See also

### Updating texture memory assignments
- [updateTextureMapping(_:mode:region:mipLevel:slice:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemapping(_:mode:region:miplevel:slice:)) — Encodes a command to update the texture mappings for a region in a single texture mipmap.
- [MTLSparseTextureMappingMode](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode) — Options for sparse texture mapping.
