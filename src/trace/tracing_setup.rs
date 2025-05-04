//use opentelemetry::sdk::trace::Tracer;
use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{self, WithExportConfig};
use opentelemetry_sdk::runtime::Tokio;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::{layer::SubscriberExt, Registry}; // 👈 necessário para `.init()`

pub fn init_tracer() -> Result<(), Box<dyn std::error::Error>> {
    // Define o nome do serviço
    let resource = Resource::new(vec![KeyValue::new("service.name", "user-api")]);

    // Configura o exportador HTTP OTLP para o Jaeger Collector
    let exporter = opentelemetry_otlp::new_exporter()
        .http() // 👈 transforma em OtlpHttpExporterBuilder
        .with_endpoint("http://otel-collector:4318/v1/traces");

    // Cria o pipeline do tracer com o exportador e runtime Tokio
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(opentelemetry_sdk::trace::config().with_resource(resource))
        .install_batch(Tokio)?;

    // Layer que conecta OpenTelemetry ao sistema de tracing
    //let otel_layer = OpenTelemetryLayer::new(tracer);
    let otel_layer = OpenTelemetryLayer::new(tracer).with_filter(EnvFilter::new("user_api=trace")); // apenas seus spans

    // Inicializa o tracing com layer de logs + layer OTEL
    Registry::default()
        .with(tracing_subscriber::fmt::layer()) // logs locais
        .with(otel_layer) // spans para Jaeger
        .try_init()?;

    Ok(())
}
