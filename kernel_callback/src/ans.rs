use crate::process::Process;
use alloc::sync::Arc;
use ostd::arch::cpu::context::UserContext;
use ostd::user::UserContextApi;

pub fn handle_syscall(user_context: &mut UserContext, process: &Arc<Process>) {}

pub fn handle_exception(user_context: &mut UserContext, _process: &Arc<Process>) {
    user_context.set_instruction_pointer(user_context.instruction_pointer() + 2);
}
