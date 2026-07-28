# MTLIOCompressionContextAppendData(_:_:_:)

*Function · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocompressioncontextappenddata(_:_:_:)>

Adds data to a compression context.

## Declaration

```swift
func MTLIOCompressionContextAppendData(_ context: MTLIOCompressionContext, _ data: UnsafeRawPointer, _ size: Int)
```

## Parameters

- **context** — An [MTLIOCompressionContext](https://developer.apple.com/documentation/metal/mtliocompressioncontext) instance that you create with the [MTLIOCreateCompressionContext(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocreatecompressioncontext(_:_:_:)) function.
- **data** — A pointer to memory that contains the data the function adds to the compression context.
- **size** — The number of bytes the function adds to the compression context from the data pointer.

## See also

### Asset compression
- [MTLIOCreateCompressionContext(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocreatecompressioncontext(_:_:_:)) — Creates a compression context that you use to compress data into a single file.
- [MTLIOCompressionMethod](https://developer.apple.com/documentation/metal/mtliocompressionmethod) — The compression codecs that Metal supports for input/output handles.
- [MTLIOCompressionContextDefaultChunkSize()](https://developer.apple.com/documentation/metal/mtliocompressioncontextdefaultchunksize()) — Returns a compression chunk size you can use as a default for creating a compression context.
- [MTLIOCompressionContext](https://developer.apple.com/documentation/metal/mtliocompressioncontext) — A pointer that represents the state of a file compression session in progress.
- [MTLIOFlushAndDestroyCompressionContext(_:)](https://developer.apple.com/documentation/metal/mtlioflushanddestroycompressioncontext(_:)) — Finishes compressing and saves the file that a compression context represents.
- [MTLIOCompressionStatus](https://developer.apple.com/documentation/metal/mtliocompressionstatus) — Represents the final state of a compression context.
