use bevy::app::App;
use bevy::prelude::*;
use bevy::reflect::{List};
use crate::AppState;
use crate::asset_loader::components::SpriteEnum;
use crate::simulation::server_sim::despawn_all_entities;
use crate::utils::{filter_points_by_min_distance, generate_evenly_spaced_points_on_polygon_edges, generate_random_points_in_polygon};
use crate::utils::prefabs::{default_background, default_camera, spawn_point, tree_leaves, tree_trunk};

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OnInitEvent>()
            .add_system(init_default.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_all_entities.in_schedule(OnExit(AppState::InGame)));
    }
}

pub fn init_default(
    mut commands: Commands,
    mut init_writer: EventWriter<OnInitEvent>,
) {
    commands.spawn(default_camera());

    fn convert_point(point: [f32; 2]) -> [f32; 2] {
        [(point[0] - 875.) * 20., (point[1] - 565.) * 20.]
    }

    let spawn_points = [
        [846., 537.],
        [840., 569.],
        [851., 593.],
        [880., 601.],
        [901., 589.],
        [907., 569.],
        [907., 539.],
        [876., 523.],
    ];

    spawn_points.into_iter().map(convert_point).for_each(|point| {
        commands.spawn(spawn_point(point.into()));
    });

    let tree_bounds = [
        (
            vec![
                [833., 506.],
                [886., 499.],
                [896., 481.],
                [910., 465.],
                [924., 448.],
                [896., 431.],
                [862., 425.],
                [830., 431.],
                [799., 447.],
            ],
            200
        ),
        (
            vec![
                [797., 483.],
                [828., 519.],
                [824., 529.],
                [821., 551.],
                [830., 576.],
                [770., 559.],
                [764., 522.],
                [773., 499.],
            ],
            200
        ),
        (
            vec![
                [797., 583.],
                [830., 591.],
                [835., 601.],
                [845., 610.],
                [862., 613.],
                [862., 638.],
                [836., 638.],
                [810., 628.],
                [801., 598.],
            ],
            200
        ),
        (
            vec![
                [879., 613.],
                [897., 613.],
                [916., 606.],
                [924., 595.],
                [934., 595.],
                [955., 615.],
                [946., 624.],
                [934., 638.],
                [902., 643.],
                [879., 640.],
            ],
            200
        ),
        (
            vec![
                [900., 506.],
                [914., 483.],
                [929., 468.],
                [940., 459.],
                [953., 456.],
                [978., 475.],
                [989., 496.],
                [991., 510.],
                [982., 526.],
                [973., 529.],
                [955., 538.],
                [958., 559.],
                [980., 568.],
                [999., 565.],
                [1004., 547.],
                [991., 530.],
                [1001., 516.],
                [1032., 532.],
                [1039., 564.],
                [1032., 586.],
                [1010., 593.],
                [982., 576.],
                [972., 589.],
                [964., 601.],
                [941., 583.],
                [927., 578.],
                [929., 568.],
                [929., 541.],
                [917., 522.],
            ],
            250
        ),
    ];

    tree_bounds.into_iter().flat_map(|(points, count)| {
        let points = points.into_iter().map(convert_point).collect::<Vec<[f32; 2]>>();
        let points = generate_random_points_in_polygon(points.as_slice(), count);
        filter_points_by_min_distance(points, 100.)
    }).chain(generate_evenly_spaced_points_on_polygon_edges(
        &[
            [886., 296.],
            [1029., 315.],
            [1088., 374.],
            [1120., 510.],
            [1045., 626.],
            [964., 694.],
            [914., 749.],
            [783., 734.],
            [698., 678.],
            [647., 598.],
            [651., 499.],
            [657., 365.],
            [758., 327.],
        ], 10.).into_iter().map(|p| convert_point(p.to_array()).into()))
        .for_each(|p| {
            commands.spawn(tree_trunk())
                .insert(Transform::from_xyz(p.x, p.y, 0.));
            // .with_children(|p| {
            //     p.spawn(tree_leaves());
            // });
        });

    init_writer.send(OnInitEvent);
}

pub struct OnInitEvent;