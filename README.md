# Typed IDs

IDs bound to an owner type.

For example: `Id<User>` and `Id<Group>` are distinct types, so you can't accidentally passed an ID of 1 type to a struct/function that takes an ID for another type.
