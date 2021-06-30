use chrono::Utc;

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Bid {
    pub auction_id: String,
    pub bidder_id: String,
    pub id: i32,
    pub amount: i32,
    pub made_at: chrono::DateTime<chrono::Utc>,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Bid {
    // note on why not implementing comp methods via PartialOrd
    // - avoiding potential user confusion with equality and identity

    pub fn new(auction_id: String, bidder_id: String, id: i32, amount: i32) -> Self {
        Bid {
            id,
            auction_id,
            bidder_id,
            amount,
            made_at: Utc::now(),
            removed_at: None,
        }
    }

    pub fn is_active(&self) -> bool {
        self.removed_at.is_none()
    }

    pub fn remove(&mut self) {
        self.removed_at = Some(Utc::now());
    }

    pub fn is_lower_bid_than(&self, other: &Self) -> bool {
        let both_bids_active = self.is_active() && other.is_active();
        let neither_bid_active = !(self.is_active() || other.is_active());
        let is_lower_amount = self.amount < other.amount;
        let is_equal_amount = self.amount == other.amount;
        let is_later_bid = other.made_at < self.made_at;

        if both_bids_active || neither_bid_active {
            is_lower_amount || (is_equal_amount && is_later_bid)
        } else {
            !self.is_active()
        }
    }

    pub fn is_equivalent_bid_to(&self, other: &Self) -> bool {
        let both_bids_active = self.is_active() && other.is_active();
        let neither_bid_active = !(self.is_active() || other.is_active());
        let is_equal_amount = self.amount == other.amount;
        let is_simultaneous_bid = other.made_at == self.made_at;

        if both_bids_active || neither_bid_active {
            is_equal_amount && is_simultaneous_bid
        } else {
            false
        }
    }

    pub fn is_higher_bid_than(&self, other: &Self) -> bool {
        !self.is_lower_bid_than(other) && !self.is_equivalent_bid_to(other)
    }
}


#[cfg(test)]
mod methods {
    use crate::models::v1::bid::Bid;
    use chrono::{Utc, DateTime, Duration, NaiveDateTime};

    #[test]
    fn new_works() {
        let auction_id: String = String::from("auction_id");
        let bidder_id: String = String::from("bidder_id");
        let id: i32 = 0;
        let amount: i32 = 10000;
        let bid = Bid::new(auction_id.clone(), bidder_id.clone(), id, amount);

        assert_eq!(auction_id, bid.auction_id);
        assert_eq!(bidder_id, bid.bidder_id);
        assert_eq!(id, bid.id);
        assert_eq!(amount, bid.amount);
        assert!(bid.made_at < Utc::now());
        assert!(bid.removed_at.is_none());
    }

    #[test]
    fn is_active_works() {
        let auction_id: String = String::from("auction_id");
        let bidder_id: String = String::from("bidder_id");
        let id: i32 = 0;
        let amount: i32 = 10000;
        let bid = Bid::new(auction_id.clone(), bidder_id.clone(), id, amount);

        assert!(bid.is_active());
    }

    #[test]
    fn remove_works() {
        let auction_id: String = String::from("auction_id");
        let bidder_id: String = String::from("bidder_id");
        let id: i32 = 0;
        let amount: i32 = 10000;
        let mut bid = Bid::new(auction_id.clone(), bidder_id.clone(), id, amount);

        assert!(bid.is_active());

        bid.remove();
        assert!(bid.removed_at.is_some());
    }

    struct TestData {}

    impl TestData {
        fn lower_amount() -> i32 {
            0
        }

        fn higher_amount() -> i32 {
            TestData::lower_amount() + 1
        }

        fn earlier_made_at() -> DateTime<Utc> {
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1000, 100), Utc)
        }

        fn later_made_at() -> DateTime<Utc> {
            TestData::earlier_made_at() + Duration::days(1)
        }

        fn removed_at() -> Option<DateTime<Utc>> {
            Some(Utc::now())
        }

        fn active_lower_earlier_bid() -> Bid {
            Bid {
                auction_id: String::from("auction_id"),
                bidder_id: String::from("0"),
                id: 0,
                amount: TestData::lower_amount(),
                made_at: TestData::earlier_made_at(),
                removed_at: None,
            }
        }

        fn active_lower_later_bid() -> Bid {
            Bid {
                auction_id: String::from("auction_id"),
                bidder_id: String::from("1"),
                id: 1,
                amount: TestData::lower_amount(),
                made_at: TestData::later_made_at(),
                removed_at: None,
            }
        }

        fn active_higher_earlier_bid() -> Bid {
            Bid {
                auction_id: String::from("auction_id"),
                bidder_id: String::from("2"),
                id: 2,
                amount: TestData::higher_amount(),
                made_at: TestData::earlier_made_at(),
                removed_at: None,
            }
        }

        fn active_higher_later_bid() -> Bid {
            Bid {
                auction_id: String::from("auction_id"),
                bidder_id: String::from("3"),
                id: 3,
                amount: TestData::higher_amount(),
                made_at: TestData::later_made_at(),
                removed_at: None,
            }
        }

        fn inactive_lower_earlier_bid() -> Bid {
            Bid {
                auction_id: String::from("auction_id"),
                bidder_id: String::from("4"),
                id: 4,
                amount: TestData::lower_amount(),
                made_at: TestData::earlier_made_at(),
                removed_at: TestData::removed_at(),
            }
        }

        fn inactive_lower_later_bid() -> Bid {
            Bid {
                auction_id: String::from("auction_id"),
                bidder_id: String::from("5"),
                id: 5,
                amount: TestData::lower_amount(),
                made_at: TestData::later_made_at(),
                removed_at: TestData::removed_at(),
            }
        }

        fn inactive_higher_earlier_bid() -> Bid {
            Bid {
                auction_id: String::from("auction_id"),
                bidder_id: String::from("6"),
                id: 6,
                amount: TestData::higher_amount(),
                made_at: TestData::earlier_made_at(),
                removed_at: TestData::removed_at(),
            }
        }

        fn inactive_higher_later_bid() -> Bid {
            Bid {
                auction_id: String::from("auction_id"),
                bidder_id: String::from("7"),
                id: 7,
                amount: TestData::higher_amount(),
                made_at: TestData::later_made_at(),
                removed_at: TestData::removed_at(),
            }
        }
    }

    #[test]
    fn is_lower_bid_than_works() {
        let bids = vec![
            TestData::inactive_lower_later_bid(),
            TestData::inactive_lower_earlier_bid(),
            TestData::inactive_higher_later_bid(),
            TestData::inactive_higher_earlier_bid(),
            TestData::active_lower_later_bid(),
            TestData::active_lower_earlier_bid(),
            TestData::active_higher_later_bid(),
            TestData::active_higher_earlier_bid(),
        ];

        let mut index: usize = 0;
        for lower_bid in &bids[0..bids.len()] {
            index += 1;
            for higher_bid in &bids[index..bids.len()] {
                assert!(lower_bid.is_lower_bid_than(&higher_bid));
            }
        }
    }

    #[test]
    fn is_equivalent_bid_to_works() {
        let mut bids: Vec<Bid> = Vec::new();
        for i in 0..10 {
            bids.push(Bid{
                auction_id: "is_equivalent_bid_to_works".to_string(),
                bidder_id: i.to_string(),
                id: i,
                amount: TestData::higher_amount(),
                made_at: TestData::earlier_made_at(),
                removed_at: None
            })
        }

        let mut index: usize = 0;
        for follower in &bids[0..bids.len()] {
            index += 1;
            for leader in &bids[index..bids.len()] {
                assert!(follower.is_equivalent_bid_to(&leader));
            }
        }
    }

    #[test]
    fn is_higher_bid_than_works() {
        let bids = vec![
            TestData::active_higher_earlier_bid(),
            TestData::active_higher_later_bid(),
            TestData::active_lower_earlier_bid(),
            TestData::active_lower_later_bid(),
            TestData::inactive_higher_earlier_bid(),
            TestData::inactive_higher_later_bid(),
            TestData::inactive_lower_earlier_bid(),
            TestData::inactive_lower_later_bid(),
        ];

        let mut index: usize = 0;
        for higher_bid in &bids[0..bids.len()] {
            index += 1;
            for lower_bid in &bids[index..bids.len()] {
                assert!(higher_bid.is_higher_bid_than(&lower_bid));
            }
        }
    }
}

#[cfg(test)]
mod serialization_and_deserialization {
    use crate::models::v1::bid::Bid;

    #[test]
    fn it_can_serialize_and_deserialize() {
        let auction_id: String = String::from("auction_id");
        let bidder_id: String = String::from("bidder_id");
        let id: i32 = 0;
        let amount: i32 = 10000;
        let bid = Bid::new(auction_id.clone(), bidder_id.clone(), id, amount);

        let result_of_serialization = serde_json::to_string(&bid);
        assert!(result_of_serialization.is_ok());

        let data = result_of_serialization.unwrap();
        let result_of_deserialization = serde_json::from_str::<Bid>(&data);
        assert!(result_of_deserialization.is_ok());

        let deserialized_bid = result_of_deserialization.unwrap();
        assert_eq!(bid, deserialized_bid);
    }
}
