# synchronize(texture:slice:level:)

*Instance Method · Mac Catalyst 13.0, macOS 10.11*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(texture:slice:level:)>

Encodes a command that synchronizes a part of the CPU’s copy of a texture so that it matches the GPU’s copy.

## Declaration

```swift
func synchronize(texture: any MTLTexture, slice: Int, level: Int)
```

## Parameters

- **texture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance with a [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) property that’s equal to [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed).
- **slice** — A slice within `texture`.
- **level** — A mipmap level within `texture`.

## Discussion

This method ensures the CPU can correctly read the changes a GPU makes to a slice of a texture that uses the managed storage mode. For the resources you create with [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed), the CPU and GPU each have a copy of that resource. As the GPU modifies its copy, the CPU’s copy remains unchanged until you synchronize with a command, such as this one.

The CPU can access the updated content from its copy of the texture after the synchronization command completes.

> **Note:**
>  The command this method encodes behaves similarly to the command that [synchronize(resource:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(resource:)) encodes, except that it flushes only the applicable slice and mipmap level.

## See also

### Synchronizing managed resources
- [synchronize(resource:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(resource:)) — Encodes a command that synchronizes the CPU’s copy of a managed resource, such as a buffer or texture, so that it matches the GPU’s copy.
