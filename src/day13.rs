use std::cmp::{max, min};

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

// https://titanwolf.org/Network/Articles/Article?AID=5987db8d-4f63-4c48-869a-e32b8dfb5d41#gsc.tab=0
fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn solve_b(data: &[String]) -> usize {
    let buses: Vec<(usize, usize)> = data[1]
        .split(",")
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i, s.parse::<usize>().unwrap()))
        .collect();

    let mut t0 = buses[0].1;
    let mut factor = buses[0].1;
    let mut index = 1;
    loop {
        let bus_offset = buses[index].0;
        let bus_id = buses[index].1;

        let mut t = t0;
        loop {
            if (t + bus_offset) % bus_id == 0 {
                break;
            }
            t += factor;
        }

        t0 = t;
        factor = lcm(factor, bus_id);
        index += 1;
        if index == buses.len() {
            break;
        }
    }
    t0
}
