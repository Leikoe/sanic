# makeArchive(url:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/makearchive(url:)>

Creates a new archive from data available at an `NSURL` address.

## Declaration

```swift
func makeArchive(url: URL) throws -> any MTL4Archive
```

## Parameters

- **url** — An `NSURL` instance that represents the path from which the device loads the [MTL4Archive](https://developer.apple.com/documentation/metal/mtl4archive).

## Return Value

A [MTL4Archive](https://developer.apple.com/documentation/metal/mtl4archive) instance, or `nil` if the function failed.
