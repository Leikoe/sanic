# MTLCounterSampleBufferError

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct>

The error codes that indicate why a GPU driver can’t create a counter sample buffer.

## Declaration

```swift
struct MTLCounterSampleBufferError
```

## Topics

### Error code values
- [outOfMemory](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/outofmemory) — An error code that indicates the GPU device doesn’t have sufficient memory to create a counter sample buffer.
- [invalid](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/invalid) — An error code that indicates the descriptor for creating a counter sample buffer descriptor has an invalid property.
- [internal](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/internal) — An error code that indicates the Metal framework has an internal problem.
- [MTLCounterSampleBufferError.Code](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/code) — The underlying error code type that indicates why a GPU driver can’t create a counter sample buffer.

### Error domain
- [errorDomain](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/errordomain) — The current counter sample buffer error domain.
- [MTLCounterErrorDomain](https://developer.apple.com/documentation/metal/mtlcountererrordomain) — The domain for Metal counter sample buffer errors.
