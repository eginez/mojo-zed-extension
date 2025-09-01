zed_extension_api::register_extension!(MojoExtension);

struct MojoExtension;

impl zed_extension_api::Extension for MojoExtension {
    fn new() -> Self {
        Self
    }
}
