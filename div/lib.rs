#![no_std]

use codec::Ops;

fn div(left: i128, right: i128) -> i128 {
    left / right
}

#[no_mangle]
extern "C" fn handle() {
    let ops: Ops = gstd::msg::load().expect("failed to load action");
    match ops {
        Ops::Div(n1, n2) => {
            gstd::msg::reply(Ops::Int128(div(n1, n2)), 0).expect("failed to reply");
        }
        _ => (),
    }
}
