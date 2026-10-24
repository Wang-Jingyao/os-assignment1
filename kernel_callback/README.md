# Kernel callback mechanism

You need to implement a kernel callback mechanism. More precisely you need to implement the following functions:

- A syscall named `SYS_REG_BRK` with ID=1. When the user calls this syscall, it will pass an address to the kernel and you need to store this address in a static variable.
- A syscall named `SYS_REG_LOAD` with ID=2. When the user calls this syscall, it will pass an address to the kernel, and you need to store it in another static variable.
- Callback in `handle_exception`. When the user triggers **Breakpoint / LoadPageFault** exception, you need to set the sepc register to the address passed by `SYS_REG_BRK` / `SYS_REG_LOAD`. Thus making a callback.

Additional requirement: The address can be registered only once. Return 0 if the registration success and 1 if the address have been already registered.

## Evaluation

Check your answer with `eval.sh`
