#![no_std]

use codec::Ops;
use gstd::ActorId;
use gstd::ToString;

async fn sub(left: i128, right: i128) -> i128 {
    // -left
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "2df243bbc717ccce2a462b82a255aa50187edc9c2ee6e9ca974eef8b842873ec"
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

async fn div(left: i128, right: i128) -> i128 {
    // -left
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "2df243bbc717ccce2a462b82a255aa50187edc9c2ee6e9ca974eef8b842873ec"
        )),
        Ops::Div(left, right),
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

async fn mult(left: i128, right: i128) -> i128 {
    // -left
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "2df243bbc717ccce2a462b82a255aa50187edc9c2ee6e9ca974eef8b842873ec"
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

async fn modulus(left: i128, right: i128) -> i128 {
    sub(0, mult(div(left, right).await, right).await).await
}

#[gstd::async_main]
async fn main() {
    let ops: Ops = gstd::msg::load().expect("failed to load action");
    match ops {
        Ops::Modulus(n1, n2) => {
            gstd::msg::reply(Ops::Int128(modulus(n1, n2).await), 0).expect("failed to reply");
        }
        _ => (),
    }
}

gstd::metadata! {
    title: "modulus",
    handle:
        input: Ops,
        output: Ops,
}
