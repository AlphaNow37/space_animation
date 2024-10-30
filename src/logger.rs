use tracing_subscriber::{fmt, layer::SubscriberExt, filter::EnvFilter};
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logger() {
    tracing_subscriber::registry()
        .with(fmt::layer().without_time())
        .with(EnvFilter::new(r#"
            info,
            wgpu_hal=warn,
            wgpu_core=warn,
            naga=warn,
        "#.replace([' ', '\n', '\t'], "")))
        .init()
}
dq