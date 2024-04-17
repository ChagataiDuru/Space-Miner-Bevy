use bevy::{prelude::*, window::PrimaryWindow};

//use crate::ui::pause::Pausable;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (linear_movement, rotate_2d, wrapping_movement)
                //.run_if(resource_equals(
                //    Pausable::NotPaused,
                //)),
        );
    }
}

#[derive(Component)]
pub struct LinearMovement {
    pub movement_factor: Vec2,
    pub movement_direction: Quat,
}

fn linear_movement(
    mut objects: Query<(&mut Transform, &LinearMovement)>,
    time: Res<Time>,
) {
    for (
        mut transform,
        LinearMovement {
            movement_factor: preexisting_movement_factor,
            movement_direction,
        },
    ) in &mut objects
    {
        let object_facing_direction =
            *movement_direction * Vec3::Y;
        let translation_delta = *preexisting_movement_factor
            + object_facing_direction.xy()
                * time.delta_seconds();
        transform.translation.x += translation_delta.x;
        transform.translation.y += translation_delta.y;
    }
}

#[derive(Component)]
pub struct Rotate2D {
    pub speed: f32,
    pub axis: Vec3,
}

pub fn rotate_2d(
    mut rotatables: Query<(&Rotate2D, &mut Transform)>,
    time: Res<Time>,
) {
    for (rotate, mut transform) in &mut rotatables {
        transform.rotate(Quat::from_scaled_axis(rotate.axis * rotate.speed * time.delta_seconds()));
    }
}

#[derive(Component)]
pub struct MovementWrapper;

pub fn wrapping_movement(
    mut wrappers: Query<
        &mut Transform,
        With<MovementWrapper>,
    >,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        warn!("No window found for wrapping_movement system");
        return;
    };
    let width = window.resolution.width() / 2.;
    let height = window.resolution.height() / 2.;
    for mut transform in &mut wrappers {
        if transform.translation.x > width {
            transform.translation.x -=
                window.resolution.width();
        } else if transform.translation.x < -width {
            transform.translation.x +=
                window.resolution.width();
        }
        if transform.translation.y > height {
            transform.translation.y -=
                window.resolution.height();
        } else if transform.translation.y < -height {
            transform.translation.y +=
                window.resolution.height();
        }
    }
}