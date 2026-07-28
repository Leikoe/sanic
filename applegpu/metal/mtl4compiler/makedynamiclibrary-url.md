# makeDynamicLibrary(url:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4compiler/makedynamiclibrary(url:)>

Creates a new dynamic library from the contents of a file at an URL location synchronously.

## Declaration

```swift
func makeDynamicLibrary(url: URL) throws -> any MTLDynamicLibrary
```

## Parameters

- **url** — An URL referencing a file whose contents this compiler uses to build a dynamic library.

## Return Value

A new dynamic Metal library upon success, `nil` otherwise.
