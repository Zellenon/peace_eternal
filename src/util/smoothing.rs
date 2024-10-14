use bevy::{
    app::{Plugin, PostUpdate, PreUpdate, Update},
    ecs::{
        component::Component,
        system::{Query, Res},
    },
    math::{Quat, Vec3},
    reflect::{DynamicStruct, FromReflect, GetTypeRegistration, Reflect, Struct, TypePath},
    time::Time,
};
use std::marker::PhantomData;

pub trait Smoothable {
    fn smooth(&self, goal: &Self, degree: f32) -> Self;
}

impl Smoothable for Vec3 {
    fn smooth(&self, goal: &Vec3, degree: f32) -> Vec3 {
        let delta = (*goal) - (*self);
        (*self) + delta * degree
    }
}

impl Smoothable for Quat {
    fn smooth(&self, goal: &Self, degree: f32) -> Self {
        self.slerp(*goal, (degree).min(1.))
    }
}

#[derive(Component, Debug, Default, Clone, Reflect)]
pub struct Smoothed<T: Reflect, S: Send + Sync + Reflect + Clone, const U: &'static str> {
    pub speed: f32,
    pub goal: Option<S>,
    pub saved: S,
    #[reflect(ignore)]
    pub _phantomdata: PhantomData<T>,
    #[reflect(ignore)]
    pub _phantomdata2: PhantomData<S>,
}

#[derive(Default)]
pub struct SmoothingPlugin<
    T: Struct + Component,
    S: Reflect + Send + Sync + Smoothable + 'static,
    const H: &'static str,
> {
    _1: PhantomData<T>,
    _2: PhantomData<S>,
    _3: &'static str,
}

impl<
        T: Struct + Component + Reflect + TypePath,
        S: Reflect
            + Send
            + Sync
            + Clone
            + Smoothable
            + Reflect
            + TypePath
            + FromReflect
            + GetTypeRegistration
            + 'static,
        const H: &'static str,
    > Plugin for SmoothingPlugin<T, S, H>
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, goal_to_component::<T, S, H>);
        app.add_systems(Update, smooth_midupdate::<T, S, H>);
        app.add_systems(PostUpdate, saved_to_component::<T, S, H>);
        app.register_type::<Smoothed<T, S, H>>();
    }
}

pub fn smooth_midupdate<
    T: Struct + Component,
    S: Reflect + Send + Sync + Clone + Smoothable + FromReflect + 'static,
    const H: &'static str,
>(
    mut smooths: Query<(&T, &mut Smoothed<T, S, H>)>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    for (component, mut smoothed_component) in smooths.iter_mut() {
        let speed = smoothed_component.speed;
        let goal = S::from_reflect(component.field(H).unwrap()).unwrap();
        smoothed_component.saved = smoothed_component.saved.smooth(&goal, speed * delta);
    }
}

pub fn goal_to_component<
    T: Struct + Component,
    S: Send + Sync + Clone + Reflect + FromReflect + 'static,
    const H: &'static str,
>(
    mut smooths: Query<(&mut T, &mut Smoothed<T, S, H>)>,
) {
    for (mut component, mut smoothed_component) in &mut smooths {
        smoothed_component.saved = S::from_reflect(component.field(H).unwrap()).unwrap();
        // avoid change detection
        if smoothed_component.goal.is_some() {
            let value = smoothed_component.goal.take().unwrap();
            let mut a = DynamicStruct::default();
            a.insert(H, value);
            component.apply(&a);
        }
    }
}

pub(crate) fn saved_to_component<
    T: Struct + Component,
    S: Send + Sync + Clone + Reflect + FromReflect + 'static,
    const H: &'static str,
>(
    mut smooths: Query<(&mut T, &mut Smoothed<T, S, H>)>,
) {
    for (mut component, mut smoothed_component) in &mut smooths {
        smoothed_component.goal = S::from_reflect(component.field(H).unwrap());
        // avoid change detection
        let value = smoothed_component.saved.clone();
        let mut a = DynamicStruct::default();
        a.insert(H, value);
        component.apply(&a);
    }
}
