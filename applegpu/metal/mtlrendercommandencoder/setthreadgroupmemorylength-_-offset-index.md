# setThreadgroupMemoryLength(_:offset:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setthreadgroupmemorylength(_:offset:index:)>

Configures the size of a threadgroup memory buffer for an entry in the fragment or tile shader argument table.

## Declaration

```swift
func setThreadgroupMemoryLength(_ length: Int, offset: Int, index: Int)
```

## Parameters

- **length** — The threadgroup memory length, in bytes.
- **offset** — An integer that represents the location, in bytes, from the start of the buffer at `index` where the threadgroup memory begins.
- **index** — An integer that represents an entry in the buffer argument table.

## Discussion

You can only change the threadgroup memory’s size between tile dispatches (see [dispatchThreadsPerTile(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/dispatchthreadspertile(_:))).

> **Important:**
>  Exceeding the threadgroup memory allocation for the render pass can trigger a debug error.

## See also

### Configuring persistent threadgroup memory
- [setObjectThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectthreadgroupmemorylength(_:index:)) — Configures the size of a threadgroup memory buffer for an entry in the object argument table.
