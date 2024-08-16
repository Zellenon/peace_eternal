use bevy::asset::Handle;
use bevy::ecs::system::Resource;
use bevy::prelude::default;
use bevy::prelude::Meshable;
use bevy::{
    asset::Assets,
    ecs::system::ResMut,
    math::primitives::{
        Capsule3d, Cone, ConicalFrustum, Cuboid, Cylinder, Sphere, Tetrahedron, Torus,
    },
    pbr::StandardMaterial,
    render::{
        mesh::Mesh,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::Image,
    },
};

#[derive(Resource, Default)]
pub struct PrimitiveResources {
    pub material: Handle<StandardMaterial>,
    pub cuboid: Handle<Mesh>,
    pub tetra: Handle<Mesh>,
    pub capsule: Handle<Mesh>,
    pub torus: Handle<Mesh>,
    pub cylinder: Handle<Mesh>,
    pub cone: Handle<Mesh>,
    pub cone_frust: Handle<Mesh>,
    pub sphere: Handle<Mesh>,
    pub sphere2: Handle<Mesh>,
}

pub(crate) fn populate_primitive_resources(
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut primitives: ResMut<PrimitiveResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    primitives.material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    primitives.cuboid = meshes.add(Cuboid::default());
    primitives.tetra = meshes.add(Tetrahedron::default());
    primitives.capsule = meshes.add(Capsule3d::default());
    primitives.torus = meshes.add(Torus::default());
    primitives.cylinder = meshes.add(Cylinder::default());
    primitives.cone = meshes.add(Cone::default());
    primitives.cone_frust = meshes.add(ConicalFrustum::default());
    primitives.sphere = meshes.add(Sphere::default().mesh().ico(5).unwrap());
    primitives.sphere2 = meshes.add(Sphere::default().mesh().uv(32, 18));

    // let extrusions = [
    //     meshes.add(Extrusion::new(Rectangle::default(), 1.)),
    //     meshes.add(Extrusion::new(Capsule2d::default(), 1.)),
    //     meshes.add(Extrusion::new(Annulus::default(), 1.)),
    //     meshes.add(Extrusion::new(Circle::default(), 1.)),
    //     meshes.add(Extrusion::new(Ellipse::default(), 1.)),
    //     meshes.add(Extrusion::new(RegularPolygon::default(), 1.)),
    //     meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
    // ];

    // for (i, shape) in shapes.into_iter().enumerate() {
    //     commands.spawn((
    //         PbrBundle {
    //             mesh: shape,
    //             material: debug_material.clone(),
    //             transform: Transform::from_xyz(
    //                 -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
    //                 2.0,
    //                 Z_EXTENT / 2.,
    //             )
    //             .with_rotation(Quat::from_rotation_x(-PI / 4.)),
    //             ..default()
    //         },
    //         Shape,
    //     ));
    // }

    // for (i, shape) in extrusions.into_iter().enumerate() {
    //     commands.spawn((
    //         PbrBundle {
    //             mesh: shape,
    //             material: debug_material.clone(),
    //             transform: Transform::from_xyz(
    //                 -EXTRUSION_X_EXTENT / 2.
    //                     + i as f32 / (num_extrusions - 1) as f32 * EXTRUSION_X_EXTENT,
    //                 2.0,
    //                 -Z_EXTENT / 2.,
    //             )
    //             .with_rotation(Quat::from_rotation_x(-PI / 4.)),
    //             ..default()
    //         },
    //         Shape,
    //     ));
    // }

    // // ground plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
    //     material: materials.add(Color::from(SILVER)),
    //     ..default()
    // });
}

/// Creates a colorful test pattern
pub fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
