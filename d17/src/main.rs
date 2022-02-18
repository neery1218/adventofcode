use std::cmp;

fn main() {
    let (min_x, max_x) = (209, 238);
    let (min_y, max_y): (i32, i32) = (-86, -59);

    // bounds for brute force
    let (min_vel_y, max_vel_y) = (min_y, min_y.abs());

    let min_vel_x = (0..max_x).find(|vel| vel * (vel + 1) / 2 >= min_x).unwrap();
    let max_vel_x = max_x;
    println!("{} {} {} {}", min_vel_y, max_vel_y, min_vel_x, max_vel_x);

    let mut valid_vels = Vec::new();
    for vel_x in min_vel_x..=max_vel_x {
        for vel_y in min_vel_y..=max_vel_y {
            let mut cur_vel = (vel_x, vel_y);
            let mut cur_pos = (0, 0);
            while cur_pos.0 <= max_x && cur_pos.1 >= min_y {
                cur_pos.0 += cur_vel.0;
                cur_pos.1 += cur_vel.1;

                cur_vel.0 = cmp::max(cur_vel.0 - 1, 0);
                cur_vel.1 -= 1;

                if cur_pos.0 >= min_x
                    && cur_pos.0 <= max_x
                    && cur_pos.1 >= min_y
                    && cur_pos.1 <= max_y
                {
                    valid_vels.push((vel_x, vel_y));
                    break;
                }
            }
        }
    }
    println!("{}", valid_vels.len());
}
