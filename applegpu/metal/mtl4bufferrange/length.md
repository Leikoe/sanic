# length

*Instance Property · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtl4bufferrange/length>

## Declaration

```swift
var length: UInt64
```

## Discussion

Length of the region which begins at the given address. If the length is not known, a value of (uint64_t)-1 represents the range from the given address to the end of the buffer.
