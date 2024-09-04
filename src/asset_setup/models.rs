use crate::util::{make_model, make_model_bundle};
use bevy::{asset::Handle, ecs::system::Resource, gltf::Gltf, prelude::Reflect, scene::Scene};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_composable::tree::ComponentTree;

#[derive(AssetCollection, Resource, Reflect, Clone, Debug, PartialEq)]
pub struct ModelResources {
    #[asset(path = "models/muzzleflare.glb")]
    pub flare_names: Handle<Gltf>,
    #[asset(path = "models/muzzleflare.glb#Scene0")]
    pub flare_scene: Handle<Scene>,

    #[asset(path = "models/basic_bullet.glb")]
    pub basic_bullet_names: Handle<Gltf>,
    #[asset(path = "models/basic_bullet.glb#Scene0")]
    pub basic_bullet_scene: Handle<Scene>,
    #[asset(path = "models/mag_curved_1.glb")]
    pub mag_curved_1_names: Handle<Gltf>,
    #[asset(path = "models/mag_curved_1.glb#Scene0")]
    pub mag_curved_1_scene: Handle<Scene>,
    #[asset(path = "models/mag_curved_2.glb")]
    pub mag_curved_2_names: Handle<Gltf>,
    #[asset(path = "models/mag_curved_2.glb#Scene0")]
    pub mag_curved_2_scene: Handle<Scene>,
    #[asset(path = "models/mag_straight_1.glb")]
    pub mag_straight_1_names: Handle<Gltf>,
    #[asset(path = "models/mag_straight_1.glb#Scene0")]
    pub mag_straight_1_scene: Handle<Scene>,
    #[asset(path = "models/mag_straight_2.glb")]
    pub mag_straight_2_names: Handle<Gltf>,
    #[asset(path = "models/mag_straight_2.glb#Scene0")]
    pub mag_straight_2_scene: Handle<Scene>,
    #[asset(path = "models/pistol_1.glb")]
    pub pistol_1_names: Handle<Gltf>,
    #[asset(path = "models/pistol_1.glb#Scene0")]
    pub pistol_1_scene: Handle<Scene>,
    #[asset(path = "models/pistol_2.glb")]
    pub pistol_2_names: Handle<Gltf>,
    #[asset(path = "models/pistol_2.glb#Scene0")]
    pub pistol_2_scene: Handle<Scene>,
    #[asset(path = "models/rifle_1.glb")]
    pub rifle_1_names: Handle<Gltf>,
    #[asset(path = "models/rifle_1.glb#Scene0")]
    pub rifle_1_scene: Handle<Scene>,
    #[asset(path = "models/rifle_2.glb")]
    pub rifle_2_names: Handle<Gltf>,
    #[asset(path = "models/rifle_2.glb#Scene0")]
    pub rifle_2_scene: Handle<Scene>,
    #[asset(path = "models/shell_pistol.glb")]
    pub shell_pistol_names: Handle<Gltf>,
    #[asset(path = "models/shell_pistol.glb#Scene0")]
    pub shell_pistol_scene: Handle<Scene>,
    #[asset(path = "models/shell_rifle.glb")]
    pub shell_rifle_names: Handle<Gltf>,
    #[asset(path = "models/shell_rifle.glb#Scene0")]
    pub shell_rifle_scene: Handle<Scene>,
    #[asset(path = "models/shell_shotgun.glb")]
    pub shell_shotgun_names: Handle<Gltf>,
    #[asset(path = "models/shell_shotgun.glb#Scene0")]
    pub shell_shotgun_scene: Handle<Scene>,
    #[asset(path = "models/shotgun_pump.glb")]
    pub shotgun_pump_names: Handle<Gltf>,
    #[asset(path = "models/shotgun_pump.glb#Scene0")]
    pub shotgun_pump_scene: Handle<Scene>,
    #[asset(path = "models/grenade_launcher_1.glb")]
    pub grenade_launcher_1_names: Handle<Gltf>,
    #[asset(path = "models/grenade_launcher_1.glb#Scene0")]
    pub grenade_launcher_1_scene: Handle<Scene>,
}

impl ModelResources {
    pub fn pistol1(&self) -> ComponentTree {
        make_model(&self.pistol_1_names, &self.pistol_1_scene)
    }

    pub fn pistol2(&self) -> ComponentTree {
        make_model(&self.pistol_2_names, &self.pistol_2_scene)
    }

    pub fn rifle1(&self) -> ComponentTree {
        make_model(&self.rifle_1_names, &self.rifle_1_scene)
    }

    pub fn rifle2(&self) -> ComponentTree {
        make_model(&self.rifle_2_names, &self.rifle_2_scene)
    }

    pub fn shotgun_pump(&self) -> ComponentTree {
        make_model(&self.shotgun_pump_names, &self.shotgun_pump_scene)
    }

    pub fn mag_curved_1(&self) -> ComponentTree {
        make_model(&self.mag_curved_1_names, &self.mag_curved_1_scene)
    }

    pub fn mag_curved_2(&self) -> ComponentTree {
        make_model(&self.mag_curved_2_names, &self.mag_curved_2_scene)
    }

    pub fn mag_straight_1(&self) -> ComponentTree {
        make_model(&self.mag_straight_1_names, &self.mag_straight_1_scene)
    }

    pub fn mag_straight_2(&self) -> ComponentTree {
        make_model(&self.mag_straight_2_names, &self.mag_straight_2_scene)
    }

    pub fn shell_pistol(&self) -> ComponentTree {
        make_model(&self.shell_pistol_names, &self.shell_pistol_scene)
    }

    pub fn shell_rifle(&self) -> ComponentTree {
        make_model(&self.shell_rifle_names, &self.shell_rifle_scene)
    }

    pub fn shell_shotgun(&self) -> ComponentTree {
        make_model(&self.shell_shotgun_names, &self.shell_shotgun_scene)
    }

    pub fn grenade_launcher_1(&self) -> ComponentTree {
        make_model(
            &self.grenade_launcher_1_names,
            &self.grenade_launcher_1_scene,
        )
    }

    pub fn basic_bullet(&self) -> ComponentTree {
        make_model(&self.basic_bullet_names, &self.basic_bullet_scene)
    }
}
