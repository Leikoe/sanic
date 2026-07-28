# MTLSparseTextureMappingMode

*Enumeration · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode>

Options for sparse texture mapping.

## Declaration

```swift
enum MTLSparseTextureMappingMode
```

## Topics

### Specifying the mapping mode
- [MTLSparseTextureMappingMode.map](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode/map) — A request to map sparse tiles from the heap to a region in the texture.
- [MTLSparseTextureMappingMode.unmap](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode/unmap) — A request to remove any mappings for a region in the texture.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode/init(rawvalue:))

## See also

### Updating texture memory assignments
- [updateTextureMapping(_:mode:region:mipLevel:slice:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemapping(_:mode:region:miplevel:slice:)) — Encodes a command to update the texture mappings for a region in a single texture mipmap.
- [updateTextureMappings(_:mode:regions:mipLevels:slices:numRegions:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemappings(_:mode:regions:miplevels:slices:numregions:)) — Encodes a command to update memory mappings for multiple regions inside a texture.
