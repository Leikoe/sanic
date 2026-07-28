# MTLIOCompressionStatus

*Enumeration · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocompressionstatus>

Represents the final state of a compression context.

## Declaration

```swift
enum MTLIOCompressionStatus
```

## Overview

The [MTLIOFlushAndDestroyCompressionContext(_:)](https://developer.apple.com/documentation/metal/mtlioflushanddestroycompressioncontext(_:)) returns an [MTLIOCompressionStatus](https://developer.apple.com/documentation/metal/mtliocompressionstatus) instance to reflect the final state of a compression context.

## Topics

### Compression result states
- [MTLIOCompressionStatus.complete](https://developer.apple.com/documentation/metal/mtliocompressionstatus/complete) — Indicates the compression API successfully flushed and destroyed a compression context.
- [MTLIOCompressionStatus.error](https://developer.apple.com/documentation/metal/mtliocompressionstatus/error) — Indicates the compression API had an error while flushing and destroying a compression context.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtliocompressionstatus/init(rawvalue:))

## See also

### Asset compression
- [MTLIOCreateCompressionContext(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocreatecompressioncontext(_:_:_:)) — Creates a compression context that you use to compress data into a single file.
- [MTLIOCompressionMethod](https://developer.apple.com/documentation/metal/mtliocompressionmethod) — The compression codecs that Metal supports for input/output handles.
- [MTLIOCompressionContextDefaultChunkSize()](https://developer.apple.com/documentation/metal/mtliocompressioncontextdefaultchunksize()) — Returns a compression chunk size you can use as a default for creating a compression context.
- [MTLIOCompressionContext](https://developer.apple.com/documentation/metal/mtliocompressioncontext) — A pointer that represents the state of a file compression session in progress.
- [MTLIOCompressionContextAppendData(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocompressioncontextappenddata(_:_:_:)) — Adds data to a compression context.
- [MTLIOFlushAndDestroyCompressionContext(_:)](https://developer.apple.com/documentation/metal/mtlioflushanddestroycompressioncontext(_:)) — Finishes compressing and saves the file that a compression context represents.
