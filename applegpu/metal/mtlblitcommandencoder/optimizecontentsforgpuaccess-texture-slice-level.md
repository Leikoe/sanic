# optimizeContentsForGPUAccess(texture:slice:level:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforgpuaccess(texture:slice:level:)>

Encodes a command that improves the performance of GPU memory operations with a specific portion of a texture.

## Declaration

```swift
func optimizeContentsForGPUAccess(texture: any MTLTexture, slice: Int, level: Int)
```

## Parameters

- **texture** — A texture the command optimizes.
- **slice** — A slice within `texture`.
- **level** — A mipmap level within `texture`.

## Discussion

This command can reduce the time it takes the GPU to access a texture. Apps typically run the command for:

- Textures the GPU accesses for an extended period of time

- Textures with a [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) property that’s [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) or [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed)

When a blit pass runs this command, the GPU only applies lossless changes to the texture’s underlying data.

> **Note:**
>  Optimizing a texture for the GPU may affect the performance of CPU memory operations, but the data the CPU retrieves from the texture remains consistent.

## See also

### Optimizing textures for GPU access
- [optimizeContentsForGPUAccess(texture:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforgpuaccess(texture:)) — Encodes a command that improves the performance of GPU memory operations with a texture.
