use std::collections::HashMap;

pub struct Ticket {
    pub id: usize,
}

impl Ticket {
    pub fn new(data: &str) -> Self {
        let id: String = data
            .chars()
            .map(|c| if c == 'F' || c == 'L' { '0' } else { '1' })
            .collect();

        Ticket {
            id: usize::from_str_radix(&id, 2).unwrap(),
        }
    }
}

pub fn solve_a(data: &[String]) -> usize {
    data.iter().map(|line| Ticket::new(line).id).max().unwrap()
}

pub fn solve_b(data: &[String]) -> usize {
    let tickets = data.iter().map(|line| Ticket::new(line));
    let mut id_hash: HashMap<usize, usize> = HashMap::new();
    for ticket in tickets {
        id_hash.insert(ticket.id, 1);
    }
    for i in 1..1022 {
        if !id_hash.contains_key(&i)
            && id_hash.contains_key(&(i - 1))
            && id_hash.contains_key(&(i + 1))
        {
            return i;
        }
    }
    0
}
