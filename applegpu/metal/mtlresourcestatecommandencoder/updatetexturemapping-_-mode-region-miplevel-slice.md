# updateTextureMapping(_:mode:region:mipLevel:slice:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemapping(_:mode:region:miplevel:slice:)>

Encodes a command to update the texture mappings for a region in a single texture mipmap.

## Declaration

```swift
func updateTextureMapping(_ texture: any MTLTexture, mode: MTLSparseTextureMappingMode, region: MTLRegion, mipLevel: Int, slice: Int)
```

```swift
optional func updateTextureMapping(_ texture: any MTLTexture, mode: MTLSparseTextureMappingMode, region: MTLRegion, mipLevel: Int, slice: Int)
```

## Parameters

- **texture** — The sparse texture to update.
- **mode** — A mode that indicates whether the method allocates or frees a memory tile in the texture.
- **region** — A region, in tile coordinates, that describes the part of the mipmap to update.
- **mipLevel** — The mipmap to update.
- **slice** — The slice in the texture to update.

## Discussion

When the GPU executes the command that updates the texture’s memory mapping, the GPU gets details about the region from the `region` parameter.

To allocate tiles from the heap, pass [MTLSparseTextureMappingMode.map](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode/map) as the `mode` parameter, and to free files back to the heap, pass [MTLSparseTextureMappingMode.unmap](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode/unmap).

If you encode other commands that use the texture’s contents, such as rendering to the texture or sampling from a texture, synchronize the texture’s mapping updates with those commands to avoid race conditions. See [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

If you encode commands with multiple resource state passes, synchronize the resources to run the commands in the passes sequentially. See the [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) protocol.

## See also

### Updating texture memory assignments
- [updateTextureMappings(_:mode:regions:mipLevels:slices:numRegions:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemappings(_:mode:regions:miplevels:slices:numregions:)) — Encodes a command to update memory mappings for multiple regions inside a texture.
- [MTLSparseTextureMappingMode](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode) — Options for sparse texture mapping.
