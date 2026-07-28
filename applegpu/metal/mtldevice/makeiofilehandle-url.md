# makeIOFileHandle(url:)

*Instance Method · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makeiofilehandle(url:)>

Creates an input/output file handle instance that represents a file at a URL.

## Declaration

```swift
func makeIOFileHandle(url: URL) throws -> any MTLIOFileHandle
```

## Parameters

- **url** — The URL to a resource file in the file system.

## Return Value

A new [MTLIOFileHandle](https://developer.apple.com/documentation/metal/mtliofilehandle) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## Discussion

For information about using input/output command queues and file handles, see [Resource loading](https://developer.apple.com/documentation/metal/resource-loading).

## See also

### Creating I/O file handles
- [makeIOFileHandle(url:compressionMethod:)](https://developer.apple.com/documentation/metal/mtldevice/makeiofilehandle(url:compressionmethod:)) — Creates an input/output file handle instance that represents a compressed file at a URL.
- [makeIOHandle(url:)](https://developer.apple.com/documentation/metal/mtldevice/makeiohandle(url:)) — Creates an input/output file handle instance that represents a file at a URL.
- [makeIOHandle(url:compressionMethod:)](https://developer.apple.com/documentation/metal/mtldevice/makeiohandle(url:compressionmethod:)) — Creates an input/output file handle instance that represents a compressed file at a URL.
