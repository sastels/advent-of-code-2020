pub fn solve_a(data: &[String]) -> usize {
    let desired_time: usize = data[0].parse().unwrap();
    let bus_time: (usize, usize, usize) = data[1]
        .split(",")
        .filter(|s| *s != "x")
        .map(|s| s.parse::<usize>().unwrap())
        .map(|id| {
            let bus = (desired_time / id) * id;
            if bus < desired_time {
                return (id, bus + id, bus + id - desired_time);
            } else {
                return (id, bus, bus - desired_time);
            }
        })
        .min_by(|x, y| x.2.cmp(&y.2))
        .unwrap();
    bus_time.0 * bus_time.2
}

pub fn solve_b(_data: &[String]) -> i32 {
    0
}
