# setFunctions(_:range:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable/setfunctions(_:range:)>

Sets a range of table entries to point to an array of callable functions.

## Declaration

```swift
func setFunctions(_ functions: [(any MTLFunctionHandle)?], range: Range<Int>)
```

## Parameters

- **functions** — An array of function handles for the functions to be called.
- **range** — A range of indices to change in the table.

## See also

### Setting a table entry
- [setFunction(_:index:)](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable/setfunction(_:index:)) — Sets a table entry to point to a callable function.
