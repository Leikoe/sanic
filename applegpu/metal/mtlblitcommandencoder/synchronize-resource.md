# synchronize(resource:)

*Instance Method · Mac Catalyst 13.0, macOS 10.11*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(resource:)>

Encodes a command that synchronizes the CPU’s copy of a managed resource, such as a buffer or texture, so that it matches the GPU’s copy.

## Declaration

```swift
func synchronize(resource: any MTLResource)
```

## Parameters

- **resource** — An [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) instance — such as an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) or [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) — with a [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) property that’s equal to [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed).

## Discussion

This method ensures the CPU can correctly read all the changes a GPU makes to a resource that uses the managed storage mode. For the resources you create with [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed), the CPU and GPU each have a copy of that resource. As the GPU modifies its copy, the CPU’s copy remains unchanged until you synchronize with a command, such as this one.

The CPU can access the updated content from its copy of the resources after the synchronization command completes.

> **Note:**
>  You can encode a command that selectively synchronizes parts of an [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) by calling the [synchronize(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(texture:slice:level:)) method.

## See also

### Synchronizing managed resources
- [synchronize(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(texture:slice:level:)) — Encodes a command that synchronizes a part of the CPU’s copy of a texture so that it matches the GPU’s copy.
