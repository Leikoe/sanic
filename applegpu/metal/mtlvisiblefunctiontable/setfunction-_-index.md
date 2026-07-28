# setFunction(_:index:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable/setfunction(_:index:)>

Sets a table entry to point to a callable function.

## Declaration

```swift
func setFunction(_ function: (any MTLFunctionHandle)?, index: Int)
```

## Parameters

- **function** — A function handle for the function to be called.
- **index** — The index of the table entry to change.

## See also

### Setting a table entry
- [setFunctions(_:range:)](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable/setfunctions(_:range:)) — Sets a range of table entries to point to an array of callable functions.
