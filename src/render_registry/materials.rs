use crate::utils::array_key;

array_key!(
    pub enum MaterialType {
        None,
        Uniform,
        Sponge,
    }
);
impl Default for MaterialType {
    fn default() -> Self {
        Self::None
    }
}
impl MaterialType {
    pub fn entry_point(self) -> &'static str {
        match self {
            Self::None => "fs_none",
            Self::Uniform => "fs_uniform",
            Self::Sponge => "fs_sponge",
        }
    }
}

#[derive(Default)]
pub struct MaterialRef {
    pub index: usize,
    pub mty: MaterialType,
}
