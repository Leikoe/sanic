# optimizeContents(forCPUAccess:slice:level:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forcpuaccess:slice:level:)>

Encodes a command that modifies the contents of a texture to improve the performance of CPU accesses to its contents in a specific region.

## Declaration

```swift
func optimizeContents(forCPUAccess texture: any MTLTexture, slice: Int, level: Int)
```

## Parameters

- **texture** — A [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) the command optimizes for CPU access.
- **slice** — A slice within `texture`.
- **level** — A mipmap level within `texture`.

## Discussion

Optimizing a texture for CPU access may affect the performance of GPU accesses, however, the data the GPU retrieves from the texture remains consistent.

You typically use this command for:

- Textures the CPU accesses for an extended period of time.

- Textures with a [storageMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/storagemode) property that’s [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) or [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed).

## See also

### Encoding optimization commands
- [optimizeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecommands(buffer:range:)) — Encode a command to attempt to improve the performance of a range of commands within an indirect command buffer.
- [optimizeContents(forCPUAccess:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forcpuaccess:)) — Encodes a command that modifies the contents of a texture to improve the performance of CPU accesses to its contents.
- [optimizeContents(forGPUAccess:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forgpuaccess:)) — Encodes a command that modifies the contents of a texture to improve the performance of GPU accesses to its contents.
- [optimizeContents(forGPUAccess:slice:level:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forgpuaccess:slice:level:)) — Encodes a command that modifies the contents of a texture instance to improve the performance of GPU accesses to its contents in a specific region.
