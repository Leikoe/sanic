# makeBinaryArchive(descriptor:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makebinaryarchive(descriptor:)>

Creates a Metal binary archive instance.

## Declaration

```swift
func makeBinaryArchive(descriptor: MTLBinaryArchiveDescriptor) throws -> any MTLBinaryArchive
```

## Parameters

- **descriptor** — An [MTLBinaryArchiveDescriptor](https://developer.apple.com/documentation/metal/mtlbinaryarchivedescriptor) instance.

## See also

### Creating binary shader archives
- [MTLBinaryArchiveDescriptor](https://developer.apple.com/documentation/metal/mtlbinaryarchivedescriptor) — A description of a binary shader archive that you want to create.
- [MTLBinaryArchiveError.Code](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/code) — Error codes when creating binary archives of compiled shader code.
- [MTLBinaryArchiveDomain](https://developer.apple.com/documentation/metal/mtlbinaryarchivedomain) — The domain for Metal binary archive errors.
