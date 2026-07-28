# Command encoder factory methods

*API Collection*

<https://developer.apple.com/documentation/metal/command-encoder-factory-methods>

A command encoder defines the actions of a single pass, such as GPU commands that draw, compute, or quickly copy resource data.

## Topics

### Creating render encoders
- [makeRenderCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makerendercommandencoder(descriptor:)) — Creates a render command encoder from a descriptor.

### Creating parallel render encoders
- [makeParallelRenderCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeparallelrendercommandencoder(descriptor:)) — Creates a parallel render command encoder from a descriptor.

### Creating acceleration structure encoders
- [makeAccelerationStructureCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeaccelerationstructurecommandencoder(descriptor:)) — Creates a ray-tracing acceleration structure command encoder from a descriptor.
- [makeAccelerationStructureCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeaccelerationstructurecommandencoder()) — Creates a ray-tracing acceleration structure command encoder that uses default settings.

### Creating compute encoders
- [makeComputeCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(descriptor:)) — Creates a compute command encoder from a descriptor.
- [makeComputeCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder()) — Creates a compute command encoder that uses default settings.
- [makeComputeCommandEncoder(dispatchType:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(dispatchtype:)) — Creates a compute command encoder with a dispatch type.
- [MTLDispatchType](https://developer.apple.com/documentation/metal/mtldispatchtype) — The type of dispatch method to use when calling encoded functions.

### Creating blit encoders
- [makeBlitCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder()) — Creates a block information transfer (blit) encoder.
- [makeBlitCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder(descriptor:)) — Creates a block information transfer (blit) encoder from a descriptor.

### Creating resource state encoders
- [resourceStateCommandEncoder(with:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/resourcestatecommandencoder(with:)) — Creates a resource state command encoder from a descriptor.
- [makeResourceStateCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeresourcestatecommandencoder()) — Creates a resource state command encoder that uses default settings.
