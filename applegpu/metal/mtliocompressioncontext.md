# MTLIOCompressionContext

*Type Alias · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtliocompressioncontext>

A pointer that represents the state of a file compression session in progress.

## Declaration

```swift
typealias MTLIOCompressionContext = UnsafeMutableRawPointer
```

## See also

### Asset compression
- [MTLIOCreateCompressionContext(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocreatecompressioncontext(_:_:_:)) — Creates a compression context that you use to compress data into a single file.
- [MTLIOCompressionMethod](https://developer.apple.com/documentation/metal/mtliocompressionmethod) — The compression codecs that Metal supports for input/output handles.
- [MTLIOCompressionContextDefaultChunkSize()](https://developer.apple.com/documentation/metal/mtliocompressioncontextdefaultchunksize()) — Returns a compression chunk size you can use as a default for creating a compression context.
- [MTLIOCompressionContextAppendData(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocompressioncontextappenddata(_:_:_:)) — Adds data to a compression context.
- [MTLIOFlushAndDestroyCompressionContext(_:)](https://developer.apple.com/documentation/metal/mtlioflushanddestroycompressioncontext(_:)) — Finishes compressing and saves the file that a compression context represents.
- [MTLIOCompressionStatus](https://developer.apple.com/documentation/metal/mtliocompressionstatus) — Represents the final state of a compression context.
