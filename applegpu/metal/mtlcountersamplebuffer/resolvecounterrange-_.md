# resolveCounterRange(_:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcountersamplebuffer/resolvecounterrange(_:)>

Transforms samples of a GPU’s counter set from the driver’s internal format to a standard Metal data structure.

## Declaration

```swift
func resolveCounterRange(_ range: Range<Int>) throws -> Data?
```

## Parameters

- **range** — A range that indicates which sample instances the method resolves in the counter sample buffer.

## Return Value

A [Data](https://developer.apple.com/documentation/Foundation/Data) instance in Swift, or an [NSData](https://developer.apple.com/documentation/Foundation/NSData) instance in Objective-C, if the method successfully resolves the range of samples in the buffer; otherwise, `nil`.

## Discussion

You can only call this method on a counter sample buffer that you create with [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) (see [storageMode](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/storagemode)). For an example of how and when to use this method, see [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format).

> **Note:**
>  The GPU stores [MTLCounterErrorValue](https://developer.apple.com/documentation/metal/mtlcountererrorvalue) in `destinationBuffer` each time it encounters an error resolving a sample.
