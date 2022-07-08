use std::ops::RangeInclusive;

use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

use crate::{Coord, Line, Line2D, Rect};

use super::{MazeGenerationResult, MazeGenerationStep};

#[derive(Clone, Copy, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

pub fn recursive_division(
    width: usize,
    height: usize,
    mut steps: Vec<MazeGenerationStep>,
) -> MazeGenerationResult {
    let mut walls = vec![];

    let mut perimeter = Rect::new(
        Coord::new(0, 0),
        Coord::new(width as isize - 1, height as isize - 1),
    )
    .get_perimeter();
    walls.append(&mut perimeter);

    steps.push(MazeGenerationStep::new(
        walls.to_owned().into_iter().collect(),
    ));

    // Create the maze's walls by dividing the area recursively until it cannot be divided further
    divide(
        width as isize - 2,
        height as isize - 2,
        Coord::new(1, 1),
        &mut walls,
        &mut steps,
    );

    MazeGenerationResult::new(steps, walls.into_iter().collect())
}

fn divide(
    width: isize,
    height: isize,
    start: Coord,
    walls: &mut Vec<Coord>,
    steps: &mut Vec<MazeGenerationStep>,
) {
    let orientation = get_orientation(width, height);

    // Return when chamber is minimum size
    if orientation == Orientation::Horizontal && height <= 2
        || orientation == Orientation::Vertical && width <= 2
    {
        return;
    }

    let (from, to): (Coord, Coord);
    let end = Coord::new(start.x + width - 1, start.y + height - 1);

    match orientation {
        Orientation::Horizontal => {
            // Horizontal walls on even y-coordinates
            let y = float_even(rand_num(start.y..start.y + height - 1) as f32) as isize;
            from = Coord::new(start.x, y);
            to = Coord::new(end.x, y);
        }
        Orientation::Vertical => {
            // Vertical walls on even x-coordinates
            let x = float_even(rand_num(start.x..start.x + width - 1) as f32) as isize;
            from = Coord::new(x, start.y);
            to = Coord::new(x, end.y);
        }
    }

    let wall = Line2D::new(from, to);
    let mut wall_points = wall.get_points();
    let odd_wall_points = wall_points
        .iter()
        .filter(|coord| match orientation {
            // Passages off horizontal walls on odd x-coordinates
            Orientation::Horizontal => coord.x % 2 != 0,
            // Passages off vertical walls on odd y-coordinates
            Orientation::Vertical => coord.y % 2 != 0,
        })
        .collect::<Vec<&Coord>>();

    // Get random point from the vec of valid passage points
    let passage = odd_wall_points[rand_num(0..odd_wall_points.len())];

    // Get the index of the passage in the wall
    let passage_index = wall_points.iter().position(|coord| coord == passage);

    // Remove the point from the wall to create a passage
    if let Some(passage_index) = passage_index {
        wall_points.remove(passage_index.min(wall_points.len() - 1));
    }

    walls.append(&mut wall_points);

    steps.push(MazeGenerationStep::new(
        walls.to_owned().into_iter().collect(),
    ));

    match orientation {
        // If the wall is horizontal, recurse above and below
        Orientation::Horizontal => {
            // Top section
            divide(width, from.y - start.y, start, walls, steps);
            // Bottom section
            divide(
                width,
                end.y - from.y,
                Coord::new(start.x, from.y + 1),
                walls,
                steps,
            );
        }
        // If the wall is vertical, recurse left and right
        Orientation::Vertical => {
            // Left section
            divide(to.x - start.x, height, start, walls, steps);
            // Right section
            divide(
                end.x - to.x,
                height,
                Coord::new(to.x + 1, start.y),
                walls,
                steps,
            );
        }
    }
}

fn get_orientation(width: isize, height: isize) -> Orientation {
    if width < height {
        Orientation::Horizontal
    } else if height < width {
        Orientation::Vertical
    } else {
        if rand::thread_rng().gen_range::<u8, RangeInclusive<u8>>(0..=1) == 0 {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        }
    }
}

fn rand_num<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    rand::thread_rng().gen_range(range)
}

fn float_even(num: f32) -> f32 {
    (num / 2.0).ceil() * 2.0
}
