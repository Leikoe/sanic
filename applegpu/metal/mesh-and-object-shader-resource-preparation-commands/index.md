# Mesh and object shader resource preparation commands

*API Collection*

<https://developer.apple.com/documentation/metal/mesh-and-object-shader-resource-preparation-commands>

Assign resources to mesh and object shaders, including buffers, textures, acceleration structures, sampler states, and function tables.

## Overview

Mesh shaders share argument tables for each resource type, such as buffers, textures, and sampler states. Object shaders share their own separate argument tables, distinct from mesh shaders and other shader types.

## Topics

### Assigning buffers for object shaders
- [setObjectBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffer(_:offset:index:)) — Assigns a buffer to an entry in the object shader argument table.
- [setObjectBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the object shader argument table.
- [setObjectBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the object shader argument table.
- [setObjectBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbufferoffset(_:index:)) — Updates an entry in the object shader argument table with a new location within the entry’s current buffer.

### Assigning textures for object shaders
- [setObjectTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjecttexture(_:index:)) — Assigns a texture to an entry in the object shader argument table.
- [setObjectTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjecttextures(_:range:)) — Assigns multiple textures to a range of entries in the object shader argument table.

### Assigning sampler states for object shaders
- [setObjectSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstate(_:index:)) — Assigns a sampler state to an entry in the object shader argument table.
- [setObjectSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the object shader argument table.
- [setObjectSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the object shader argument table.
- [setObjectSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the object shader argument table.

### Assigning buffers for mesh shaders
- [setMeshBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffer(_:offset:index:)) — Assigns a buffer to an entry in the mesh shader argument table.
- [setMeshBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the mesh shader argument table.
- [setMeshBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the mesh shader argument table.
- [setMeshBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbufferoffset(_:index:)) — Updates an entry in the mesh shader argument table with a new location within the entry’s current buffer.

### Assigning textures for mesh shaders
- [setMeshTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshtexture(_:index:)) — Assigns a texture to an entry in the mesh shader argument table.
- [setMeshTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshtextures(_:range:)) — Assigns multiple textures to a range of entries in the mesh shader argument table.

### Assigning sampler states for mesh shaders
- [setMeshSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstate(_:index:)) — Assigns a sampler state to an entry in the mesh shader argument table.
- [setMeshSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the mesh shader argument table.
- [setMeshSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the mesh shader argument table.
- [setMeshSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the mesh shader argument table.

## See also

### Resource preparation commands
- [Vertex shader resource preparation commands](https://developer.apple.com/documentation/metal/vertex-shader-resource-preparation-commands) — Assign resources to vertex shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Fragment shader resource preparation commands](https://developer.apple.com/documentation/metal/fragment-shader-resource-preparation-commands) — Assign resources to fragment shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Tile shaders resource preparation commands](https://developer.apple.com/documentation/metal/tile-shaders-resource-preparation-commands) — Assign resources to tile shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Argument buffer resource preparation commands](https://developer.apple.com/documentation/metal/argument-buffer-resource-preparation-commands) — Load individual resources and multiple resources within a heap into GPU memory so that they’re available to shaders through argument buffers.
