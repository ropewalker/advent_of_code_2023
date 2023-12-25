use aoc_runner_derive::{aoc, aoc_generator};
use num::rational::*;
use num::{BigInt, Signed};
use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

type Vec3 = (i64, i64, i64);
type Position3 = Vec3;
type Velocity3 = Vec3;

type Vec2 = (i64, i64);
type Position2 = Vec2;
type Velocity2 = Vec2;

const MIN_AREA_COORDINATE: i64 = 200_000_000_000_000;
const MAX_AREA_COORDINATE: i64 = 400_000_000_000_000;
const AREA_BOTTOM_LEFT: Position2 = (MIN_AREA_COORDINATE, MIN_AREA_COORDINATE);
const AREA_TOP_RIGHT: Position2 = (MAX_AREA_COORDINATE, MAX_AREA_COORDINATE);

#[aoc_generator(day24)]
fn parse_input(trajectories: &str) -> Vec<(Position3, Velocity3)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(px:i64 ", " " "* py:i64 ", " " "* pz:i64
    " @ " " "*
    vx:i64 ", " " "* vy:i64 ", " " "* vz:i64 =>
        ((px, py, pz), (vx, vy, vz))
    ));

    parser.parse(trajectories).unwrap()
}

fn intersect_2d(
    (position1, velocity1): (Position2, Velocity2),
    (position2, velocity2): (Position2, Velocity2),
    area_bottom_left: Position2,
    area_top_right: Position2,
) -> bool {
    let x1 = BigRational::from(BigInt::from(position1.0));
    let y1 = BigRational::from(BigInt::from(position1.1));

    let x2 = BigRational::from(BigInt::from(position2.0));
    let y2 = BigRational::from(BigInt::from(position2.1));

    if (x1 < BigRational::from(BigInt::from(area_bottom_left.0)) && velocity1.0 < 0)
        || (x1 > BigRational::from(BigInt::from(area_top_right.0)) && velocity1.0 > 0)
        || (y1 < BigRational::from(BigInt::from(area_bottom_left.1)) && velocity1.1 < 0)
        || (y1 > BigRational::from(BigInt::from(area_top_right.1)) && velocity1.1 > 0)
        || (x2 < BigRational::from(BigInt::from(area_bottom_left.0)) && velocity2.0 < 0)
        || (x2 > BigRational::from(BigInt::from(area_top_right.0)) && velocity2.0 > 0)
        || (y2 < BigRational::from(BigInt::from(area_bottom_left.1)) && velocity2.1 < 0)
        || (y2 > BigRational::from(BigInt::from(area_top_right.1)) && velocity2.1 > 0)
    {
        return false;
    }

    let a1 = BigRational::from((BigInt::from(velocity1.1), BigInt::from(velocity1.0)));
    let b1 = &y1 - &a1 * &x1;

    let a2 = BigRational::from((BigInt::from(velocity2.1), BigInt::from(velocity2.0)));
    let b2 = &y2 - &a2 * &x2;

    if a1 == a2 {
        return false;
    }

    let x = (&b2 - &b1) / (&a1 - &a2);

    let y = &a1 * &x + &b1;

    x >= BigRational::from(BigInt::from(area_bottom_left.0))
        && x <= BigRational::from(BigInt::from(area_top_right.0))
        && y >= BigRational::from(BigInt::from(area_bottom_left.1))
        && y <= BigRational::from(BigInt::from(area_top_right.1))
        && (&x - &x1).signum() == BigRational::from(BigInt::from(velocity1.0.signum()))
        && (&y - &y1).signum() == BigRational::from(BigInt::from(velocity1.1.signum()))
        && (&x - &x2).signum() == BigRational::from(BigInt::from(velocity2.0.signum()))
        && (&y - &y2).signum() == BigRational::from(BigInt::from(velocity2.1.signum()))
}

fn count_intersections_2d(
    trajectories: &[(Position2, Velocity2)],
    area_bottom_left: Position2,
    area_top_right: Position2,
) -> usize {
    let mut count = 0;

    for i in 0..trajectories.len() {
        for j in i + 1..trajectories.len() {
            if intersect_2d(
                trajectories[i],
                trajectories[j],
                area_bottom_left,
                area_top_right,
            ) {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day24, part1)]
fn part1(trajectories: &[(Position3, Velocity3)]) -> usize {
    count_intersections_2d(
        &trajectories
            .iter()
            .map(|((px, py, _pz), (vx, vy, _vz))| ((*px, *py), (*vx, *vy)))
            .collect::<Vec<_>>(),
        AREA_BOTTOM_LEFT,
        AREA_TOP_RIGHT,
    )
}

#[aoc(day24, part2)]
fn part2(trajectories: &[(Position3, Velocity3)]) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for (position, velocity) in trajectories.iter().skip(5).take(15) {
        let pxn = Int::from_i64(&ctx, position.0);
        let pyn = Int::from_i64(&ctx, position.1);
        let pzn = Int::from_i64(&ctx, position.2);
        let vxn = Int::from_i64(&ctx, velocity.0);
        let vyn = Int::from_i64(&ctx, velocity.1);
        let vzn = Int::from_i64(&ctx, velocity.2);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();
    x + y + z
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    const MIN_TEST_AREA_COORDINATE: i64 = 7;
    const MAX_TEST_AREA_COORDINATE: i64 = 27;
    const TEST_AREA_BOTTOM_LEFT: Position2 = (MIN_TEST_AREA_COORDINATE, MIN_TEST_AREA_COORDINATE);
    const TEST_AREA_TOP_RIGHT: Position2 = (MAX_TEST_AREA_COORDINATE, MAX_TEST_AREA_COORDINATE);

    #[test]
    fn part1_example() {
        assert_eq!(
            count_intersections_2d(
                &parse_input(TEST_INPUT)
                    .iter()
                    .map(|((px, py, _pz), (vx, vy, _vz))| ((*px, *py), (*vx, *vy)))
                    .collect::<Vec<_>>(),
                TEST_AREA_BOTTOM_LEFT,
                TEST_AREA_TOP_RIGHT,
            ),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 47);
    }
}
