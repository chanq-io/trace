use async_std::task;
use trace::trace;

trace::init_depth_var!();
trace::init_performance_logging!();

#[trace(performance_log = "example_async")]
async fn squared(x: i32) -> i32 {
    x * x
}

#[async_trait::async_trait]
trait Log {
    async fn log(&self, message: &str) -> String;
}

#[async_trait::async_trait]
trait Cubed {
    async fn cubed(x: i32) -> i32;
}

struct Logger {
    level: String,
}

struct Math {}

#[trace]
#[async_trait::async_trait]
impl Log for Logger {
    async fn log(&self, message: &str) -> String {
        format!("[{}] {message}", self.level)
    }
}

#[trace(performance_log = "example_async")]
#[async_trait::async_trait]
impl Cubed for Math {
    async fn cubed(x: i32) -> i32 {
        squared(squared(x).await).await
    }
}

#[trace(performance_log = "example_async")]
impl Math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn subtract(x: i32, y: i32) -> i32 {
        x - y
    }

    pub fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }
}

fn main() {
    std::env::set_var("ENABLE_PERFORMANCE_LOGGING", "1");
    task::block_on(async {
        Math::cubed(32).await;
        Math::add(32, 32);
        Math::subtract(32, 32);
        Math::multiply(32, 32);
    });
}

#[cfg(test)]
#[macro_use]
mod trace_test;

#[cfg(test)]
trace_test!(test_async, main());
