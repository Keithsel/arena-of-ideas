use super::*;

pub struct ContextSystem {}

impl System for ContextSystem {
    fn update(&mut self, world: &mut legion::World, resources: &mut Resources) {
        Self::refresh_all(world, resources);
    }
}

impl ContextSystem {
    pub fn new() -> Self {
        Self {}
    }

    /// Merge data from other entity components
    pub fn refresh_entity(
        entity: legion::Entity,
        world: &mut legion::World,
        resources: &Resources,
    ) {
        let entry = world.entry(entity).expect("Unit entity not found");
        let mut context = Context {
            vars: default(),
            ..entry.get_component::<Context>().unwrap().clone()
        };

        if let Some(component) = entry.get_component::<AreaComponent>().ok() {
            component.extend_vars(&mut context.vars, resources);
        }
        if let Some(component) = entry.get_component::<HpComponent>().ok() {
            component.extend_vars(&mut context.vars, resources);
        }
        if let Some(component) = entry.get_component::<DescriptionComponent>().ok() {
            component.extend_vars(&mut context.vars, resources);
        }
        if let Some(component) = entry.get_component::<InputComponent>().ok() {
            component.extend_vars(&mut context.vars, resources);
        }
        if let Some(component) = entry.get_component::<HouseComponent>().ok() {
            component.extend_vars(&mut context.vars, resources);
        }
        if let Some(component) = entry.get_component::<UnitComponent>().ok() {
            component.extend_vars(&mut context.vars, resources);
        }

        world.entry(entity).unwrap().add_component(context);
    }

    pub fn try_get_context(
        entity: legion::Entity,
        world: &legion::World,
    ) -> Result<Context, legion::world::ComponentError> {
        world
            .entry_ref(entity)
            .unwrap()
            .get_component::<Context>()
            .and_then(|context| {
                Ok(match context.parent {
                    Some(parent) => context
                        .clone()
                        .merge_mut(&Self::get_context(parent, world), false),
                    None => context.clone(),
                })
            })
    }

    pub fn get_context(entity: legion::Entity, world: &legion::World) -> Context {
        Self::try_get_context(entity, world).expect(&format!(
            "Failed to get Context component from entity#{:?}",
            entity
        ))
    }

    pub fn refresh_all(world: &mut legion::World, resources: &Resources) {
        <&EntityComponent>::query()
            .filter(!component::<WorldComponent>() & component::<Context>())
            .iter(world)
            .map(|entity| entity.entity)
            .collect_vec()
            .into_iter()
            .for_each(|entity| Self::refresh_entity(entity, world, resources));
    }
}