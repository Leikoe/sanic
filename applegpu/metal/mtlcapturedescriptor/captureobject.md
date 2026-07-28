# captureObject

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturedescriptor/captureobject>

The instance whose contents should be captured.

## Declaration

```swift
var captureObject: Any? { get set }
```

## Discussion

The default value is `nil`, but you need to set an instance before using this descriptor to start a capture session.

The behavior of the capture session depends on the kind of instance being captured:

- Specify an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance to capture commands in command buffers created on any command queues created by the device instance.

- Specify an [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance to capture commands in command buffers created by a specific command queue.

- Specify an [MTLCaptureScope](https://developer.apple.com/documentation/metal/mtlcapturescope) instance to indirectly define which commands are captured.

## See also

### Setting capture parameters
- [destination](https://developer.apple.com/documentation/metal/mtlcapturedescriptor/destination) — The destination for any captured command data.
- [outputURL](https://developer.apple.com/documentation/metal/mtlcapturedescriptor/outputurl) — A URL for a file to write the capture data into.
