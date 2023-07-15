use serde;

/// The description of a location.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GeoFence {
    longitude: f64,
    latitude: f64,
    // In meters.
    radius: f32,
    name: String,
    description: String,
}
