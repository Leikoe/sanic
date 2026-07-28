# MTLParallelRenderCommandEncoder

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder>

An instance that splits up a single render pass so that it can be simultaneously encoded from multiple threads.

## Declaration

```swift
protocol MTLParallelRenderCommandEncoder : MTLCommandEncoder
```

## Overview

Your app does not define classes that implement this protocol. To create an [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) instance, call the [makeParallelRenderCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeparallelrendercommandencoder(descriptor:)) method of the [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance that you want to encode the rendering commands into. Then, call the renderCommandEncoder method on this [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) instance to create one or more [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instances. The subordinate [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instances created encode their commands to the same command buffer and target the same [MTLRenderPassAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor) instance. The [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) instance ensures the attachment load and store actions only occur at the start and end of the entire rendering pass.

You can assign each [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) to its own thread and each can encode commands in parallel. You are responsible for any thread synchronization that is required. After all the subordinate encoders have finished encoding their commands, call [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) to execute the commands. The rendering commands are executed in the order that the subordinate encoders were created.

## Topics

### Creating a render command encoder
- [makeRenderCommandEncoder()](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/makerendercommandencoder()) — Create an object that encodes commands that perform graphics rendering operations and may be assigned to a different thread.

### Setting render pass state
- [setColorStoreAction(_:index:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setcolorstoreaction(_:index:)) — Specifies a known store action to replace the initial [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value specified for a given color attachment.
- [setColorStoreActionOptions(_:index:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setcolorstoreactionoptions(_:index:)) — Specifies known store action options for a given color attachment.
- [setDepthStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setdepthstoreaction(_:)) — Specifies a known store action to replace the initial [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value specified for a given depth attachment.
- [setDepthStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setdepthstoreactionoptions(_:)) — Specifies known store action options for a given depth attachment.
- [setStencilStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setstencilstoreaction(_:)) — Specifies a known store action to replace the initial [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value specified for a given stencil attachment.
- [setStencilStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setstencilstoreactionoptions(_:)) — Specifies known store action options for a given stencil attachment.

## See also

### Encoding a render pass in parallel
- [MTLLoadAction](https://developer.apple.com/documentation/metal/mtlloadaction) — Types of actions performed for an attachment at the start of a rendering pass.
- [MTLStoreAction](https://developer.apple.com/documentation/metal/mtlstoreaction) — Types of actions performed for an attachment at the end of a rendering pass.
- [MTLStoreActionOptions](https://developer.apple.com/documentation/metal/mtlstoreactionoptions) — Options that modify a store action.
