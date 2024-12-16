use cgmath::{Quaternion, Rad, Vector3};

use super::model::Instance;
use crate::{
    core::{
        cache::{CacheKey, HashCache},
        error::AppError,
    },
    ecs::{entity::Entity, traits::Cache},
};

#[derive(Debug)]
pub struct InstanceManager {
    pub instances: HashCache<Vec<Instance>>,
}

impl InstanceManager {
    pub fn new() -> Self {
        InstanceManager {
            instances: HashCache::new(),
        }
    }
    pub fn update_instance_transform(&mut self, entity: &Entity) {
        use cgmath::Rotation3;
        let cache_key = CacheKey::from(entity);

        if let Some(instances) = self.instances.get_mut(&cache_key) {
            for instance in instances {
                let rotation_speed_y = Rad(0.01);
                let rotation_speed_x = Rad(0.01);

                let rotation_axis_y = Vector3::unit_y();
                let rotation_axis_x = Vector3::unit_x();

                let incremental_rotation_y =
                    Quaternion::from_axis_angle(rotation_axis_y, rotation_speed_y);
                let incremental_rotation_x =
                    Quaternion::from_axis_angle(rotation_axis_x, rotation_speed_x);

                instance.transform.rotation =
                    incremental_rotation_x * incremental_rotation_y * instance.transform.rotation;
            }
        }
    }

    pub fn add_instance(&mut self, entity: &Entity, instance: Instance) -> Result<(), AppError> {
        let cache_key = CacheKey::from(entity);
        let instances = self.instances.get_or_create(cache_key, || Ok(Vec::new()))?;
        instances.push(instance);

        Ok(())
    }
}
