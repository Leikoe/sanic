# failOnBinaryArchiveMiss

*Type Property · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlstitchedlibraryoptions/failonbinaryarchivemiss>

An option that instructs the compiler to return an error when a GPU function for a stitched library isn’t in a binary archive.

## Declaration

```swift
static var failOnBinaryArchiveMiss: MTLStitchedLibraryOptions { get }
```

## Discussion

By default, Metal compiles the functions for a stitched library if they aren’t in a binary archive. When you set this option, Metal returns an error instead of compiling a missing function.
