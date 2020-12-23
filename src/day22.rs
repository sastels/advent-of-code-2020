use crate::utils::join_lines;

pub fn deal(decks: &mut Vec<String>) -> (Vec<usize>, Vec<usize>) {
    // bottom of deck is first value. so to get top you pop()
    let deck2: Vec<usize> = decks
        .pop()
        .unwrap()
        .replace("Player 2: ", "")
        .split(' ')
        .map(|s| s.parse().unwrap())
        .rev()
        .collect();
    let deck1: Vec<usize> = decks
        .pop()
        .unwrap()
        .replace("Player 1: ", "")
        .split(' ')
        .map(|s| s.parse().unwrap())
        .rev()
        .collect();
    (deck1, deck2)
}

pub fn draw_cards(deck1: &mut Vec<usize>, deck2: &mut Vec<usize>) -> Option<(usize, usize)> {
    if deck1.is_empty() || deck2.is_empty() {
        None
    } else {
        Some((deck1.pop().unwrap(), deck2.pop().unwrap()))
    }
}

pub fn calc_score(deck: &[usize]) -> usize {
    deck.iter()
        .enumerate()
        .map(|(n, card)| (n + 1) * card)
        .sum()
}

pub fn wins_hand(deck: &mut Vec<usize>, card1: usize, card2: usize) {
    deck.insert(0, card1);
    deck.insert(0, card2);
}

pub fn solve_a(data: &[String]) -> usize {
    let mut data = join_lines(data);
    let (mut deck1, mut deck2) = deal(&mut data);

    loop {
        if let Some((card1, card2)) = draw_cards(&mut deck1, &mut deck2) {
            if card1 > card2 {
                wins_hand(&mut deck1, card1, card2);
            } else {
                wins_hand(&mut deck2, card2, card1);
            }
        } else {
            break;
        }
    }

    if deck1.is_empty() {
        calc_score(&deck2)
    } else {
        calc_score(&deck1)
    }
}

// return the winning player and change the decks (changing the decks needed for final score)
pub fn recursive_combat(round: usize, deck1: &mut Vec<usize>, deck2: &mut Vec<usize>) -> usize {
    let mut previous_rounds: Vec<String> = vec![];

    loop {
        let round_description = format!("P1 {:?} P2 {:?}", deck1, deck2);
        if previous_rounds.contains(&round_description) {
            return 1;
        } else {
            previous_rounds.push(round_description);
            if let Some((card1, card2)) = draw_cards(deck1, deck2) {
                if deck1.len() >= card1 && deck2.len() >= card2 {
                    let mut rec_deck1: Vec<usize> =
                        deck1.iter().rev().take(card1).rev().map(|n| *n).collect();
                    let mut rec_deck2: Vec<usize> =
                        deck2.iter().rev().take(card2).rev().map(|n| *n).collect();
                    let winner = recursive_combat(round + 1, &mut rec_deck1, &mut rec_deck2);
                    if winner == 1 {
                        wins_hand(deck1, card1, card2);
                    } else {
                        wins_hand(deck2, card2, card1);
                    }
                } else {
                    if card1 > card2 {
                        wins_hand(deck1, card1, card2);
                    } else {
                        wins_hand(deck2, card2, card1);
                    }
                }
            } else {
                break;
            }
        }
    }
    if deck1.is_empty() {
        2
    } else {
        1
    }
}

pub fn solve_b(data: &[String]) -> usize {
    let mut data = join_lines(data);
    let (mut deck1, mut deck2) = deal(&mut data);

    let winner = recursive_combat(1, &mut deck1, &mut deck2);

    println!("Final 1 {:?}", deck1);
    println!("Final 2 {:?}", deck2);
    if winner == 1 {
        calc_score(&deck1)
    } else {
        calc_score(&deck2)
    }
}
