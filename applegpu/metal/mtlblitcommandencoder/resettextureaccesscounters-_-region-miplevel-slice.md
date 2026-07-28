# resetTextureAccessCounters(_:region:mipLevel:slice:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resettextureaccesscounters(_:region:miplevel:slice:)>

Encodes a command that resets a sparse texture’s access data for a specific region, mipmap level, and slice.

## Declaration

```swift
func resetTextureAccessCounters(_ texture: any MTLTexture, region: MTLRegion, mipLevel: Int, slice: Int)
```

```swift
optional func resetTextureAccessCounters(_ texture: any MTLTexture, region: MTLRegion, mipLevel: Int, slice: Int)
```

## Parameters

- **texture** — A sparse texture instance.
- **region** — A region within the sparse texture’s `mipLevel`, in sparse tile coordinates.
- **mipLevel** — A mipmap level within the sparse texture.
- **slice** — A slice within the sparse texture.

## See also

### Managing sparse texture access counters
- [getTextureAccessCounters(_:region:mipLevel:slice:resetCounters:countersBuffer:countersBufferOffset:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/gettextureaccesscounters(_:region:miplevel:slice:resetcounters:countersbuffer:countersbufferoffset:)) — Encodes a command that retrieves a sparse texture’s access data for a specific region, mipmap level, and slice.
