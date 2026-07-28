# copyMappings(sourceTexture:destinationTexture:operations:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/copymappings(sourcetexture:destinationtexture:operations:)>

Copies multiple regions within a source placement sparse texture to a destination placement sparse texture.

## Declaration

```swift
func copyMappings(sourceTexture: any MTLTexture, destinationTexture: any MTLTexture, operations: [MTL4CopySparseTextureMappingOperation])
```

## Parameters

- **sourceTexture** — The source placement sparse [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture).
- **destinationTexture** — The destination placement sparse [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture).
- **operations** — An array of [MTL4CopySparseTextureMappingOperation](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation) instances to perform.

## Discussion

You are responsible for ensuring the source and destination textures have the same [placementSparsePageSize](https://developer.apple.com/documentation/metal/mtltexturedescriptor/placementsparsepagesize).

Additionally, you are responsible for ensuring that the source and destination textures don’t use the same aliased tiles at the same time.

> **Note:**
> If a sparse texture and a sparse buffer share the same backing tiles, these don’t provide you you with meaningful views of the other resource’s data.
