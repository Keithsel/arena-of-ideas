use bevy::{
    prelude::{Bezier, CubicGenerator},
    sprite::Mesh2dHandle,
};

use super::*;

#[derive(Serialize, TypeUuid, TypePath, Deserialize, Debug, Component, Resource, Clone)]
#[uuid = "cc360991-638e-4066-af03-f4f8abbbc450"]
#[serde(deny_unknown_fields)]
pub struct Representation {
    pub material: RepresentationMaterial,
    #[serde(default)]
    pub children: Vec<Box<Representation>>,
    #[serde(default)]
    pub mapping: VarMapping,
}

#[derive(Serialize, Deserialize, Debug, Component, Clone, Default)]
pub struct VarMapping(HashMap<VarName, Expression>);

impl VarMapping {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a VarName, &'a Expression)> {
        self.0.iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub enum RepresentationMaterial {
    Shape {
        shape: Shape,
        #[serde(default = "default_one_vec2_e")]
        size: Expression,
        #[serde(default = "default_one_f32_e")]
        thickness: Expression,
        #[serde(default = "default_color_e")]
        color: Expression,
    },
    Text {
        #[serde(default = "default_one_f32_e")]
        size: Expression,
        text: Expression,
        #[serde(default = "default_color_e")]
        color: Expression,
        #[serde(default = "default_font_size")]
        font_size: f32,
    },
    Curve {
        #[serde(default = "default_one_f32_e")]
        thickness: Expression,
        #[serde(default = "default_one_f32_e")]
        curvature: Expression,
        #[serde(default = "default_color_e")]
        color: Expression,
    },
}

fn default_font_size() -> f32 {
    32.0
}
fn default_one_f32_e() -> Expression {
    Expression::Float(1.0)
}
fn default_one_vec2_e() -> Expression {
    Expression::Vec2(1.0, 1.0)
}
fn default_color_e() -> Expression {
    Expression::State(VarName::HouseColor)
}

impl RepresentationMaterial {
    pub fn unpack(&self, entity: Entity, world: &mut World) {
        match self {
            RepresentationMaterial::Shape { shape, .. } => {
                let mut materials = world.resource_mut::<Assets<LineShapeMaterial>>();
                let material = LineShapeMaterial {
                    color: Color::PINK,
                    shape: *shape,
                    ..default()
                };
                let material = materials.add(material);
                let mesh = world
                    .resource_mut::<Assets<Mesh>>()
                    .add(Mesh::new(default()));
                world.entity_mut(entity).insert(MaterialMesh2dBundle {
                    material,
                    mesh: mesh.into(),
                    ..default()
                });
            }
            RepresentationMaterial::Text { font_size, .. } => {
                world.entity_mut(entity).insert(Text2dBundle {
                    text: Text::from_section(
                        "".to_owned(),
                        TextStyle {
                            font_size: *font_size,
                            color: Color::PINK,
                            ..default()
                        },
                    ),
                    ..default()
                });
            }
            RepresentationMaterial::Curve { .. } => {
                let mut materials = world.resource_mut::<Assets<CurveMaterial>>();
                let material = CurveMaterial {
                    color: Color::PINK,
                    ..default()
                };
                let material = materials.add(material);
                let mesh = world
                    .resource_mut::<Assets<Mesh>>()
                    .add(Mesh::new(PrimitiveTopology::TriangleStrip));
                world.entity_mut(entity).insert(MaterialMesh2dBundle {
                    material,
                    mesh: mesh.into(),
                    ..default()
                });
            }
        }
    }

    fn set_visible(entity: Entity, value: bool, world: &mut World) {
        if let Some(mut entity) = world.get_entity_mut(entity) {
            match value {
                true => entity.insert(Visibility::Inherited),
                false => entity.insert(Visibility::Hidden),
            };
        }
    }

    pub fn update(&self, entity: Entity, world: &mut World) {
        let t = get_t(world);
        if let Some(state) = world.get::<VarState>(entity) {
            let visible = state.get_bool_at(VarName::Visible, t).unwrap_or(true);
            let visible = visible && state.birth < t;
            Self::set_visible(entity, visible, world);
            if !visible {
                return;
            }
        }
        let context = Context::from_owner(entity, world);
        match self {
            RepresentationMaterial::Shape {
                shape,
                size,
                color,
                thickness,
            } => {
                let size = size.get_vec2(&context, world).unwrap_or_default();
                let thickness = thickness.get_float(&context, world).unwrap_or_default();
                let color = color.get_color(&context, world).unwrap_or(Color::Rgba {
                    red: 1.0,
                    green: 0.0,
                    blue: 1.0,
                    alpha: 1.0,
                });
                let handle = world
                    .get::<Handle<LineShapeMaterial>>(entity)
                    .unwrap()
                    .clone();
                let mut materials = world
                    .get_resource_mut::<Assets<LineShapeMaterial>>()
                    .unwrap();
                if let Some(mat) = materials.get_mut(&handle) {
                    mat.color = color;
                    mat.thickness = thickness;
                    if mat.size != size {
                        mat.size = size;
                        let mesh = world.entity(entity).get::<Mesh2dHandle>().unwrap().clone();
                        if let Some(mesh) = world
                            .get_resource_mut::<Assets<Mesh>>()
                            .unwrap()
                            .get_mut(&mesh.0)
                        {
                            *mesh = shape.mesh(size);
                        }
                    }
                }
            }
            RepresentationMaterial::Text {
                size,
                text,
                font_size,
                color,
            } => {
                let color = color.get_color(&context, world).unwrap();
                world.get_mut::<Text>(entity).unwrap().sections[0].value =
                    text.get_string(&context, world).unwrap_or_default();
                world.get_mut::<Text>(entity).unwrap().sections[0].style = TextStyle {
                    font_size: *font_size,
                    color,
                    ..default()
                };
                world.get_mut::<Transform>(entity).unwrap().scale =
                    vec3(1.0 / *font_size, 1.0 / *font_size, 1.0)
                        * size.get_float(&context, world).unwrap();
            }
            RepresentationMaterial::Curve {
                thickness,
                curvature,
                color,
            } => {
                let thickness = thickness.get_float(&context, world).unwrap() * 0.1;
                let curvature = curvature.get_float(&context, world).unwrap();
                let color = color.get_color(&context, world).unwrap();

                let delta = context
                    .get_var(VarName::Delta, world)
                    .unwrap()
                    .get_vec2()
                    .unwrap();
                let control_delta = vec2(0.0, curvature);
                let curve =
                    Bezier::new([[Vec2::ZERO, control_delta, delta + control_delta, delta]])
                        .to_curve();
                let mut points: Vec<Vec3> = default();
                let mut uvs: Vec<Vec2> = default();
                const SEGMENTS: usize = 30;
                for t in 0..SEGMENTS {
                    let t = t as f32 / SEGMENTS as f32;
                    let position = curve.position(t).extend(0.0);
                    let velocity = curve.velocity(t);
                    points.push(position);
                    points.push(
                        position + (Vec2::Y.rotate(velocity.normalize()) * thickness).extend(0.0),
                    );
                    uvs.push(vec2(t, -1.0));
                    uvs.push(vec2(t, 1.0));
                }

                let handle = world.get::<Handle<CurveMaterial>>(entity).unwrap().clone();
                let mut materials = world.get_resource_mut::<Assets<CurveMaterial>>().unwrap();
                if let Some(mat) = materials.get_mut(&handle) {
                    mat.color = color;
                    let mesh = world.entity(entity).get::<Mesh2dHandle>().unwrap().clone();
                    if let Some(mesh) = world
                        .get_resource_mut::<Assets<Mesh>>()
                        .unwrap()
                        .get_mut(&mesh.0)
                    {
                        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
                        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                    }
                }
            }
        }
    }
}

impl Representation {
    pub fn unpack(
        mut self,
        entity: Option<Entity>,
        parent: Option<Entity>,
        world: &mut World,
    ) -> Entity {
        let entity = match entity {
            Some(value) => value,
            None => world.spawn_empty().id(),
        };
        self.material.unpack(entity, world);
        let mut entity = world.entity_mut(entity);
        entity.get_mut::<Transform>().unwrap().translation.z += 0.0000001; // children always rendered on top of parents
        if let Some(parent) = parent {
            entity.set_parent(parent);
        }
        let entity = entity.id();
        for (i, child) in self.children.drain(..).enumerate() {
            let entity = child.unpack(None, Some(entity), world);
            world.get_mut::<Transform>(entity).unwrap().translation.z += 0.001 * i as f32;
        }
        world.entity_mut(entity).insert(self);
        entity
    }
    pub fn pack(entity: Entity, world: &World) -> Self {
        let mut rep = world.get::<Representation>(entity).unwrap().clone();
        rep.pack_children(entity, world);
        rep
    }
    fn pack_children(&mut self, entity: Entity, world: &World) {
        if let Some(children) = world.get::<Children>(entity) {
            for child in children.iter() {
                if let Some(mut rep) = world.get::<Representation>(*child).cloned() {
                    rep.pack_children(*child, world);
                    self.children.push(Box::new(rep));
                }
            }
        }
    }

    pub fn despawn_all(world: &mut World) {
        for entity in world
            .query_filtered::<Entity, With<Representation>>()
            .iter(world)
            .collect_vec()
        {
            world.entity_mut(entity).despawn_recursive()
        }
    }
}