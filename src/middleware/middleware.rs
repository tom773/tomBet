use tower_http::cors::{Any, CorsLayer};

// Broken code
pub MWARE: CorsLayer = CorsLayer::new().allow_headers(Any);

    
