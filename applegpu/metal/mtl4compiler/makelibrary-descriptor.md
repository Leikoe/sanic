# makeLibrary(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4compiler/makelibrary(descriptor:)>

Creates a new Metal library synchronously.

## Declaration

```swift
func makeLibrary(descriptor: MTL4LibraryDescriptor) throws -> any MTLLibrary
```

## Parameters

- **descriptor** — A description of the library to create.

## Return Value

A Metal library instance upon success, `nil` otherwise.
