use crate::utils::join_lines;
use std::cmp;

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
    deck.insert(0, cmp::max(card1, card2));
    deck.insert(0, cmp::min(card1, card2));
}

pub fn solve_a(data: &[String]) -> usize {
    let mut data = join_lines(data);
    let (mut deck1, mut deck2) = deal(&mut data);

    // play the game!
    loop {
        if let Some((card1, card2)) = draw_cards(&mut deck1, &mut deck2) {
            if card1 > card2 {
                wins_hand(&mut deck1, card1, card2);
            } else {
                wins_hand(&mut deck2, card1, card2);
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

pub fn solve_b(data: &[String]) -> usize {
    let mut data = join_lines(data);
    let (mut deck1, mut deck2) = deal(&mut data);
    let mut previous_rounds: Vec<String> = vec![];

    // play the game!
    loop {
        if let Some((card1, card2)) = draw_cards(&mut deck1, &mut deck2) {
            if card1 > card2 {
                wins_hand(&mut deck1, card1, card2);
            } else {
                wins_hand(&mut deck2, card1, card2);
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
