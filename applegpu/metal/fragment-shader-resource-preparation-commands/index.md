# Fragment shader resource preparation commands

*API Collection*

<https://developer.apple.com/documentation/metal/fragment-shader-resource-preparation-commands>

Assign resources to fragment shaders, including buffers, textures, acceleration structures, sampler states, and function tables.

## Overview

Fragment shaders share argument tables for each resource type, such as buffers, textures, and sampler states. Each shader type has its own argument tables, separate from fragment shaders and other shader types.

## Topics

### Assigning buffers
- [setFragmentBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbuffer(_:offset:index:)) — Assigns a buffer to an entry in the fragment shader argument table.
- [setFragmentBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the fragment shader argument table.
- [setFragmentBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the fragment shader argument table.
- [setFragmentBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbufferoffset(_:index:)) — Updates an entry in the fragment shader argument table with a new location within the entry’s current buffer.

### Assigning textures
- [setFragmentTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmenttexture(_:index:)) — Assigns a texture to an entry in the fragment shader argument table.
- [setFragmentTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmenttextures(_:range:)) — Assigns multiple textures to a range of entries in the fragment shader argument table.

### Assigning sampler states
- [setFragmentSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstate(_:index:)) — Assigns a sampler state to an entry in the fragment shader argument table.
- [setFragmentSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the fragment shader argument table.
- [setFragmentSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the fragment shader argument table.
- [setFragmentSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the fragment shader argument table.

### Assigning acceleration structures
- [setFragmentAccelerationStructure(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentaccelerationstructure(_:bufferindex:)) — Assigns an acceleration structure to an entry in the fragment shader argument table.

### Assigning visible function tables
- [setFragmentVisibleFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentvisiblefunctiontable(_:bufferindex:)) — Assigns a visible function table to an entry in the fragment shader argument table.
- [setFragmentVisibleFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentvisiblefunctiontables(_:bufferrange:)) — Assigns multiple visible function tables to a range of entries in the fragment shader argument table.

### Assigning intersection function tables
- [setFragmentIntersectionFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentintersectionfunctiontable(_:bufferindex:)) — Assigns an intersection function table to an entry in the fragment shader argument table.
- [setFragmentIntersectionFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentintersectionfunctiontables(_:bufferrange:)) — Assigns multiple intersection function tables to a range of entries in the fragment shader argument table.

## See also

### Resource preparation commands
- [Mesh and object shader resource preparation commands](https://developer.apple.com/documentation/metal/mesh-and-object-shader-resource-preparation-commands) — Assign resources to mesh and object shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Vertex shader resource preparation commands](https://developer.apple.com/documentation/metal/vertex-shader-resource-preparation-commands) — Assign resources to vertex shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Tile shaders resource preparation commands](https://developer.apple.com/documentation/metal/tile-shaders-resource-preparation-commands) — Assign resources to tile shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Argument buffer resource preparation commands](https://developer.apple.com/documentation/metal/argument-buffer-resource-preparation-commands) — Load individual resources and multiple resources within a heap into GPU memory so that they’re available to shaders through argument buffers.
