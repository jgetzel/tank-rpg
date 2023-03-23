pub mod ndc;
pub mod prefabs;
pub mod networking;
pub mod commands;

use bevy::math::Vec2;
use rand;
use rand::distributions::{Distribution};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::Normal;

pub fn filter_points_by_min_distance(points: Vec<Vec2>, min_distance: f32) -> Vec<Vec2> {
    let mut filtered_points = Vec::new();

    for point in points {
        let mut too_close = false;
        for other_point in &filtered_points {
            if point.distance(*other_point) < min_distance {
                too_close = true;
                break;
            }
        }
        if !too_close {
            filtered_points.push(point);
        }
    }

    filtered_points
}

pub fn generate_random_points_in_polygon<T: Into<Vec2> + Copy>(polygon: &[T], count: usize) -> Vec<Vec2> {
    let mut points = Vec::new();

    // Find the bounding box of the polygon
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);

    let mut centroid = Vec2::ZERO;

    for &point in polygon {
        let point: Vec2 = point.into();
        min_x = min_x.min(point.x);
        min_y = min_y.min(point.y);
        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
        centroid += point;
    }

    centroid /= polygon.len() as f32;

    let x_radius = (max_x - min_x) / 2.0;
    let y_radius = (max_y - min_y) / 2.0;

    let x_dist = Normal::new(centroid.x, x_radius / 2.0).unwrap();
    let y_dist = Normal::new(centroid.y, y_radius / 2.0).unwrap();


    // Set up random number generator and distributions
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    while points.len() < count {
        let x = x_dist.sample(&mut rng);
        let y = y_dist.sample(&mut rng);
        let test_point = Vec2::new(x, y);

        if is_point_in_polygon(&test_point, polygon) {
            points.push(test_point);
        }
    }

    points
}

pub fn generate_evenly_spaced_points_on_polygon_edges<T: Into<Vec2> + Copy>(polygon: &[T], spacing: f32) -> Vec<Vec2> {
    let mut points = Vec::new();

    let mut remaining_length = spacing;

    for i in 0..polygon.len() {
        let start = polygon[i];
        let end = polygon[(i + 1) % polygon.len()];
        let edge_length = (end.into() - start.into()).length();
        let edge_direction = (end.into() - start.into()) / edge_length;

        while remaining_length < edge_length {
            let point = start.into() + edge_direction * remaining_length;
            points.push(point);
            remaining_length += spacing;
        }

        remaining_length -= edge_length;
    }

    points
}

fn is_point_in_polygon<T: Into<Vec2> + Copy>(point: &Vec2, polygon: &[T]) -> bool {
    let mut is_inside = false;
    let mut i = 0;
    let mut j = polygon.len() - 1;

    while i < polygon.len() {
        if ((polygon[i].into().y > point.y) != (polygon[j].into().y > point.y))
            && (point.x < (polygon[j].into().x - polygon[i].into().x) * (point.y - polygon[i].into().y)
            / (polygon[j].into().y - polygon[i].into().y)
            + polygon[i].into().x)
        {
            is_inside = !is_inside;
        }
        j = i;
        i += 1;
    }

    is_inside
}