# threadgroupMemoryDataSize

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargument/threadgroupmemorydatasize>

The size, in bytes, of the threadgroup data.

## Declaration

```swift
var threadgroupMemoryDataSize: Int { get }
```

## Discussion

If the argument is not a threadgroup, querying this property is a fatal error. The Metal device determines this value.

## See also

### Describing a threadgroup memory argument
- [threadgroupMemoryAlignment](https://developer.apple.com/documentation/metal/mtlargument/threadgroupmemoryalignment) — The required byte alignment in memory for the threadgroup data.
