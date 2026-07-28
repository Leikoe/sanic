# Setting up a command structure

*Article*

<https://developer.apple.com/documentation/metal/setting-up-a-command-structure>

Discover how Metal executes commands on a GPU.

## Overview

In Metal, you send commands to the GPU so it can perform work on your behalf. A command performs the drawing, parallel computation, and resource management work your app requires.

The relationship between Metal apps and the GPU on a device is a client/server model where your app is the client and the GPU is the server. You make requests by sending commands to the GPU that you encapsulate in a command buffer and then add to a command queue. After processing the commands, the GPU notifies your app when it’s ready for more work.

![image](https://docs-assets.developer.apple.com/published/33d2c0c5043e08e483395771ed84f86c/setting-up-a-command-structure-1%402x.png)

The order that you place commands in command buffers, then enqueue and commit command buffers, affects the perceived order in which Metal executes your commands.

The following sections explain how to set up a command structure to produce the results you want. Some objects you create once and use throughout your app, and others you create specifically to execute a set of commands.

### Create expensive shared objects during initialization

Create objects that are expensive to allocate during initialization, not in time-critical code paths. Objects that you can share in your code are command queues, pipelines, buffers, and textures. After you initialize these objects, they’re fast to reuse.

#### Make a Command Queue

To make a command queue, call the device’s [makeCommandQueue()](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue()) function.

```swift
commandQueue = device.makeCommandQueue()
```

```objective-c
commandQueue = [device newCommandQueue];
```

Then use the same command queue throughout your app to hold command buffers. The figure below illustrates the command queue that contains command buffers:

![image](https://docs-assets.developer.apple.com/published/e4410f1f785637e3936881b01c9af215/setting-up-a-command-structure-2%402x.png)

#### Make One or More Pipeline Objects

A *pipeline object* tells Metal how to process your commands. The pipeline object encapsulates functions that you write in the Metal shading language. To use a pipeline in your Metal workflow, follow these steps:

1. Write Metal shader functions that process your data.

2. Create a pipeline object that contains your shaders.

3. Set the state of the render or compute pipeline.

4. Make draw or compute calls.

Metal doesn’t perform your draw or compute calls immediately. Instead, you use an encoder object to insert commands that encapsulate those calls into your command buffer. After you commit the command buffer, Metal sends it to the GPU and uses the active pipeline object to process the commands.

The figure below illustrates the active pipeline on the GPU that contains your custom shader code that processes commands:

![image](https://docs-assets.developer.apple.com/published/7bbc31985047f1bea71243ca16f4da5d/setting-up-a-command-structure-3%402x.png)

### Issue commands to the GPU

To execute commands on the GPU, follow this process:

1. Create a command buffer from a command queue.

2. Create a command encoder using the command buffer.

3. Add the commands to the command buffer using the command encoder.

4. Get callbacks when the GPU schedules and executes the commands by setting completion handlers.

5. Commit the command buffer.

If you’re performing animation as part of a rendering loop, do this for each frame of the animation. You also follow this process to execute one-off image processing, or machine learning tasks.

#### Create a Command Buffer

Create a command buffer by calling [makeCommandBuffer()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer()) on the command queue.

```swift
guard let commandBuffer = commandQueue.makeCommandBuffer() else { 
    return 
}
```

```objective-c
id <MTLCommandBuffer> commandBuffer = [commandQueue commandBuffer];
```

For single-threaded apps, create a single command buffer containing the commands. The figure below illustrates the command buffer’s relationship to the commands it contains:

![image](https://docs-assets.developer.apple.com/published/571ca85ee804499bc66432d2146cc911/setting-up-a-command-structure-4%402x.png)

#### Add Commands to the Command Buffer

When you call task-specific functions on an encoder object — like draws or compute operations — the encoder places commands corresponding to those calls in the command buffer. The encoder inserts the commands into the command buffer, including everything the GPU needs to process the task at runtime.

The figure below illustrates a command encoder inserting commands into a command buffer when the app makes a draw call:

![image](https://docs-assets.developer.apple.com/published/58d2dd686fce344abf109ffa1a500cae/setting-up-a-command-structure-5%402x.png)

You encode actual commands with concrete subclasses of [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder), depending on your task. For example, use [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) to issue render commands, and [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) to issue parallel computation commands. For a complete list of subclasses, see [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder).

For a complete rendering example, see [Drawing a triangle with Metal 4](https://developer.apple.com/documentation/metal/drawing-a-triangle-with-metal-4). For a complete parallel processing example, see [Combining blit and compute operations in a single pass](https://developer.apple.com/documentation/metal/combining-blit-and-compute-operations-in-a-single-pass).

#### Commit a Command Buffer

To submit your commands to run on the GPU, commit the command buffer to the GPU.

```swift
commandBuffer.commit()
```

```objective-c
[commandBuffer commit];
```

Committing a command buffer doesn’t run its commands immediately. Instead, Metal schedules the buffer’s commands to run only after you commit prior command buffers that are waiting in the queue. If you don’t explicitly enqueue a command buffer, Metal does that for you when you commit the buffer.

You can’t reuse a buffer after you commit it, but you can receive notifications when Metal schedules and completes the commands, or you can query the buffer’s [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status). To receive callbacks during this process, use the [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)) and [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) methods.

As much as possible, the perceived order in which Metal executes the commands is the same as the way you order them. Although Metal might reorder some of your commands before processing them, this usually only occurs when there’s a performance gain and no other perceivable impact.

## See also

### Submitting work to a GPU with Metal
- [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) — An instance you use to create, submit, and schedule command buffers to a specific GPU device to run the commands within those buffers.
- [MTLCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor) — A configuration that customizes the behavior for a new command queue.
- [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) — A container that stores a sequence of GPU commands that you encode into it.
- [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) — A configuration that customizes the behavior for a new command buffer.
- [MTLCommandBufferError](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct) — The command buffer error codes that indicate why the GPU doesn’t finish executing a command buffer.
- [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder) — An encoder that writes GPU commands into a command buffer.
