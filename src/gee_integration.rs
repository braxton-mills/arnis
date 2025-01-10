use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GEEBuildingData {
    pub height: f64,
    pub confidence: f32,
    pub footprint: Vec<(f64, f64)>,
}

#[derive(Debug)]
pub struct GEEClient {
    cache: HashMap<String, GEEBuildingData>,
}

impl GEEClient {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Fetches building height data for a given bounding box
    /// Returns a map of building footprint IDs to their height data
    pub async fn fetch_building_heights(
        &mut self,
        bbox: (f64, f64, f64, f64),
    ) -> Result<HashMap<String, GEEBuildingData>, Box<dyn Error>> {
        // TODO: Implement actual GEE API call
        // For now, return empty map to allow for graceful fallback
        Ok(HashMap::new())
    }

    /// Gets building height for a specific building footprint
    /// Returns None if no data is available
    pub fn get_building_height(&self, footprint_id: &str) -> Option<f64> {
        self.cache.get(footprint_id).map(|data| data.height)
    }
}

/// Converts OSM coordinates to a unique string ID for caching
pub fn coords_to_footprint_id(coords: &[(f64, f64)]) -> String {
    let mut id = String::new();
    for (lat, lon) in coords {
        id.push_str(&format!("{:.6},{:.6};", lat, lon));
    }
    id
} 