use stratum_lib::StratumContext;
use stratum_lib::Stratum;

fn main() {
    let execution_context = StratumContext();
    execution_context.inspect();
}
