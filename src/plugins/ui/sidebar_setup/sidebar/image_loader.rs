use bevy::prelude::*;
use std::path::Path;

use crate::plugins::shp::shp_reader;

pub fn load_image(
    images: &mut Assets<Image>,
    shp_path: &Path,
    pal_path: &Path,
) -> Option<Handle<Image>> {
    if let Ok(handles) = shp_reader::decode_shp_to_image(images, shp_path, pal_path, false) {
        if let Some(handle) = handles.get(0) {
            return Some(handle.clone());
        }
    }
    None
}
