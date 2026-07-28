# Vertex shader resource preparation commands

*API Collection*

<https://developer.apple.com/documentation/metal/vertex-shader-resource-preparation-commands>

Assign resources to vertex shaders, including buffers, textures, acceleration structures, sampler states, and function tables.

## Overview

Vertex shaders share argument tables for each resource type, such as buffers, textures, and sampler states. Each shader type has its own argument tables, separate from vertex shaders and other shader types.

## Topics

### Assigning buffers
- [setVertexBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:index:)) — Assigns a buffer to an entry in the vertex shader argument table.
- [setVertexBuffer(_:offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:attributestride:index:))
- [setVertexBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the vertex shader argument table.
- [setVertexBuffers(_:offsets:attributeStrides:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers(_:offsets:attributestrides:range:))
- [setVertexBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the vertex shader argument table.
- [setVertexBytes(_:length:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbytes(_:length:attributestride:index:))
- [setVertexBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbufferoffset(_:index:)) — Updates an entry in the vertex shader argument table with a new location within the entry’s current buffer.
- [setVertexBufferOffset(offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbufferoffset(offset:attributestride:index:))

### Assigning textures
- [setVertexTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertextexture(_:index:)) — Assigns a texture to an entry in the vertex shader argument table.
- [setVertexTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertextextures(_:range:)) — Assigns multiple textures to a range of entries in the vertex shader argument table.

### Assigning sampler states
- [setVertexSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstate(_:index:)) — Assigns a sampler state to an entry in the vertex shader argument table.
- [setVertexSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the vertex shader argument table.
- [setVertexSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the vertex shader argument table.
- [setVertexSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the vertex shader argument table.

### Assigning acceleration structures
- [setVertexAccelerationStructure(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexaccelerationstructure(_:bufferindex:)) — Assigns an acceleration structure to an entry in the vertex shader argument table.

### Assigning visible function tables
- [setVertexVisibleFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexvisiblefunctiontable(_:bufferindex:)) — Assigns a visible function table to an entry in the vertex shader argument table.
- [setVertexVisibleFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexvisiblefunctiontables(_:bufferrange:)) — Assigns multiple visible function tables to a range of entries in the vertex shader argument table.

### Assigning intersection function tables
- [setVertexIntersectionFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexintersectionfunctiontable(_:bufferindex:)) — Assigns an intersection function table to an entry in the vertex shader argument table.
- [setVertexIntersectionFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexintersectionfunctiontables(_:bufferrange:)) — Assigns multiple intersection function tables to a range of entries in the vertex shader argument table.

## See also

### Resource preparation commands
- [Mesh and object shader resource preparation commands](https://developer.apple.com/documentation/metal/mesh-and-object-shader-resource-preparation-commands) — Assign resources to mesh and object shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Fragment shader resource preparation commands](https://developer.apple.com/documentation/metal/fragment-shader-resource-preparation-commands) — Assign resources to fragment shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Tile shaders resource preparation commands](https://developer.apple.com/documentation/metal/tile-shaders-resource-preparation-commands) — Assign resources to tile shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Argument buffer resource preparation commands](https://developer.apple.com/documentation/metal/argument-buffer-resource-preparation-commands) — Load individual resources and multiple resources within a heap into GPU memory so that they’re available to shaders through argument buffers.
