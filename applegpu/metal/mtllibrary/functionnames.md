# functionNames

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllibrary/functionnames>

The names of all public functions in the library.

## Declaration

```swift
var functionNames: [String] { get }
```

## Discussion

Inside a Metal library, functions with the `vertex`, `fragment`, or `kernel` function attributes are entry points into the library. Functions without these attributes are private.
