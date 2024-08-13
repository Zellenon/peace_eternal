pub mod camera_controls;
pub mod info_dumping_systems;
pub mod platformer_control_systems;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimensionality {
    Dim2,
    Dim3,
}
