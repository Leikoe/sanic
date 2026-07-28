# setObjectThreadgroupMemoryLength(_:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectthreadgroupmemorylength(_:index:)>

Configures the size of a threadgroup memory buffer for an entry in the object argument table.

## Declaration

```swift
func setObjectThreadgroupMemoryLength(_ length: Int, index: Int)
```

## Parameters

- **length** — The threadgroup memory length, in bytes.
- **index** — An integer that represents an entry in the object argument table.

## See also

### Configuring persistent threadgroup memory
- [setThreadgroupMemoryLength(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setthreadgroupmemorylength(_:offset:index:)) — Configures the size of a threadgroup memory buffer for an entry in the fragment or tile shader argument table.
