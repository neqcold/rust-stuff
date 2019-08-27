use std::iter::FromIterator;

enum Taste {
    Reeee,
    Noice,
    Yeaah,
}

impl Taste {
    fn as_str(&self) -> &'static str{
        match self {
            Taste::Reeee => "Reeee",
            Taste::Noice => "Noice",
            Taste::Yeaah => "Yeaah",
        }
    }
}

enum GotFromChicken {
    Tendies { n: u32, taste: Taste },
    Cancer { stage: u8 },
    Depression,
}

impl GotFromChicken {
    fn new_tendies(n: u32, taste: Taste) -> GotFromChicken {
        assert!(n > 0);
        GotFromChicken::Tendies { n, taste }
    }

    fn new_cancer(stage: u8) -> GotFromChicken {
        assert!((1..5).contains(&stage)); //[1; 4]
        GotFromChicken::Cancer { stage }
    }

    fn new_depression() -> GotFromChicken {
        GotFromChicken::Depression
    }
}

fn chicken_gimme_tendies() -> GotFromChicken {
    GotFromChicken::new_tendies(5, Taste::Reeee)
}

fn main() {
    type GFC = GotFromChicken;
    match chicken_gimme_tendies() {
        GFC::Tendies { n, taste: Taste::Reeee } => println!("R{}", String::from_iter(['E'; 100].into_iter())),
        GFC::Tendies { n, taste } => println!("Got me {} tendies\n{}", n, taste.as_str()),
        GFC::Cancer { stage } => println!("Wanted some tasty tendies - got stage {} cancer instead :c", stage),
        GFC::Depression => println!("JUST FUCKKEN KILL YOURSELF"),
    }
}

