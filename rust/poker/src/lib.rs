use std::cmp::Ordering;
use poker::PokerHand;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // convert hands str in to vec of PokerHands
    let mut hands: Vec<PokerHand> = hands.iter().map(|&s| PokerHand::new(s)).collect();
    hands.sort();  // sort all hands in ascending order
    hands.reverse(); // and flip to descending
    hands.iter() // return any hands that tie for first place
        .take_while(|&h| h.cmp(hands.first().unwrap()) == Ordering::Equal)
        .map(|h| h.hand_str).collect()
}


mod poker {
    use std::cmp::Ordering;
    use itertools::Itertools;


    ///---------------------------------------------------------------------------------------------
    ///  Suit
    ///---------------------------------------------------------------------------------------------

    #[repr(u8)]
    #[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash, Debug)]
    enum Suit {
        Hearts = b'H',
        Diamonds = b'D',
        Clubs = b'C',
        Spades = b'S',
    }

    impl TryFrom<char> for Suit {
        type Error = String;

        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                'H' => Ok(Self::Hearts),
                'D' => Ok(Self::Diamonds),
                'C' => Ok(Self::Clubs),
                'S' => Ok(Self::Spades),
                _ => Err("Bad Suit string".to_string())
            }
        }
    }

    ///---------------------------------------------------------------------------------------------
    ///  Rank
    ///---------------------------------------------------------------------------------------------

    #[repr(u8)]
    #[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash, Debug)]
    enum Rank {
        One,   // used only in case of Ace-low straights
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl TryFrom<&str> for Rank {
        type Error = String;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let filtered: String = value.chars()
                .take_while(|c| !['H', 'D', 'C', 'S'].contains(c))
                .collect();
            match filtered.as_str() {
                "1" => Ok(Self::One),
                "2" => Ok(Self::Two),
                "3" => Ok(Self::Three),
                "4" => Ok(Self::Four),
                "5" => Ok(Self::Five),
                "6" => Ok(Self::Six),
                "7" => Ok(Self::Seven),
                "8" => Ok(Self::Eight),
                "9" => Ok(Self::Nine),
                "10" => Ok(Self::Ten),
                "J" => Ok(Self::Jack),
                "Q" => Ok(Self::Queen),
                "K" => Ok(Self::King),
                "A" => Ok(Self::Ace),
                _ => Err("Invalid Rank string".to_string()),
            }
        }
    }


    ///---------------------------------------------------------------------------------------------
    ///  Card
    ///---------------------------------------------------------------------------------------------

    #[derive(Copy, Clone, Debug, Eq)]
    struct Card {
        suit: Suit,
        rank: Rank,
    }

    impl PartialEq for Card {
        fn eq(&self, other: &Self) -> bool { self.rank.eq(&other.rank) }
    }

    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.rank.partial_cmp(&other.rank)
        }
    }

    impl Ord for Card {
        fn cmp(&self, other: &Self) -> Ordering { self.rank.cmp(&other.rank) }
    }

    impl TryFrom<&str> for Card {
        type Error = String;

        fn try_from(source: &str) -> Result<Self, Self::Error> {
            Ok(Self {
                rank: Rank::try_from(source)?,
                suit: Suit::try_from(source.chars().last().unwrap())?,
            })
        }
    }


    ///---------------------------------------------------------------------------------------------
    ///  HandKind
    ///---------------------------------------------------------------------------------------------

    #[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
    enum HandKind {
        StraightFlush = 9,
        FourOfAKind = 8,
        FullHouse = 7,
        Flush = 6,
        Straight = 5,
        ThreeOfAKind = 4,
        TwoPair = 3,
        OnePair = 2,
        HighCard = 1,
    }

    impl HandKind {
        fn is_straight(cards: &[Card]) -> bool {
            let mut cards = cards.to_vec();
            if cards.len() == 5 { PokerHand::convert_ace_low_straight(&mut cards) }  // len check to avoid infinite loop
            for (i, card) in cards[..cards.len() - 1].iter().enumerate() {
                let next_card_rank = &cards[i + 1].rank;
                if (*next_card_rank as u8) - (card.rank as u8) != 1 { return false }
            }
            true
        }

        fn is_four_of_a_kind(cards: &[Card]) -> bool {
            cards.iter().filter(|&c| c == cards.iter().min().unwrap()).count() == 4
                || cards.iter().filter(|&c| c == cards.iter().max().unwrap()).count() == 4
        }

        fn is_full_house(cards: &[Card]) -> bool {
            let count_map = cards.iter().map(|c| c.rank).counts();
            count_map.keys().len() == 2 && count_map.values().sorted().cloned().collect_vec() == vec![2, 3]
        }

        fn is_flush(cards: &[Card]) -> bool {
            cards.iter().filter(|c| c.suit == cards.first().unwrap().suit).count() == 5
        }

        fn is_three_of_a_kind(cards: &[Card]) -> bool {
            let count_map = cards.iter().map(|c| c.rank).counts();
            *count_map.values().max().unwrap() == 3
        }

        fn is_two_pair(cards: &[Card]) -> bool {
            let count_map = cards.iter().map(|c| c.rank).counts();
            count_map.values().filter(|&v| *v == 2).count() == 2
        }

        fn is_one_pair(cards: &[Card]) -> bool {
            let count_map = cards.iter().map(|c| c.rank).counts();
            count_map.values().filter(|&v| *v == 2).count() == 1
        }
    }

    impl From<&Vec<Card>> for HandKind {
        fn from(cards: &Vec<Card>) -> Self {
            if Self::is_straight(cards) && Self::is_flush(cards) {
                Self::StraightFlush
            } else if Self::is_four_of_a_kind(cards) {
                Self::FourOfAKind
            } else if Self::is_full_house(cards) {
                Self::FullHouse
            } else if Self::is_flush(cards) {
                Self::Flush
            } else if Self::is_straight(cards) {
                Self::Straight
            } else if Self::is_three_of_a_kind(cards) {
                Self::ThreeOfAKind
            } else if Self::is_two_pair(cards) {
                Self::TwoPair
            } else if Self::is_one_pair(cards) {
                Self::OnePair
            } else {
                Self::HighCard
            }
        }
    }


    ///---------------------------------------------------------------------------------------------
    ///  PokerHand
    ///---------------------------------------------------------------------------------------------

    #[derive(PartialEq, Eq, Debug)]
    pub struct PokerHand<'a> {
        pub hand_str: &'a str,
        cards: Vec<Card>,
        kind: HandKind,
    }

    impl<'a> PokerHand<'a> {
        pub fn new(hand_str: &'a str) -> Self {
            let mut cards = hand_str.split(' ').into_iter()
                .map(|card| Card::try_from(card).unwrap())
                .collect::<Vec<Card>>();
            cards.sort();
            PokerHand {
                hand_str,
                kind: HandKind::from(&cards),
                cards,
            }
        }

        // convert Ace to One if it's an Ace-low straight
        fn convert_ace_low_straight(cards: &mut Vec<Card>) {
            if cards.len() < 5 { return }
            let min_is_2 = matches!(cards.first().unwrap().rank, Rank::Two);
            let max_is_ace = matches!(cards.last().unwrap().rank, Rank::Ace);
            if min_is_2 && max_is_ace && HandKind::is_straight(&cards[0..=3]) {
                cards.insert(0, Card { rank: Rank::One, suit: cards.last().unwrap().suit });
                cards.pop();
            }
        }
    }

    // .sort() doesn't work without this
    impl<'a> PartialOrd for PokerHand<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    impl<'a> Ord for PokerHand<'a> {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.kind.cmp(&other.kind) {
                // if the hands are equal, we must compare them by high card
                Ordering::Equal => {
                    let mut self_cards = self.cards.clone();
                    let mut other_cards = other.cards.clone();
                    // handle ace-low-straight cases
                    if matches!(self.kind, HandKind::Straight | HandKind::StraightFlush) {
                        PokerHand::convert_ace_low_straight(&mut self_cards);
                        PokerHand::convert_ace_low_straight(&mut other_cards);
                    }
                    // Group each rank in each hand into pairs of (rank, count) sorted by count, sub-sorted by rank
                    let self_count_map = self_cards.iter().map(|c| c.rank).counts();
                    let other_count_map = other_cards.iter().map(|c| c.rank).counts();
                    let mut self_rank_count = self_count_map.iter()
                        .sorted_by(|(r1, _), (r2, _)| r2.cmp(r1))
                        .sorted_by(|(_, c1), (_, c2)| c2.cmp(c1));
                    let mut other_rank_count = other_count_map.iter()
                        .sorted_by(|(r1, _), (r2, _)| r2.cmp(r1))
                        .sorted_by(|(_, c1), (_, c2)| c2.cmp(c1));
                    // while there are items left
                    while self_rank_count.len() != 0 { // self_rank_count.clone().any(|(_r, c)| c > &1_usize)  {
                        // get the next rank & size for each (should have highest count of any rank left)
                        let (self_rank, _) = self_rank_count.next().unwrap();
                        let (other_rank, _) = other_rank_count.next().unwrap();
                        // if they are not equal, return the result of the comparison, otherwise continue
                        let ord = self_rank.cmp(other_rank);
                        if !matches!(ord, Ordering::Equal) {
                            return ord
                        }
                    }
                    Ordering::Equal
                },
                // if one hand type is better outright, return the ordering as-is
                unequal_ordering => unequal_ordering
            }
        }
    }
}