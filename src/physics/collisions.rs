
#[derive(Component)]
pub struct Collider{
    pub size_x: f32,
    pub size_y: f32,
    pub collider_type: ColliderType
}

pub enum ColliderType{
    none,
    cube,
    one_sided,
}