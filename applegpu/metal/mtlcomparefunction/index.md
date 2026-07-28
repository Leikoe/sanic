# MTLCompareFunction

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomparefunction>

Options used to specify how a sample compare operation should be performed on a depth texture.

## Declaration

```swift
enum MTLCompareFunction
```

## Overview

Whenever the comparison test passes, the incoming fragment is compared to the stored data at the specified location.

## Topics

### Compare function options
- [MTLCompareFunction.never](https://developer.apple.com/documentation/metal/mtlcomparefunction/never) — A new value never passes the comparison test.
- [MTLCompareFunction.less](https://developer.apple.com/documentation/metal/mtlcomparefunction/less) — A new value passes the comparison test if it is less than the existing value.
- [MTLCompareFunction.equal](https://developer.apple.com/documentation/metal/mtlcomparefunction/equal) — A new value passes the comparison test if it is equal to the existing value.
- [MTLCompareFunction.lessEqual](https://developer.apple.com/documentation/metal/mtlcomparefunction/lessequal) — A new value passes the comparison test if it is less than or equal to the existing value.
- [MTLCompareFunction.greater](https://developer.apple.com/documentation/metal/mtlcomparefunction/greater) — A new value passes the comparison test if it is greater than the existing value.
- [MTLCompareFunction.notEqual](https://developer.apple.com/documentation/metal/mtlcomparefunction/notequal) — A new value passes the comparison test if it is not equal to the existing value.
- [MTLCompareFunction.greaterEqual](https://developer.apple.com/documentation/metal/mtlcomparefunction/greaterequal) — A new value passes the comparison test if it is greater than or equal to the existing value.
- [MTLCompareFunction.always](https://developer.apple.com/documentation/metal/mtlcomparefunction/always) — A new value always passes the comparison test.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlcomparefunction/init(rawvalue:))

## See also

### Declaring the depth comparison mode
- [compareFunction](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/comparefunction) — The sampler comparison function used when performing a sample compare operation on a depth texture.
