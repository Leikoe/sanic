# outputURL

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturedescriptor/outputurl>

A URL for a file to write the capture data into.

## Declaration

```swift
var outputURL: URL? { get set }
```

## Discussion

The default value is `nil`. If you set [destination](https://developer.apple.com/documentation/metal/mtlcapturedescriptor/destination) to [MTLCaptureDestination.gpuTraceDocument](https://developer.apple.com/documentation/metal/mtlcapturedestination/gputracedocument), you need to set this property to where you want the file to be written to.

## See also

### Setting capture parameters
- [captureObject](https://developer.apple.com/documentation/metal/mtlcapturedescriptor/captureobject) — The instance whose contents should be captured.
- [destination](https://developer.apple.com/documentation/metal/mtlcapturedescriptor/destination) — The destination for any captured command data.
