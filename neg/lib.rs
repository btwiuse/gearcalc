#![no_std]

use codec::Ops;
use gstd::ActorId;
use gstd::ToString;

async fn sub(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "2628ef3dc3e9fc20841b6dbafc7aa0d07ee6a8c280408ecafec4ad31942c1d2d"
        )),
        Ops::Sub(left, right),
        0,
    )
    .unwrap()
    .await
    .unwrap();
    match x {
        Ops::Int128(i) => i,
        _ => 0,
    }
}

async fn neg(left: i128) -> i128 {
    sub(0, left).await
}

#[gstd::async_main]
async fn main() {
    let ops: Ops = gstd::msg::load().expect("failed to load action");
    match ops {
        Ops::Neg(n) => {
            gstd::msg::reply(Ops::Int128(neg(n).await), 0).expect("failed to reply");
        }
        _ => (),
    }
}

gstd::metadata! {
    title: "neg",
    handle:
        input: Ops,
        output: Ops,
}
