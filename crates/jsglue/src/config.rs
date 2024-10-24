use derive_builder::Builder;
use include_dir::Dir;

use crate::framework::Framework;

#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(setter(into))]
pub struct GlueConfig {
    #[builder(setter(strip_option))]
    pub framework: Option<Framework>,

    #[builder(setter(custom))]
    pub base: Option<String>,

    pub dir: Option<Dir<'static>>,

    #[builder(setter(custom))]
    pub project: Option<String>,

    #[builder(setter(custom))]
    pub cmd: Vec<String>,
}

impl GlueConfig {
    pub fn builder() -> GlueConfigBuilder {
        GlueConfigBuilder::default()
    }
}

impl GlueConfigBuilder {
    pub fn base<T>(&mut self, value: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        let new = self;
        new.base = Some(Some(value.as_ref().to_string()));
        new
    }

    pub fn project<T>(&mut self, value: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        let new = self;
        new.project = Some(Some(value.as_ref().to_string()));
        new
    }

    pub fn cmd<T>(&mut self, value: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        let new = self;
        let mut arr = new.cmd.clone().unwrap_or_default();

        arr.push(value.as_ref().to_string());
        new.cmd = Some(arr);
        new
    }

    pub fn arg<T>(&mut self, value: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        self.cmd(value)
    }
}
