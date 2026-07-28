# MTLIOCreateCompressionContext(_:_:_:)

*Function · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtliocreatecompressioncontext(_:_:_:)>

Creates a compression context that you use to compress data into a single file.

## Declaration

```swift
func MTLIOCreateCompressionContext(_ path: String, _ type: MTLIOCompressionMethod, _ chunkSize: Int) -> MTLIOCompressionContext?
```

## Parameters

- **path** — A location in the file system where the function creates the new, compressed file.
- **type** — A compression codec the function uses to compress data resource file’s compression format.
- **chunkSize** — The number of uncompressed bytes the compression codec compresses at a time.

## See also

### Asset compression
- [MTLIOCompressionMethod](https://developer.apple.com/documentation/metal/mtliocompressionmethod) — The compression codecs that Metal supports for input/output handles.
- [MTLIOCompressionContextDefaultChunkSize()](https://developer.apple.com/documentation/metal/mtliocompressioncontextdefaultchunksize()) — Returns a compression chunk size you can use as a default for creating a compression context.
- [MTLIOCompressionContext](https://developer.apple.com/documentation/metal/mtliocompressioncontext) — A pointer that represents the state of a file compression session in progress.
- [MTLIOCompressionContextAppendData(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocompressioncontextappenddata(_:_:_:)) — Adds data to a compression context.
- [MTLIOFlushAndDestroyCompressionContext(_:)](https://developer.apple.com/documentation/metal/mtlioflushanddestroycompressioncontext(_:)) — Finishes compressing and saves the file that a compression context represents.
- [MTLIOCompressionStatus](https://developer.apple.com/documentation/metal/mtliocompressionstatus) — Represents the final state of a compression context.
