use std::collections::{HashMap, HashSet};
use std::fs;

const ROW: i64 = 2000000;
const MAX: i64 = 4000000;
//const MAX: i64 = 20;

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i64 {
        (self.x-other.x).abs() + (self.y-other.y).abs()
    }
}

struct Sensor {
    position: Point,
    closest_beacon: Point,
    distance: i64,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the input");

    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let mut non_possible = HashSet::new();
    let mut sensors_grid = Vec::new();

    for sensor_line in input.lines() {
        let sensor = get_sensor(sensor_line);
        if (sensor.position.y - ROW).abs() <= sensor.distance {
            let min = -sensor.distance + sensor.position.x + (sensor.position.y-ROW).abs();
            let max = sensor.position.x + sensor.position.x - min;
            for i in min..=max{
                non_possible.insert((i, ROW));
            }

            sensors_grid.push(sensor);
        }
    }

    for sensor in sensors_grid {
        let beacon_pos = (sensor.closest_beacon.x, sensor.closest_beacon.y);
        if non_possible.contains(&beacon_pos) {
            non_possible.remove(&beacon_pos);
        }
    }

    non_possible.len()
}

fn part_two(input: &str) -> i64 {
    let mut sensors_grid = Vec::new();

    for sensor_line in input.lines() {
        sensors_grid.push(get_sensor(sensor_line));
    }

    let (mut x, mut y) = (0, 0);
    /*for i in 0..=20 {
        for j in 0..=20 {
            let mut possible = true;
            for sensor in &sensors_grid {
                if sensor.position.manhattan_distance(&Point{x: i, y: j}) <= sensor.distance {
                    possible = false;
                }
            }

            if possible {
                (x, y) = (i, j);
            }
        }
    }*/
    for sensor in &sensors_grid {
        let max_x = if sensor.position.x + sensor.distance+1 <= MAX { sensor.position.x + sensor.distance+1 } else { MAX };
        let min_x = if sensor.position.x - sensor.distance+1 >= 0 { sensor.position.x - sensor.distance+1 } else { 0 };

        for i in min_x..=max_x {
            let diff = sensor.distance+1 - (sensor.position.x - i).abs();
            let u_y = sensor.position.y - diff;
            let d_y = sensor.position.y + diff;

            for pos in [Point{x: i, y: u_y}, Point{x: i, y: d_y}] {
                if pos.y >= 0 && pos.y <= MAX {
                    let mut distress_signal = true;
                    for sensor in &sensors_grid {
                        if sensor.position.manhattan_distance(&pos) <=sensor.distance {
                            distress_signal = false;
                            break
                        }
                    }

                    if distress_signal {
                        (x, y) = (pos.x, pos.y);
                        break
                    }
                }
            }
        }
    }

    println!("{x},{y}");
    x*MAX+y
}

fn get_sensor(sensor_line: &str) -> Sensor {
    let (sensor, beacon) = sensor_line.split_once(":").unwrap();
    let (sensor, beacon) = (get_pos(sensor), get_pos(beacon));
    let distance = sensor.manhattan_distance(&beacon);

    Sensor{
        position: sensor,
        closest_beacon: beacon,
        distance,
    }
}

fn get_pos(line: &str) -> Point {
    let pos_start = line.find("x=").unwrap();
    let (x, y) = line[pos_start..].split_once(",").unwrap();

    Point{
        x: x.split_once("=").unwrap().1.parse().unwrap(),
        y: y.split_once("=").unwrap().1.parse().unwrap(),
    }
}
