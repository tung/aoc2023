const FIVE_OF_A_KIND: i32 = 7;
const FOUR_OF_A_KIND: i32 = 6;
const FULL_HOUSE: i32 = 5;
const THREE_OF_A_KIND: i32 = 4;
const TWO_PAIR: i32 = 3;
const ONE_PAIR: i32 = 2;
const HIGH_CARD: i32 = 1;

fn main() {
    let mut hands = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut tokens = line.split_whitespace();
            (
                String::from(tokens.next().expect("hand")),
                tokens
                    .next()
                    .expect("bid")
                    .parse::<usize>()
                    .expect("bid as usize"),
            )
        })
        .collect::<Vec<(String, usize)>>();
    hands.sort_unstable_by_key(|(hand, _)| {
        let mut hand_chars = hand.chars();
        (
            hand_type_value(hand),
            card_from_char(hand_chars.next().expect("card 1")),
            card_from_char(hand_chars.next().expect("card 2")),
            card_from_char(hand_chars.next().expect("card 3")),
            card_from_char(hand_chars.next().expect("card 4")),
            card_from_char(hand_chars.next().expect("card 5")),
        )
    });
    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum::<usize>();
    println!("{total_winnings}");
}

fn card_from_char(c: char) -> usize {
    if c >= '2' && c <= '9' {
        c as usize - '1' as usize
    } else {
        match c {
            'T' => 9,
            'J' => 0,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => unreachable!(),
        }
    }
}

fn count_matching_cards(hand: &str) -> [u8; 13] {
    let mut matches: [u8; 13] = [0; 13];
    for c in hand.chars() {
        matches[card_from_char(c)] += 1;
    }
    matches
}

fn reassign_jokers(matches: &[u8; 13]) -> [u8; 13] {
    let mut new_matches: [u8; 13] = *matches;
    if matches[0] > 0 && matches[0] < 5 {
        new_matches[0] = 0;
        let mut pos = 0;
        for (i, &m) in matches.iter().enumerate().skip(1) {
            if m > 0 && m >= new_matches[pos] {
                pos = i;
            }
        }
        new_matches[pos] += matches[0];
    }
    new_matches
}

fn count_group_sizes(matches: &[u8; 13]) -> [u8; 5] {
    let mut group_sizes: [u8; 5] = [0; 5];
    for &m in matches {
        if m > 0 {
            group_sizes[m as usize - 1] += 1;
        }
    }
    group_sizes
}

fn hand_type_value(hand: &str) -> i32 {
    match count_group_sizes(&reassign_jokers(&count_matching_cards(hand))) {
        [0, 0, 0, 0, 1] => FIVE_OF_A_KIND,
        [1, 0, 0, 1, 0] => FOUR_OF_A_KIND,
        [0, 1, 1, 0, 0] => FULL_HOUSE,
        [2, 0, 1, 0, 0] => THREE_OF_A_KIND,
        [1, 2, 0, 0, 0] => TWO_PAIR,
        [3, 1, 0, 0, 0] => ONE_PAIR,
        [5, 0, 0, 0, 0] => HIGH_CARD,
        _ => unreachable!(),
    }
}

#[rustfmt::skip]
#[test]
fn test_card_from_char() {
    assert_eq!(card_from_char('J'), 0);
    assert_eq!(card_from_char('2'), 1);
    assert_eq!(card_from_char('3'), 2);
    assert_eq!(card_from_char('4'), 3);
    assert_eq!(card_from_char('5'), 4);
    assert_eq!(card_from_char('6'), 5);
    assert_eq!(card_from_char('7'), 6);
    assert_eq!(card_from_char('8'), 7);
    assert_eq!(card_from_char('9'), 8);
    assert_eq!(card_from_char('T'), 9);
    assert_eq!(card_from_char('Q'), 10);
    assert_eq!(card_from_char('K'), 11);
    assert_eq!(card_from_char('A'), 12);
}

#[test]
#[rustfmt::skip]
fn test_count_matching_cards() {
    assert_eq!([5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("JJJJJ"));
    assert_eq!([0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("22222"));
    assert_eq!([0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("33333"));
    assert_eq!([0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("44444"));
    assert_eq!([0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("55555"));
    assert_eq!([0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("66666"));
    assert_eq!([0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0], count_matching_cards("77777"));
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0], count_matching_cards("88888"));
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0], count_matching_cards("99999"));
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0], count_matching_cards("TTTTT"));
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0], count_matching_cards("QQQQQ"));
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0], count_matching_cards("KKKKK"));
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5], count_matching_cards("AAAAA"));

    assert_eq!([0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("22233"));
    assert_eq!([0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("22332"));
    assert_eq!([0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("23322"));
    assert_eq!([0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("33222"));
    assert_eq!([0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("32322"));
    assert_eq!([0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("23232"));
    assert_eq!([0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("22323"));
    assert_eq!([0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("23223"));
    assert_eq!([0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("32223"));

    assert_eq!([0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("22234"));
    assert_eq!([0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("22342"));
    assert_eq!([0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("23422"));
    assert_eq!([0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("34222"));
    assert_eq!([0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("32422"));
    assert_eq!([0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("23242"));
    assert_eq!([0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("22324"));
    assert_eq!([0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("23224"));
    assert_eq!([0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("32224"));

    assert_eq!([0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0], count_matching_cards("65432"));
    assert_eq!([1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0], count_matching_cards("JT987"));
    assert_eq!([0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1], count_matching_cards("32AKQ"));
}

#[test]
#[rustfmt::skip]
fn test_reassign_jokers() {
    assert_eq!(          [0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!(          [0, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[1, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 1, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[1, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 1, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[1, 1, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[1, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[1, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[1, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!(          [0, 1, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[2, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[2, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!(          [0, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[3, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!(          [0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(          [0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[4, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!(          [5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        reassign_jokers(&[5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
}

#[test]
#[rustfmt::skip]
fn test_count_group_sizes() {
    assert_eq!([0, 0, 0, 0, 1], count_group_sizes(&[0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!([0, 0, 0, 0, 1], count_group_sizes(&[0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!([1, 0, 0, 1, 0], count_group_sizes(&[0, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!([1, 0, 0, 1, 0], count_group_sizes(&[0, 0, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!([0, 1, 1, 0, 0], count_group_sizes(&[0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!([0, 1, 1, 0, 0], count_group_sizes(&[0, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!([2, 0, 1, 0, 0], count_group_sizes(&[0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!([2, 0, 1, 0, 0], count_group_sizes(&[0, 0, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!([1, 2, 0, 0, 0], count_group_sizes(&[0, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!([1, 2, 0, 0, 0], count_group_sizes(&[0, 0, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0]));

    assert_eq!([3, 1, 0, 0, 0], count_group_sizes(&[0, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!([3, 1, 0, 0, 0], count_group_sizes(&[0, 0, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]));
}

#[test]
#[rustfmt::skip]
fn test_hand_type_value() {
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("22222"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("33333"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("44444"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("55555"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("66666"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("77777"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("88888"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("99999"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("TTTTT"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("JJJJJ"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("QQQQQ"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("KKKKK"));
    assert_eq!(FIVE_OF_A_KIND, hand_type_value("AAAAA"));

    assert_eq!(FOUR_OF_A_KIND, hand_type_value("AA2AA"));
    assert_eq!(FULL_HOUSE, hand_type_value("A222A"));
    assert_eq!(THREE_OF_A_KIND, hand_type_value("A2223"));
    assert_eq!(HIGH_CARD, hand_type_value("23456"));

    assert_eq!(ONE_PAIR, hand_type_value("32T3K"));
    assert_eq!(TWO_PAIR, hand_type_value("KK677"));
    assert_eq!(FOUR_OF_A_KIND, hand_type_value("KTJJT"));
    assert_eq!(FOUR_OF_A_KIND, hand_type_value("T55J5"));
    assert_eq!(FOUR_OF_A_KIND, hand_type_value("QQQJA"));
}
