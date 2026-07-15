use tower_http::trace::TraceLayer;

/// Placeholder for a future dedicated request-id / correlation-id
/// middleware. For now it returns a `tower-http` tracing layer so the
/// router has sensible request logging out of the box.
pub fn request_id_layer() -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    TraceLayer::new_for_http()
}
