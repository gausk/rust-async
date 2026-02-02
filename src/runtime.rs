use std::collections::{LinkedList};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::task::{Poll, Wake, Waker};

pub struct Runtime {
    /// A queue to place all the tasks
    queue: Queue,
    /// A spawner which can spawn tasks on to the queue
    spawner: Spawner,
    /// Count of number of tasks
    tasks: AtomicUsize,
}

impl Runtime {
    pub fn start() {
        std::thread::spawn(|| loop {
            let task = match Runtime::get().queue.lock().unwrap().pop_front() {
                Some(task) => task,
                None => continue,
            };
            if task.will_block() {
                while task.poll().is_pending() {}
            } else if task.poll().is_pending() {
                Arc::new(task).wake()
            }
        });
    }

    pub(crate) fn get() -> &'static Runtime {
        &RUNTIME
    }

    pub fn spawner() -> Spawner {
        Runtime::get().spawner.clone()
    }
}

fn wait() {
    while Runtime::get().tasks.load(Ordering::Relaxed) >  0 {}
}

fn spawn<F>(future: impl Future<Output = ()> + Send + Sync + 'static) {
    Runtime::spawner().spawn(future);
}

fn block_on(future: impl Future<Output = ()> + Send + Sync + 'static) {
    Runtime::spawner().spawn_blocking(future);
}

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Runtime::start();
    let queue = Arc::new(Mutex::new(LinkedList::new()));
    Runtime {
        spawner: Spawner {
            queue: queue.clone()
        },
        queue,
        tasks: AtomicUsize::new(0),
    }
});

struct Spawner {
    queue: Arc<Mutex<LinkedList<Task>>>
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + Send + Sync + 'static) {}

    fn spawn_blocking(&self, future: impl Future<Output = ()> + Send + Sync + 'static) {

    }
}

type Queue = Arc<Mutex<LinkedList<Task>>>;

#[derive(Clone)]
struct Task {
    future: Box<dyn Future<Output=()> + Send + Sync + 'static>,
    blocking: bool,
}

impl Task {
    fn new(future: impl Future<Output=()> + Send + Sync + 'static, blocking: bool) -> Self {
        Self {
            future: Box::new(future),
            blocking
        }
    }

    fn will_block(&self) -> bool {
        self.blocking
    }

    fn waker(self: Arc<Self>) -> Waker {
        self.clone().into()
    }

    fn poll(&self) -> Poll<()> {
        self.future.poll()
    }
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
    }
}
