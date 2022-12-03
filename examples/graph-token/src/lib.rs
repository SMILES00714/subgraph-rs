use subgraph::{log, num::BigInt};

#[no_mangle]
pub extern "C" fn call_me() {
    log::log(log::Level::Debug, "called me");

    let answer = BigInt::temp_new(42);
    log::log(
        log::Level::Info,
        &format!("The Answer to Life, The Universe, and Everything is {answer}!"),
    );

    for x in [BigInt::temp_new(42), BigInt::temp_new(-42)] {
        for s in [
            format!("{x:?}"),
            format!("{x:x?}"),
            format!("{x:+X?}"),
            format!("{x:#?}"),
            format!("{x:#x?}"),
            format!("{x:+#X?}"),
            format!("{x}"),
            format!("{x:+}"),
            format!("{x:x}"),
            format!("{x:+X}"),
            format!("{x:#x}"),
            format!("{x:+#X}"),
        ] {
            log::log(log::Level::Info, &s);
        }
    }
}
