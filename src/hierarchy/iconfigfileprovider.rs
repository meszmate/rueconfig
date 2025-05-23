use crate::parsing::ConfigIni;

use super::iconfigfileioadapter::ConfigFileIOAdapter;

pub trait ConfigFileProvider {
    fn file_io_adapter(&self) -> &dyn ConfigFileIOAdapter;
    fn engine_path(&self) -> Option<&str>;
    fn project_path(&self) -> Option<&str>;
    fn is_setup(&self) -> bool;

    fn setup(
        &mut self,
        adapter: Box<dyn ConfigFileIOAdapter>,
        engine_path: Option<String>,
        project_path: Option<String>,
    );

    // fn resolve_config_file_path(&self, reference: &ConfigFileReference) -> Option<String>;

    // fn load_or_create_config(
    //     &self,
    //     reference: &ConfigFileReference,
    // ) -> io::Result<(bool, ConfigIni)>;

    // fn load_or_create_data_driven_platform_config(
    //     &self,
    //     platform_identifier: &str,
    // ) -> io::Result<(bool, ConfigIni)>;

    // fn save_config(&self, reference: &ConfigFileReference, config: &ConfigIni) -> io::Result<()>;
}