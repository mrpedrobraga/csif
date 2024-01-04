# Compact Structure Interchange Format

A bytewise serialization/deserialization format to efficiently store/transmit
structured data on files or over the web.

Unlike formats such as JSON, this format doesn't store the schema of the transferred data,
only the individual data points. 
//!
Types such as [`u8`], [`i32`], [`bool`], are stored directly into the file in the precise
amount of bytes needed: to represent them, and no more. The only thing it wont do is store
values in spaces less than 1 byte long.

Dynamically sized types such as lists, maps, [`Option`] and [`String`]s are stored at the end of the file,
and referred in-line through a [`SizedPointer`].

Structs are saved in an interesting way: 

They save their fields side by side, tightly packed, except array or struct fields (even if they're reasonably sized).
Nested struct fields are saved through a thin pointer (since their size IS properly known). This choice was made so formats
with complex information, if transmitted gradually over a stream, would reveal its fields as if you were doing Breadth-First traversal
on the structure tree. This also allows loading the file partially when accessing up to a level of depth on the tree.

## Size

This format is as size-efficient as possible, while using whole bytes, having dynamically sized data and being BFT.
If it wasn't for the third constraint (being BFT), it would be possible to save space on structures by saving them inline.
Perhaps this could be a future feature?

## Parallelism

This format is easy to parallelize when deserializing, because structures's sizes and field positions are known beforehand.
Serializing can be done in parallel, slightly less efficiently, if each concurrent job share a stack they can allocate from/push to.
