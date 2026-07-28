# label

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandencoder/label>

A string that labels the command encoder.

## Declaration

```swift
var label: String? { get set }
```

## Discussion

Object and command labels are useful identifiers at runtime or when profiling and debugging your app using any Metal tool. See [Naming resources and commands](https://developer.apple.com/documentation/Xcode/Naming-resources-and-commands).

## See also

### Identifying the command encoder
- [device](https://developer.apple.com/documentation/metal/mtlcommandencoder/device) — The Metal device from which the command encoder was created.
