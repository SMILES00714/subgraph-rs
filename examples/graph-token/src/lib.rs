use subgraph::log;

#[no_mangle]
pub extern "C" fn call_me() {
    log::log(log::Level::Info, "called me");
}
