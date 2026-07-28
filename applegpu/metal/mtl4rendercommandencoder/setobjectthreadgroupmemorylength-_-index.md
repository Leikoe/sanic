# setObjectThreadgroupMemoryLength(_:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setobjectthreadgroupmemorylength(_:index:)>

Configures the size of a threadgroup memory buffer for a threadgroup argument in the object shader function.

## Declaration

```swift
func setObjectThreadgroupMemoryLength(_ length: Int, index: Int)
```

## Parameters

- **length** — The size of the threadgroup memory, in bytes.
- **index** — An integer that corresponds to the index of the argument you annotate with attribute `[[threadgroup(index)]]` in the shader function.

## See also

### Configuring persistent threadgroup memory
- [setThreadgroupMemoryLength(_:offset:index:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setthreadgroupmemorylength(_:offset:index:)) — Configures the size of a threadgroup memory buffer for a threadgroup argument in the fragment and tile shader functions.
