use egui_extras::RetainedImage;
use QcRender::resources::Resource;
pub trait ImageLoader {
    fn load_texture(res: &Resource) -> RetainedImage;
}

impl ImageLoader for RetainedImage {
    fn load_texture(res: &Resource) -> RetainedImage {
        let bytes = res.file.data.as_ref();
        RetainedImage::from_image_bytes(res.name.clone(), bytes).unwrap()
    }
}
