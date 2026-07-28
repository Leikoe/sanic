# subscript(_:)

*Instance Subscript · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptorarray/subscript(_:)>

Returns the descriptor object for the specified sample buffer attachment.

## Declaration

```swift
subscript(attachmentIndex: Int) -> MTLComputePassSampleBufferAttachmentDescriptor! { get set }
```

## Parameters

- **attachmentIndex** — An index for the sample buffer attachment to fetch.

## Return Value

The requested  [MTLComputePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor) object.
