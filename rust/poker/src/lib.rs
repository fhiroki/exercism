use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Eq, Debug)]
enum HandRank {
    StraightFlush = 1,
    FourCard,
    FullHouse,
    Flush,
    Straight,
    ThreeCard,
    TwoPair,
    OnePair,
    NoPair,
}

#[derive(Eq, Debug)]
struct Rank {
    rank: HandRank,
    nums: Vec<u8>,
    hand_index: usize,
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank == other.rank {
            if self.rank == HandRank::TwoPair {
                other.nums.iter().rev().cmp(self.nums.iter().rev())
            } else {
                other.nums.cmp(&self.nums)
            }
        } else {
            self.rank.partial_cmp(&other.rank).unwrap()
        }
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.nums == other.nums
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut ranks = vec![];

    for (i, hand) in hands.iter().enumerate() {
        let mut nums = vec![];
        let mut marks = vec![];
        for card in hand.split_whitespace() {
            let len = card.len() - 1;
            let n = match &card[..len] {
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => {
                    if hand.contains('2') {
                        1
                    } else {
                        14
                    }
                }
                val => val.parse::<u8>().unwrap(),
            };
            let m = match &card[len..] {
                "S" => 1,
                "H" => 2,
                "D" => 3,
                "C" => 4,
                _ => 0,
            };
            nums.push(n);
            marks.push(m);
        }
        nums.sort();
        let mut diffs = vec![];
        for i in 0..4 {
            let d = nums[i + 1] - nums[i];
            diffs.push(d);
        }

        let n_same_pair = diffs.iter().filter(|&d| *d == 0).count();
        let mut max_same_card = 0;
        for i in 0..5 {
            let count = nums.iter().filter(|&n| *n == nums[i]).count();
            if count > max_same_card {
                max_same_card = count;
            }
        }

        let rank = match n_same_pair {
            1 => HandRank::OnePair,
            2 => {
                if max_same_card == 3 {
                    HandRank::ThreeCard
                } else {
                    HandRank::TwoPair
                }
            }
            3 => {
                if max_same_card == 4 {
                    HandRank::FourCard
                } else {
                    HandRank::FullHouse
                }
            }
            _ => {
                let is_flush = marks.iter().all(|&m| m == marks[0]);
                let is_straight = diffs.iter().all(|&d| d == 1);
                if is_flush && is_straight {
                    HandRank::StraightFlush
                } else if is_flush {
                    HandRank::Flush
                } else if is_straight {
                    HandRank::Straight
                } else {
                    HandRank::NoPair
                }
            }
        };

        ranks.push(Rank {
            rank,
            nums: nums,
            hand_index: i,
        });
    }

    ranks.sort();

    let mut wins = vec![hands[ranks[0].hand_index]];
    for i in 1..ranks.len() {
        if ranks[i] == ranks[i - 1] {
            wins.push(hands[ranks[i].hand_index]);
        }
    }
    wins
}
