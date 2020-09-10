use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(u32, String);

impl Future for CountDown {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        if self.0 == 0 {
            Poll::Ready("Zero!!!".to_string())
        } else {
            println!("{} : {}", self.1, self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn async_add(left: i32, right:  i32) -> i32 {
    left + right
}

async fn print_result(value: i32) {
    println!("{}", value);
}

async fn something_great_async_function() -> i32 {
    let ans1 = async_add(2, 3).await;
    print_result(ans1).await;
    let ans2 = async_add(3, 4).await;
    print_result(ans2).await;
    let ans3 = async_add(4, 5).await;
    print_result(ans3).await;
    // 何か処理を挟むこともできる。
    let ans = ans1 + ans2 + ans3;
    println!("{}", ans);
    ans
}

// fn something_great_async_function() -> impl Future<Output = i32> {
//     async {
//         let anc = async_add(2, 3).await;
//         // 何か処理を挟むこともできる。
//         println!("{}", ans);
//         ans
//     }
// }

fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_variable = "this is outside".to_string();
    async move {
        println!("{}", outside_variable);
    }
}

fn main() {
    let countdown_future1 = CountDown(10, "countdown_future1".to_string());
    let countdown_future2 = CountDown(20, "countdown_future2".to_string());
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s);
    }
    executor::block_on(something_great_async_function());
}
