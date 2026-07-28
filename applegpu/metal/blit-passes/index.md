# Blit passes

*API Collection*

<https://developer.apple.com/documentation/metal/blit-passes>

Encode a block information transfer pass to adjust and copy data to and from GPU resources, such as buffers and textures.

## Overview

Your app can use a block information transfer (blit) pass to manage data within, and copy data between, buffers, textures, and other Metal resources. Start by creating a blit command encoder by calling an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance’s [makeBlitCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder()) method. Then use the [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) instance’s methods to encode individual commands of your blit pass.

You also have the option to customize a blit pass’s runtime behavior, such as sampling GPU hardware data. To enable these options, configure an [MTLBlitPassDescriptor](https://developer.apple.com/documentation/metal/mtlblitpassdescriptor) instance and pass it to the command buffer’s [makeBlitCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder(descriptor:)) method. For more information about sampling GPU hardware data in a blit pass, see the articles in [GPU counters and counter sample buffers](https://developer.apple.com/documentation/metal/gpu-counters-and-counter-sample-buffers), including:

- [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers)

- [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format)

## Topics

### Encoding a blit pass
- [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) — Encodes commands that copy and modify resources for a single blit pass.
- [MTLBlitOption](https://developer.apple.com/documentation/metal/mtlblitoption) — The options that enable behavior for some blit operations.

### Configuring a blit command encoder
- [MTLBlitPassDescriptor](https://developer.apple.com/documentation/metal/mtlblitpassdescriptor) — A configuration you create to customize a blit command encoder, which affects the runtime behavior of the blit pass you encode with it.
- [MTLBlitPassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor) — A configuration that instructs the GPU where to store counter data from the beginning and end of a blit pass.
- [MTLBlitPassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptorarray) — A container that stores an array of sample buffer attachments for a blit pass.

## See also

### Command encoders
- [Render passes](https://developer.apple.com/documentation/metal/render-passes) — Encode a render pass to draw graphics into an image.
- [Compute passes](https://developer.apple.com/documentation/metal/compute-passes) — Encode a compute pass that runs computations in parallel on a thread grid, processing and manipulating Metal resource data on multiple cores of a GPU.
- [Machine learning passes](https://developer.apple.com/documentation/metal/machine-learning-passes) — Add machine learning model inference to your Metal app’s GPU workflow.
- [Indirect command encoding](https://developer.apple.com/documentation/metal/indirect-command-encoding) — Store draw commands in Metal buffers and run them at a later time on the GPU, either once or repeatedly.
- [Ray tracing with acceleration structures](https://developer.apple.com/documentation/metal/ray-tracing-with-acceleration-structures) — Build a representation of your scene’s geometry using triangles and bounding volumes to quickly trace rays through the scene.
