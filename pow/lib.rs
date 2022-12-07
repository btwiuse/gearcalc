#![no_std]

use async_recursion::async_recursion;
use codec::Ops;
use gstd::ActorId;
use gstd::Box;
use gstd::ToString;

async fn mult(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "ebb3ed0712ef2ba890ca6b8a317aba6be6c3598c127e9b9828c3c42dbb879815"
        )),
        Ops::Mult(left, right),
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

#[async_recursion]
async fn pow(left: i128, right: u32) -> i128 {
    match right {
        0 => 1,
        1 => left,
        p => mult(left, pow(left, p - 1).await).await,
    }
}

#[gstd::async_main]
async fn main() {
    let ops: Ops = gstd::msg::load().expect("failed to load action");
    match ops {
        Ops::Pow(left, right) => {
            gstd::msg::reply(Ops::Int128(pow(left, right).await), 0).expect("failed to reply");
        }
        _ => (),
    }
}

gstd::metadata! {
    title: "pow",
    handle:
        input: Ops,
        output: Ops,
}
