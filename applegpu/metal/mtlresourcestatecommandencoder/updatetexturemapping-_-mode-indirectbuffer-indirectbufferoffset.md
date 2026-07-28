# updateTextureMapping(_:mode:indirectBuffer:indirectBufferOffset:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemapping(_:mode:indirectbuffer:indirectbufferoffset:)>

Encodes a command to update a texture’s memory mappings, specifying the parameters indirectly.

## Declaration

```swift
func updateTextureMapping(_ texture: any MTLTexture, mode: MTLSparseTextureMappingMode, indirectBuffer: any MTLBuffer, indirectBufferOffset: Int)
```

```swift
optional func updateTextureMapping(_ texture: any MTLTexture, mode: MTLSparseTextureMappingMode, indirectBuffer: any MTLBuffer, indirectBufferOffset: Int)
```

## Parameters

- **texture** — The sparse texture to update.
- **mode** — A mode that indicates whether the method allocates or frees a memory tile in the texture.
- **indirectBuffer** — A buffer that contains an array of mapping arguments that are instances of the [MTLMapIndirectArguments](https://developer.apple.com/documentation/metal/mtlmapindirectarguments) structure.
- **indirectBufferOffset** — The offset, in bytes, where the first argument begins in the `indirectBuffer` parameter.

## Discussion

When the GPU executes the command that updates the texture’s memory mapping, the GPU gets details about the region to update from the `indirectBuffer` parameter.

To allocate tiles from the heap, pass [MTLSparseTextureMappingMode.map](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode/map) as the `mode` parameter, and to free files back to the heap, pass [MTLSparseTextureMappingMode.unmap](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode/unmap).

If you encode other commands that use the texture’s contents, such as rendering to the texture or sampling from a texture, synchronize the texture’s mapping updates with those commands to avoid race conditions. See [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

If you encode commands with multiple resource state passes, synchronize the resources to run the commands in the passes sequentially. See the [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) protocol.
