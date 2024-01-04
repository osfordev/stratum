use stratum_lib::ExecutionContext;
use stratum_lib::Stratum;

#[no_mangle]
pub extern "C" fn stratum_build() {
    //
    let execution_context = ExecutionContext();
    execution_context.build();
}

#[no_mangle]
pub extern "C" fn stratum_inspect() {
    //
    let execution_context = ExecutionContext();
    execution_context.inspect();
}

#[no_mangle]
pub extern "C" fn stratum_mount() {
    //
    let execution_context = ExecutionContext();
    execution_context.mount();
}

#[no_mangle]
pub extern "C" fn stratum_pull() {
    //
    let execution_context = ExecutionContext();
    execution_context.pull();
}

#[no_mangle]
pub extern "C" fn stratum_umount() {
    //
    let execution_context = ExecutionContext();
    execution_context.umount();
}
