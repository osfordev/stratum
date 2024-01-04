use stratum_lib::ExecutionContext;
use stratum_lib::Stratum;

fn main() {
    let execution_context = ExecutionContext();
    execution_context.mount();
}
