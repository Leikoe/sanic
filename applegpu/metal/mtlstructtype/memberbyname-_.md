# memberByName(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstructtype/memberbyname(_:)>

Provides a representation of a struct member.

## Declaration

```swift
func memberByName(_ name: String) -> MTLStructMember?
```

## Parameters

- **name** — The name of a member in the struct.

## Return Value

An object that represents the named struct member. If `name` does not match a member name, `nil` is returned.

## See also

### Obtaining information about struct members
- [members](https://developer.apple.com/documentation/metal/mtlstructtype/members) — An array of instances that describe the fields in the struct.
