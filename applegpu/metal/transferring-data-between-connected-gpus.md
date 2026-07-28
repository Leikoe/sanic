# Transferring data between connected GPUs

*Article*

<https://developer.apple.com/documentation/metal/transferring-data-between-connected-gpus>

Use high-speed connections between GPUs to transfer data quickly.

## Overview

In macOS 10.15 and later, some Mac systems directly connect GPUs to each other, allowing you to quickly transfer data between them. These connections are not only faster, but they also avoid using the memory bus between the CPU and GPUs, leaving it available for other tasks. If your app uses multiple GPUs, test to see if they’re connected, and when they are, use the transfer mechanism described here.

When GPUs are connected to each other, they’re said to be in the same *peer group*. You determine whether a GPU is in a peer group by reading the device instance’s [peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid) property. A nonzero value indicates that the GPU is in a peer group.

```swift
func isMemberOfAnyPeerGroup(_ device: MTLDevice ) -> Bool
{
    return device.peerGroupID != 0
}
```

```objective-c
bool isMemberOfAnyPeerGroup(id<MTLDevice> device)
{
    return (device.peerGroupID != 0);
}
```

GPUs in the same peer group share the same peer group ID.

```swift
func areMembersOfSamePeerGroup(_ deviceA:MTLDevice,_ deviceB: MTLDevice) -> Bool
{
    return isMemberOfAnyPeerGroup(deviceA) &&
           deviceA.peerGroupID == deviceB.peerGroupID
}
```

```objective-c
bool areMembersOfSamePeerGroup(id<MTLDevice> deviceA, id<MTLDevice> deviceB)
{
    return (isMemberOfAnyPeerGroup(deviceA) &&
            deviceA.peerGroupID == deviceB.peerGroupID);
}
```

You can get the list of all devices associated with a peer group by filtering on this ID.

```swift
func devicesInPeerGroup(_ peerGroupID: UInt64) -> [MTLDevice]
{
    if peerGroupID == 0
    {
        return []
    }
    let allDevices = MTLCopyAllDevices()
    return allDevices.filter({$0.peerGroupID == peerGroupID})
}
```

```objective-c
NSArray<id<MTLDevice>>* devicesInPeerGroup(uint64_t peerGroupID)
{
    if (peerGroupID == 0)
    {
        return @[];
    }
    return [MTLCopyAllDevices() filteredArrayUsingPredicate: [NSPredicate predicateWithFormat:@"SELF.peerGroupID == %uul", peerGroupID]];
}
```

### Copy resources to the GPU that needs to access them

In Metal, resources are created by device instances, and are always associated with the device instance that created them. Peer groups don’t change that association. If a resource is associated with a device instance, and you want to access it on another device instance, you need to copy the data to a resource associated with the second device instance.

To copy data between members of a peer group, make a *remote view* on the second GPU that’s connected to the resource you want to copy. A remote view is a resource instance that contains no storage of its own; it references the storage on the original GPU. You can only use remote views to copy data; using them in other Metal commands results in an error.

### Create a remote view of a resource

To create a remote view of a resource, the device instance you make the resource view on needs to share the same [peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid) as the device instance that created the resource. In addition, the resource needs to use the [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) storage mode or be backed by an [IOSurface](https://developer.apple.com/documentation/IOSurface/IOSurface).

To create a buffer view, call the [makeRemoteBufferView(_:)](https://developer.apple.com/documentation/metal/mtlbuffer/makeremotebufferview(_:)) method:

```swift
let remoteBufferView = sourceBuffer.makeRemoteBufferView(otherDevice)
```

```objective-c
id<MTLBuffer> remoteBufferView = [sourceBuffer newRemoteBufferViewForDevice:otherDevice];
```

Similarly, to create a texture view, call the [makeRemoteTextureView(_:)](https://developer.apple.com/documentation/metal/mtltexture/makeremotetextureview(_:)) method.

```swift
let remoteTextureView = sourceTexture.makeRemoteTextureView(otherDevice)
```

```objective-c
id<MTLTexture> remoteTextureView = [sourceTexture newRemoteTextureViewForDevice:otherDevice];
```

### Copy data between connected GPUs

Create an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) and encode a copy command. The source for this copy command is the remote view instance:

```swift
if let blitEncoder = commandBuffer.makeBlitCommandEncoder()
{
    blitEncoder.copy(from: remoteBufferView, sourceOffset: 0,
                       to: destinationBuffer, destinationOffset: 0,
                     size: remoteBufferView.allocatedSize)
        
    blitEncoder.copy(from: remoteTextureView,
                       to: destinationTexture)
        
    blitEncoder.endEncoding()
}
```

```objective-c
id<MTLBlitCommandEncoder> blitEncoder = [commandBuffer blitCommandEncoder];
[blitEncoder copyFromBuffer:remoteBufferView sourceOffset:0
                   toBuffer:destinationBuffer destinationOffset:0
                       size:remoteBufferView.allocatedSize];
[blitEncoder copyFromTexture:remoteTextureView
                   toTexture:destinationTexture];
[blitEncoder endEncoding];
```

As shown in the following illustration, there are three resource instances: the original resource that contains the data, a remote view that references the data, and a resource that receives the data.

![image](https://docs-assets.developer.apple.com/published/9bd44e5a93a5da65ff277553c3c99ad1/transferring-data-between-connected-gpus-1%402x.png)

### Synchronize access to resources

Blit commands used in peer-to-peer transfers follow all of the usual synchronization rules on the GPU they’re performed on. However, they don’t automatically synchronize with any commands running on the source GPU. If you encode commands that modify the source resource, ensure that those commands are complete before executing the blit command to transfer the data to the other GPU. This is the same as what you do when transferring resources between GPUs through system memory.

To synchronize commands between different device instances, use shared events. See [Synchronizing events across multiple devices or processes](https://developer.apple.com/documentation/metal/synchronizing-events-across-multiple-devices-or-processes) and [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events).

## See also

### Resource management
- [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes) — Set a storage mode that defines the memory location and access permissions of a resource.
- [Choosing a resource storage mode for Apple GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-apple-gpus) — Select an appropriate storage mode for your textures and buffers on Apple GPUs.
- [Choosing a resource storage mode for Intel and AMD GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-intel-and-amd-gpus) — Select an appropriate storage mode for your textures and buffers on AMD and Intel GPUs.
- [Copying data to a private resource](https://developer.apple.com/documentation/metal/copying-data-to-a-private-resource) — Use a blit command encoder to copy buffer or texture data to a private resource.
- [Synchronizing a managed resource in macOS](https://developer.apple.com/documentation/metal/synchronizing-a-managed-resource-in-macos) — Manually synchronize memory for a Metal resource in apps.
- [Reducing the memory footprint of Metal apps](https://developer.apple.com/documentation/metal/reducing-the-memory-footprint-of-metal-apps) — Learn best practices for using memory efficiently in iOS and tvOS.
