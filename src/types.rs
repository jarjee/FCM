pub struct Demand {
    burgers: u8,
    pizzas: u8,
    beers: u8,
    sodas: u8,
    lemonade: u8,
}

impl Demand {
    pub fn hasDemand(&self) -> bool {
        return (self.burgers + self.pizzas + self.beers + self.sodas + self.lemonade) > 0;
    }

    pub fn burgers(&self) -> u8 {
        return self.burgers;
    }

    pub fn pizzas(&self) -> u8 {
        return self.pizzas;
    }

    pub fn beers(&self) -> u8 {
        return self.beers;
    }

    pub fn sodas(&self) -> u8 {
        return self.sodas;
    }

    pub fn lemonade(&self) -> u8 {
        return self.lemonade;
    }

    fn new() -> Demand {
        Demand {
            burgers: 0,
            pizzas: 0,
            beers: 0,
            sodas: 0,
            lemonade: 0,
        }
    }
}

pub struct House {
    pub id: u8,
    pub demand: Demand,
}

impl House {
    pub fn new(id: u8) -> House {
        House {
            id: id,
            demand: Demand::new(),
        }
    }
}

enum AdvertType {
    Bilboard,
    Mailbox,
    Airplane,
    RadioTower,
}

struct Advert {
    id: u8,
    demand: Demand,
    category: AdvertType,
}
