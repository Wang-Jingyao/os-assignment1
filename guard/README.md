# Guard-like structure

## Question description

Guard is a useful way to monitor resource recycling in Rust. When a user acquires a resource, a corresponding guard is created. When the guard is released, the resource is automatically freed.

In this question, you need to implement a structure called `Slot` that holds several slots and a guard-like structure called `SlotGuard`. When user calling the `Slot::allocate_slot`, he/she will get a `SlotGuard` that bind to the `Slot` and holds the number of slot it owns. The `SlotGuard` has the following properties:

1. Each `SlotGuard` is bound to a `Slot`.
2. A `Slot` can be bound with multiple `SlotGuard`s.
3. `SlotGuard` holds the number of slot it owns.
4. When the `SlotGuard` is dropped, it adds the number of slots it owns to the bound `Slot`.

> Guard is widely used in Rust, and is generally used as a solution when it comes to allocating and releasing resources that Rust does not govern. For example, in `Mutex` when a user acquires a lock, a `MutexGuard` is created to monitor the release of the lock. When the `MutexGuard` is dropped, the lock is automatically released.

## Evaluation

Check your answer with `eval.sh`
