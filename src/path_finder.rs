use std::cmp::Ordering;

use std::collections::HashMap;
use std::collections::BinaryHeap;

use super::map;


struct HeapData {
    point: (i32, i32),
    goal_point: Option<(i32, i32)>,
    distance_so_far: f32,
    weight: f32,
}
impl PartialOrd for HeapData {
    fn partial_cmp(&self, other: &HeapData) -> Option<Ordering> {
        Some(((other.weight * 100.0) as i32).cmp(&((self.weight * 100.0) as i32)))
    }
}
impl Ord for HeapData {
    fn cmp(&self, other: &HeapData) -> Ordering {
        ((other.weight * 100.0) as i32).cmp(&((self.weight * 100.0) as i32))
    }
}
impl PartialEq for HeapData {
    fn eq(&self, other: &HeapData) -> bool {
        return self.weight == other.weight
    }
}
impl Eq for HeapData {}
impl HeapData {
    pub fn new(
        point: (i32, i32),
        goal_point: Option<(i32, i32)>,
        start_points: &Vec<(i32, i32)>,
        distance_so_far: f32,
    ) -> HeapData {
        let mut max_distance: f32 = 0.0;

        for start_point in start_points {
            let distance: f32 = (((point.0 - start_point.0).pow(2) + (point.1 - start_point.1).pow(2)) as f32).sqrt();
            if distance > max_distance {
                max_distance = distance
            }
        }

        HeapData {
            point: point,
            goal_point: goal_point,
            distance_so_far: distance_so_far,
            weight: max_distance + distance_so_far
        }
    }
}


pub fn build_search_tree(map: &map::Map, goal_point: (i32, i32), start_points: &Vec<(i32, i32)>)
    -> HashMap<(i32, i32), Option<(i32, i32)>>
{
    let mut return_data: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();

    let mut heap: BinaryHeap<HeapData> = BinaryHeap::new();

    heap.push(HeapData::new(goal_point, None, start_points, 0.0));

    let mut counter = 0;
    
    loop {
        // Pop stuuf from queue
        let heap_data: HeapData = match heap.pop() {
            Some(x) => x,
            _ => {return return_data}
        };

        let point = heap_data.point;
        // Add stuff to queue
        if !return_data.contains_key(&point) {
            return_data.insert(point, heap_data.goal_point);
            // Sides
            if map.point_moveable((point.0 - 1, point.1)) {
                heap.push(HeapData::new((point.0 - 1, point.1), Some(point), start_points, heap_data.distance_so_far + 1.0))
            };
            if map.point_moveable((point.0 + 1, point.1)) {
                heap.push(HeapData::new((point.0 + 1, point.1), Some(point), start_points, heap_data.distance_so_far + 1.0))
            };
            if map.point_moveable((point.0, point.1 - 1)) {
                heap.push(HeapData::new((point.0, point.1 - 1), Some(point), start_points, heap_data.distance_so_far + 1.0))
            };
            if map.point_moveable((point.0, point.1 + 1)) {
                heap.push(HeapData::new((point.0, point.1 + 1), Some(point), start_points, heap_data.distance_so_far + 1.0))
            };
            // Corners
            if map.point_moveable((point.0 + 1, point.1)) && 
                map.point_moveable((point.0, point.1 + 1)) && 
                map.point_moveable((point.0 + 1, point.1 + 1))
            {
                heap.push(HeapData::new((point.0 + 1, point.1 + 1), Some(point), start_points, heap_data.distance_so_far + 1.414213));
            }
            if map.point_moveable((point.0 - 1, point.1)) &&
                map.point_moveable((point.0, point.1 + 1)) &&
                map.point_moveable((point.0 - 1, point.1 + 1))
            {
                heap.push(HeapData::new((point.0 - 1, point.1 + 1), Some(point), start_points, heap_data.distance_so_far + 1.414213));
            }
            if map.point_moveable((point.0 + 1, point.1)) &&
                map.point_moveable((point.0, point.1 - 1)) &&
                map.point_moveable((point.0 + 1, point.1 - 1))
            {
                heap.push(HeapData::new((point.0 + 1, point.1 - 1), Some(point), start_points, heap_data.distance_so_far + 1.414213));
            }
            if map.point_moveable((point.0 - 1, point.1)) &&
                map.point_moveable((point.0, point.1 - 1)) &&
                map.point_moveable((point.0 - 1, point.1 - 1))
            {
                heap.push(HeapData::new((point.0 - 1, point.1 - 1), Some(point), start_points, heap_data.distance_so_far + 1.414213));
            }
        }

        // Check if found everything
        let mut all_satisfied = true; 
        for start_point in start_points.iter() { // TODO: This loop is not ok
            if !return_data.contains_key(start_point) {
                all_satisfied = false;
            }
        }
        if all_satisfied {
            return return_data;
        }

        // Dont search indefinitely
        counter += 1;
        if counter > 500000 {
            return return_data;
        }
    }
}


