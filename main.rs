use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Doubler {
    n: i32,
}
impl Future for Doubler {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(self.n * 2)
    }
}

fn double(n: i32) -> Doubler {
    Doubler { n }
}

fn main() {
    // TODO: build the future for 7, bind it to _fut, and print `created future`.
    let _fut = double(7);
    println!("created future")
}
