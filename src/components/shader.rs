use super::*;

/// Component to link to a shader program with specific parameters
#[derive(Debug, Clone, Deserialize)]
pub struct Shader {
    pub path: PathBuf, // full path
    #[serde(default)]
    pub parameters: ShaderParameters,
    pub layer: ShaderLayer,
    #[serde(default)]
    pub order: i32,
}

impl Shader {
    pub fn set_uniform(mut self, key: &str, value: ShaderUniform) -> Shader {
        self.parameters.uniforms.insert(String::from(key), value);
        self
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum ShaderLayer {
    Background,
    Unit,
    Vfx,
    UI,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShaderParameters {
    #[serde(default = "vertices_default")]
    pub vertices: usize,
    #[serde(default = "instances_default")]
    pub instances: usize,
    pub uniforms: ShaderUniforms,
}

fn vertices_default() -> usize {
    3
}

fn instances_default() -> usize {
    1
}

impl Default for ShaderParameters {
    fn default() -> Self {
        Self {
            vertices: 3,
            instances: 1,
            uniforms: default(),
        }
    }
}

impl ugli::Uniforms for ShaderParameters {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: ugli::UniformVisitor,
    {
        for (name, value) in &self.uniforms.0 {
            visitor.visit(name, value);
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ShaderUniforms(HashMap<String, ShaderUniform>);

impl ShaderUniforms {
    pub fn merge(&self, other: &ShaderUniforms) -> Self {
        let mut result: ShaderUniforms = self.clone();
        other
            .0
            .iter()
            .for_each(|(key, value)| result.insert(key.clone(), value.clone()));
        result
    }

    pub fn mix(a: &Self, b: &Self, t: f32) -> Self {
        let mut result: ShaderUniforms = default();
        for (key, value) in a.0.iter() {
            let a = value;
            let b = b.get(key).expect(&format!("Parameter {} absent in B", key));
            match (a, b) {
                (ShaderUniform::Int(a), ShaderUniform::Int(b)) => {
                    result.insert(key.clone(), ShaderUniform::Int(a + (b - a) * t as i32));
                }
                (ShaderUniform::Float(a), ShaderUniform::Float(b)) => {
                    result.insert(key.clone(), ShaderUniform::Float(a + (b - a) * t));
                }
                (ShaderUniform::Vec2(a), ShaderUniform::Vec2(b)) => {
                    result.insert(key.clone(), ShaderUniform::Vec2(*a + (*b - *a) * t));
                }
                (ShaderUniform::Vec3(a), ShaderUniform::Vec3(b)) => {
                    result.insert(key.clone(), ShaderUniform::Vec3(*a + (*b - *a) * t));
                }
                (ShaderUniform::Vec4(a), ShaderUniform::Vec4(b)) => {
                    result.insert(key.clone(), ShaderUniform::Vec4(*a + (*b - *a) * t));
                }
                (ShaderUniform::Color(a), ShaderUniform::Color(b)) => {
                    result.insert(
                        key.clone(),
                        ShaderUniform::Color(Rgba::new(
                            a.r + (b.r - a.r) * t,
                            a.g + (b.g - a.g) * t,
                            a.b + (b.b - a.b) * t,
                            a.a + (b.a - a.a) * t,
                        )),
                    );
                }
                _ => panic!(
                    "Failed to mix: not matching parameter types: {:?} {:?}",
                    a, b
                ),
            }
        }

        result
    }
}

impl ShaderUniforms {
    pub fn insert(&mut self, key: String, value: ShaderUniform) {
        self.0.insert(key, value);
    }
    pub fn get(&self, key: &String) -> Option<&ShaderUniform> {
        self.0.get(key)
    }
    pub fn find_string(&self) -> Option<(String, String)> {
        for (key, value) in self.0.iter() {
            match value {
                ShaderUniform::String(value) => return Some((key.to_string(), value.to_string())),
                _ => {}
            }
        }
        None
    }
}

impl From<HashMap<&str, ShaderUniform>> for ShaderUniforms {
    fn from(value: HashMap<&str, ShaderUniform>) -> Self {
        Self(HashMap::from_iter(
            value
                .into_iter()
                .map(|(key, value)| (key.to_string(), value)),
        ))
    }
}

impl From<HashMap<String, ShaderUniform>> for ShaderUniforms {
    fn from(value: HashMap<String, ShaderUniform>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ShaderUniform {
    Int(i32),
    Float(f32),
    Vec2(vec2<f32>),
    Vec3(vec3<f32>),
    Vec4(vec4<f32>),
    Color(Rgba<f32>),
    String(String),
}

impl ugli::Uniform for ShaderUniform {
    fn apply(&self, gl: &ugli::raw::Context, info: &ugli::UniformInfo) {
        match self {
            Self::Int(value) => value.apply(gl, info),
            Self::Float(value) => value.apply(gl, info),
            Self::Vec2(value) => value.apply(gl, info),
            Self::Vec3(value) => value.apply(gl, info),
            Self::Vec4(value) => value.apply(gl, info),
            Self::Color(value) => value.apply(gl, info),
            Self::String(_) => {}
        }
    }
}