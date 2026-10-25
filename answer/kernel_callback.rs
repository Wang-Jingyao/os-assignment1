use crate::process::Process;
use alloc::sync::Arc;
use ostd::arch::cpu::context::UserContext;
use ostd::user::UserContextApi;
use riscv::register::scause::Exception;
use spin::Once;

static BRK_CALLBACK: Once<usize> = Once::new();
static LOAD_CALLBACK: Once<usize> = Once::new();

pub fn handle_syscall(user_context: &mut UserContext, process: &Arc<Process>) {
    let _ = process;

    const SYS_REG_BRK: usize = 1;
    const SYS_REG_LOAD: usize = 2;

    let callback = match user_context.a7() {
        SYS_REG_BRK => &BRK_CALLBACK,
        SYS_REG_LOAD => &LOAD_CALLBACK,
        _ => {
            user_context.set_a0(1);
            return;
        }
    };

    let address = user_context.a0();
    let mut registered = false;

    callback.call_once(|| {
        registered = true;
        address
    });

    user_context.set_a0(if registered { 0 } else { 1 });
}

pub fn handle_exception(user_context: &mut UserContext, _process: &Arc<Process>) {
    ulet exception = user_context
        .take_exception()
        .expect("Missing user exception information");

    let callback = match exception.code {
        Exception::Breakpoint => &BRK_CALLBACK,
        Exception::LoadPageFault => &LOAD_CALLBACK,
        _ => panic!("Unhandled user exception: {:?}", exception.code),
    };

    let address = callback
        .get()
        .copied()
        .expect("Exception callback has not been registered");

    user_context.set_instruction_pointer(address);
}
