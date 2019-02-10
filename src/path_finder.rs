
use std::collections::HashMap;
use std::collections::VecDeque;

use super::map;


pub fn build_search_three(map: &map::Map, goal_point: (i32, i32), start_points: &Vec<(i32, i32)>)
    -> HashMap<(i32, i32), (i32, i32)>
{
    println!("Building search three goal: {:?} starts: {:?}", goal_point, start_points);
    let mut return_data: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    /*
    if start_points.len() == 0 {
        return return_data;
    }
    */

    let mut queue: VecDeque<((i32, i32), (i32, i32))> = VecDeque::new();
    queue.push_back((goal_point, goal_point));

    let mut counter = 0;
    
    loop {
        // Pop stuuf from queue
        let (point, vector) = match queue.pop_front() {
            Some(x) => x,
            _ => {return return_data}
        };

        // Add stuff to queue
        if !return_data.contains_key(&point) {
            return_data.insert(point, vector);
            if map.point_moveable(point) {
                // println!("Popped moveable point {:?} {:?}", point, vector);
                queue.push_back(((point.0 - 1, point.1), point));
                queue.push_back(((point.0 + 1, point.1), point));
                queue.push_back(((point.0, point.1 - 1), point));
                queue.push_back(((point.0, point.1 + 1), point));
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
        if counter > 100000 {
            return return_data;
        }
    }
}


