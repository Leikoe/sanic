# optimizeContentsForCPUAccess(texture:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforcpuaccess(texture:)>

Encodes a command that improves the performance of CPU memory operations with a texture.

## Declaration

```swift
func optimizeContentsForCPUAccess(texture: any MTLTexture)
```

## Parameters

- **texture** — A texture the command optimizes.

## Discussion

This command can reduce the time it takes the CPU to access a texture. Apps typically run the command for:

- Textures the CPU accesses for an extended period of time

- Textures with a [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) property that’s [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) or [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed)

When a blit pass runs this command, the GPU only applies lossless changes to the texture’s underlying data.

> **Note:**
>  Optimizing a texture for the CPU may affect the performance of GPU memory operations, but the data the GPU retrieves from the texture remains consistent.

## See also

### Optimizing textures for CPU access
- [optimizeContentsForCPUAccess(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforcpuaccess(texture:slice:level:)) — Encodes a command that improves the performance of CPU memory operations with a specific portion of a texture.
