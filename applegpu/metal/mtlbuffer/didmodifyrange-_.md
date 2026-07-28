# didModifyRange(_:)

*Instance Method · Mac Catalyst 14.0, macOS 10.11*

<https://developer.apple.com/documentation/metal/mtlbuffer/didmodifyrange(_:)>

Informs the GPU that the CPU has modified a section of the buffer.

## Declaration

```swift
func didModifyRange(_ range: Range<Int>)
```

## Parameters

- **range** — The range of bytes that have been modified.

## Discussion

If you write information to a buffer created with the [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed) storage mode, you need to call this method to inform the GPU that the information has changed. If you execute GPU commands that read the data without calling this method first, the behavior is undefined.
