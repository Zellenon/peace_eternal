use avian3d::prelude as avian;
use bevy::{
    ecs::system::{EntityCommands, SystemParam},
    prelude::*,
};
use bevy_tnua::math::{AsF32, Float, Vector3};

use crate::levels_setup::LevelObject;

#[derive(SystemParam, Deref, DerefMut)]
pub struct LevelSetupHelper3d<'w, 's> {
    #[deref]
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    asset_server: Res<'w, AssetServer>,
}

impl<'w, 's> LevelSetupHelper3d<'w, 's> {
    pub fn spawn_named(&mut self, name: impl ToString) -> EntityCommands {
        self.commands
            .spawn((LevelObject, Name::new(name.to_string())))
    }

    pub fn spawn_floor(&mut self, color: impl Into<Color>) -> EntityCommands {
        let mesh = self
            .meshes
            .add(Plane3d::default().mesh().size(128.0, 128.0));
        let material = self.materials.add(color.into());
        let mut cmd = self.spawn_named("Floor");
        cmd.insert(PbrBundle {
            mesh,
            material,
            ..Default::default()
        });

        {
            cmd.insert(avian::RigidBody::Static);
            cmd.insert(avian::Collider::half_space(Vector3::Y));
        }

        cmd
    }

    pub fn with_material<'a>(
        &'a mut self,
        material: impl Into<StandardMaterial>,
    ) -> LevelSetupHelper3dWithMaterial<'a, 'w, 's> {
        let material = self.materials.add(material);
        LevelSetupHelper3dWithMaterial {
            parent: self,
            material,
        }
    }

    pub fn with_color<'a>(
        &'a mut self,
        color: impl Into<Color>,
    ) -> LevelSetupHelper3dWithMaterial<'a, 'w, 's> {
        self.with_material(color.into())
    }

    pub fn spawn_scene_cuboid(
        &mut self,
        name: impl ToString,
        path: impl ToString,
        transform: Transform,
        #[allow(unused)] size: Vector3,
    ) -> EntityCommands {
        let scene = self.asset_server.load(path.to_string());
        let mut cmd = self.spawn_named(name);

        cmd.insert(SceneBundle {
            scene,
            transform,
            ..Default::default()
        });

        {
            cmd.insert(avian::RigidBody::Static);
            cmd.insert(avian::Collider::cuboid(size.x, size.y, size.z));
        }

        cmd
    }
}

pub struct LevelSetupHelper3dWithMaterial<'a, 'w, 's> {
    parent: &'a mut LevelSetupHelper3d<'w, 's>,
    material: Handle<StandardMaterial>,
}

impl LevelSetupHelper3dWithMaterial<'_, '_, '_> {
    pub fn spawn_cuboid(
        &mut self,
        name: impl ToString,
        transform: Transform,
        size: Vector3,
    ) -> EntityCommands {
        let mesh = self.parent.meshes.add(Cuboid::from_size(size.f32()));
        let mut cmd = self.parent.spawn_named(name);

        cmd.insert(PbrBundle {
            mesh,
            material: self.material.clone(),
            transform,
            ..Default::default()
        });

        {
            cmd.insert(avian::RigidBody::Static);
            cmd.insert(avian::Collider::cuboid(size.x, size.y, size.z));
        }

        cmd
    }

    pub fn spawn_cylinder(
        &mut self,
        name: impl ToString,
        transform: Transform,
        radius: Float,
        half_height: Float,
    ) -> EntityCommands {
        let mesh = self.parent.meshes.add(Cylinder {
            radius: radius.f32(),
            half_height: half_height.f32(),
        });
        let mut cmd = self.parent.spawn_named(name);

        cmd.insert(PbrBundle {
            mesh,
            material: self.material.clone(),
            transform,
            ..Default::default()
        });

        {
            cmd.insert(avian::RigidBody::Static);
            cmd.insert(avian::Collider::cylinder(radius, 2.0 * half_height));
        }

        cmd
    }
}

pub trait LevelSetupHelper3dEntityCommandsExtension {
    fn make_kinematic(&mut self) -> &mut Self;
    fn make_kinematic_with_angular_velocity(&mut self, angvel: Vector3) -> &mut Self;
}

impl LevelSetupHelper3dEntityCommandsExtension for EntityCommands<'_> {
    fn make_kinematic(&mut self) -> &mut Self {
        self.insert((avian::RigidBody::Kinematic,))
    }

    fn make_kinematic_with_angular_velocity(
        &mut self,
        #[allow(unused)] angvel: Vector3,
    ) -> &mut Self {
        self.insert(((avian::AngularVelocity(angvel), avian::RigidBody::Kinematic),))
    }
}
