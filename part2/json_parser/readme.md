# Simple JSON parser

It does not follow the [specification] of [RFC 8259](https://datatracker.ietf.org/doc/html/rfc8259) exactly.

- Root must be an object/array
- Anything following the root object/array is ignored
- Commas are ignored in general, so extra commas are allowed
