use image::{DynamicImage};
use lenna_core::plugins::PluginRegistrar;
use lenna_core::ProcessorConfig;
use lenna_core::{core::processor::ExifProcessor, core::processor::ImageProcessor, Processor};

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.add_plugin(Box::new({{project-cc}}::default()));
}

lenna_core::export_plugin!(register);

#[derive(Clone)]
pub struct {{project-cc}} {
    config: Config,
}

impl {{project-cc}} {
}

impl Default for {{project-cc}} {
    fn default() -> Self {
        {{project-cc}} {
            config: Config::default(),
        }
    }
}

impl ImageProcessor for {{project-cc}} {
    fn process_image(
        &self,
        image: &mut Box<DynamicImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut img = DynamicImage::ImageRgba8(image.to_rgba8());
        *image = Box::new(img);
        Ok(())
    }
}

impl ExifProcessor for {{project-cc}} {}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Config {
}

impl Default for Config {
    fn default() -> Self {
        Config {
        }
    }
}

impl Processor for {{project-cc}} {
    fn name(&self) -> String {
        "{{project-name}}".into()
    }

    fn title(&self) -> String {
        "{{project-cc}}".into()
    }

    fn author(&self) -> String {
        "{{authors}}".into()
    }

    fn description(&self) -> String {
        "{{project-description}}".into()
    }

    fn process(
        &mut self,
        config: ProcessorConfig,
        image: &mut Box<lenna_core::LennaImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.config = serde_json::from_value(config.config).unwrap();
        self.process_exif(&mut image.exif).unwrap();
        self.process_image(&mut image.image).unwrap();
        Ok(())
    }

    fn default_config(&self) -> serde_json::Value {
        serde_json::to_value(Config::default()).unwrap()
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
lenna_core::export_wasm_plugin!({{project-cc}});

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
#[allow(non_camel_case_types)]
type lenna_{{project-sc}}_plugin = {{project-cc}};
#[cfg(feature = "python")]
lenna_core::export_python_plugin!(lenna_{{project-sc}}_plugin);

#[cfg(test)]
mod tests {
    use super::*;
    use lenna_core::ProcessorConfig;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn default() {
        let mut {{project-sc}} = {{project-cc}}::default();
        let mut c = {{project-sc}}.default_config();

        let config = ProcessorConfig {
            id: "{{project-name}}".into(),
            config: c,
        };
        assert_eq!({{project-sc}}.name(), "{{project-name}}");
        let mut img =
            Box::new(lenna_core::io::read::read_from_file("assets/lenna.png".into()).unwrap());
        {{project-sc}}.process(config, &mut img).unwrap();
        img.name = "test".to_string();
        lenna_core::io::write::write_to_file(&img, image::ImageOutputFormat::Jpeg(80)).unwrap();
    }
}
