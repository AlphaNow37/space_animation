use std::borrow::Cow;

use crate::{materials::materials::Material, world::world::{EntityRef, World}};

use super::entity::Entity;

pub struct ChildBuilder<'w, 'v> {
    view: &'v mut EntityView<'w>,
    material: Material,
    name: Cow<'static, str>,
}
impl<'w, 'v> ChildBuilder<'w, 'v> {
    pub fn with_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name = name.into();
        self
    }
    pub fn with_material(mut self, mat: Material) -> Self {
        self.material = mat;
        self
    }
    pub fn build(self) {//} -> EntityView<'w> {
        self.view.push_child(Entity {
            childs: Vec::new(),
            material: self.material,
            name: self.name,
        });
    }
}

pub struct EntityView<'w> {
    pub world: &'w mut World,
    pub eref: EntityRef,
}
impl<'w> EntityView<'w> {
    pub fn new_child<'v>(&'v mut self) -> ChildBuilder<'w, 'v> {
        ChildBuilder {
            material: Material::None,
            view: self,
            name: "X".into()
        }
    }
    pub fn ent_mut(&mut self) -> &mut Entity {
        self.world.get_mut(self.eref)
    }
    pub fn ent(&self) -> &Entity {
        self.world.get(self.eref)
    }
    pub fn push_child<'a>(&'a mut self, ent: Entity) -> EntityView<'a> {
        let eref = self.world.add(ent);
        self.ent_mut().childs.push(eref);
        EntityView {
            world: self.world,
            eref,
        }
    }
}
