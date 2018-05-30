trait ContextManager<T> {
    fn enter(&mut self) -> &mut T;
    fn exit(&mut self);
}

macro_rules! with {
    ($mgr:expr; as $mgr_ref:ident $code_block:expr) => (
        let mut manager = $mgr;
        let $mgr_ref = manager.enter();
        $code_block;
        $mgr_ref.exit();
    );
}

struct SimpleManager {}

impl ContextManager<SimpleManager> for SimpleManager {
    fn enter(&mut self) -> &mut SimpleManager {
        println!("Entering!");
        self
    }

    fn exit(&mut self) {
        println!("Exiting!");
    }
}

struct Counter {
    ctr: i32
}

impl Counter {
    fn inc(&mut self) {
        self.ctr += 1;
    }
    fn new() -> Counter {
        Counter { ctr: 0 }
    }
}

impl ContextManager<Counter> for Counter {
    fn enter(&mut self) -> &mut Counter {
        self
    }

    fn exit(&mut self) {
        println!("Counter status: {}", self.ctr)
    }
}

fn main() {
    with!(SimpleManager{}; as mgr {
        println!("Inside the block");
    });

    with!(Counter::new(); as counter {
        counter.inc();
        counter.inc();
    });
}
