use std::borrow::Cow;

use crate::materials::materials::Material;

use super::world::EntityRef;

#[derive(Clone, Default)]
pub struct Entity {
    pub name: Cow<'static, str>,
    pub childs: Vec<EntityRef>,
    pub material: Material,
}
impl Entity {

}
