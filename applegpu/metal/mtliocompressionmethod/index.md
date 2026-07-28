# MTLIOCompressionMethod

*Enumeration · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtliocompressionmethod>

The compression codecs that Metal supports for input/output handles.

## Declaration

```swift
enum MTLIOCompressionMethod
```

## Overview

For more information on the individual codecs, see the [Algorithm](https://developer.apple.com/documentation/Compression/Algorithm) enumeration in the [Compression](https://developer.apple.com/documentation/Compression) framework.

## Topics

### Compression codecs
- [MTLIOCompressionMethod.zlib](https://developer.apple.com/documentation/metal/mtliocompressionmethod/zlib) — Indicates that a file uses the zlib compression algorithm codec.
- [MTLIOCompressionMethod.lzfse](https://developer.apple.com/documentation/metal/mtliocompressionmethod/lzfse) — Indicates that a file uses the LZFSE compression algorithm codec.
- [MTLIOCompressionMethod.lz4](https://developer.apple.com/documentation/metal/mtliocompressionmethod/lz4) — Indicates that a file uses the LZ4 compression algorithm codec.
- [MTLIOCompressionMethod.lzma](https://developer.apple.com/documentation/metal/mtliocompressionmethod/lzma) — Indicates that a file uses the LZMA compression algorithm codec.
- [MTLIOCompressionMethod.lzBitmap](https://developer.apple.com/documentation/metal/mtliocompressionmethod/lzbitmap) — Indicates that a file uses the LZBitmap compression algorithm codec.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtliocompressionmethod/init(rawvalue:))

## See also

### Asset compression
- [MTLIOCreateCompressionContext(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocreatecompressioncontext(_:_:_:)) — Creates a compression context that you use to compress data into a single file.
- [MTLIOCompressionContextDefaultChunkSize()](https://developer.apple.com/documentation/metal/mtliocompressioncontextdefaultchunksize()) — Returns a compression chunk size you can use as a default for creating a compression context.
- [MTLIOCompressionContext](https://developer.apple.com/documentation/metal/mtliocompressioncontext) — A pointer that represents the state of a file compression session in progress.
- [MTLIOCompressionContextAppendData(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocompressioncontextappenddata(_:_:_:)) — Adds data to a compression context.
- [MTLIOFlushAndDestroyCompressionContext(_:)](https://developer.apple.com/documentation/metal/mtlioflushanddestroycompressioncontext(_:)) — Finishes compressing and saves the file that a compression context represents.
- [MTLIOCompressionStatus](https://developer.apple.com/documentation/metal/mtliocompressionstatus) — Represents the final state of a compression context.
