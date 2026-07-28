# Tile shaders resource preparation commands

*API Collection*

<https://developer.apple.com/documentation/metal/tile-shaders-resource-preparation-commands>

Assign resources to tile shaders, including buffers, textures, acceleration structures, sampler states, and function tables.

## Overview

Tile shaders share argument tables for each resource type, such as buffers, textures, and sampler states. Each shader type has its own argument tables, separate from tile shaders and other shader types.

## Topics

### Assigning buffers
- [setTileBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffer(_:offset:index:)) — Assigns a buffer to an entry in the tile shader argument table.
- [setTileBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the tile shader argument table.
- [setTileBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the tile shader argument table.
- [setTileBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebufferoffset(_:index:)) — Updates an entry in the tile shader argument table with a new location within the entry’s current buffer.

### Assigning textures
- [setTileTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settiletexture(_:index:)) — Assigns a texture to an entry in the tile shader argument table.
- [setTileTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settiletextures(_:range:)) — Assigns multiple textures to a range of entries in the tile shader argument table.

### Assigning sampler states
- [setTileSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstate(_:index:)) — Assigns a sampler state to an entry in the tile shader argument table.
- [setTileSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the tile shader argument table.
- [setTileSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the tile shader argument table.
- [setTileSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the tile shader argument table.

### Assigning acceleration structures
- [setTileAccelerationStructure(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settileaccelerationstructure(_:bufferindex:)) — Assigns an acceleration structure to an entry in the tile shader argument table.

### Assigning visible function tables
- [setTileVisibleFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilevisiblefunctiontable(_:bufferindex:)) — Assigns a visible function table to an entry in the tile shader argument table.
- [setTileVisibleFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilevisiblefunctiontables(_:bufferrange:)) — Assigns multiple visible function tables to a range of entries in the tile shader argument table.

### Assigning intersection function tables
- [setTileIntersectionFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settileintersectionfunctiontable(_:bufferindex:)) — Assigns an intersection function table to an entry in the tile shader argument table.
- [setTileIntersectionFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settileintersectionfunctiontables(_:bufferrange:)) — Assigns multiple intersection function tables to a range of entries in the tile shader argument table.

## See also

### Resource preparation commands
- [Mesh and object shader resource preparation commands](https://developer.apple.com/documentation/metal/mesh-and-object-shader-resource-preparation-commands) — Assign resources to mesh and object shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Vertex shader resource preparation commands](https://developer.apple.com/documentation/metal/vertex-shader-resource-preparation-commands) — Assign resources to vertex shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Fragment shader resource preparation commands](https://developer.apple.com/documentation/metal/fragment-shader-resource-preparation-commands) — Assign resources to fragment shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Argument buffer resource preparation commands](https://developer.apple.com/documentation/metal/argument-buffer-resource-preparation-commands) — Load individual resources and multiple resources within a heap into GPU memory so that they’re available to shaders through argument buffers.
