use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::SystemTime;

/// Example
/// async fn sleep_example() {
///     Sleep::new(20000).await;
/// }
///
/// This get converted to:
/// async fn sleep_example {
///     let mut sleep = Sleep::new(2000);
///     loop {
///         match Pin::new(sleep).as_mut().poll(&mut context) {
///             Poll::Ready(()) => (),
///             Poll::pending => yeild,
///         }
///     }
/// }


pub struct Sleep {
    /// Time when future was created
    start: SystemTime,
    /// sleep time in ms
    duration: u128,
}

impl Sleep {
    // A simple method to create an async sleep
    pub fn new(ms: u128) -> Self {
        Self {
            start: SystemTime::now(),
            duration: ms,
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.start.elapsed().unwrap().as_millis() >= self.duration {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}