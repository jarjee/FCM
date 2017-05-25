struct Demand {
    Burgers: u8,
    Pizzas: u8,
    Beers: u8,
    Sodas: u8,
    Lemonade: u8,
}

impl Demand {
    pub fn hasDemand(&self) -> bool {
        return (self.Burgers + self.Pizzas + self.Beers + self.Sodas + self.Lemonade) == 0;
    }

    pub fn burgers(&self) -> u8 {
        return self.Burgers;
    }

    pub fn pizzas(&self) -> u8 {
        return self.Pizzas;
    }

    pub fn beers(&self) -> u8 {
        return self.Beers;
    }

    pub fn sodas(&self) -> u8 {
        return self.Sodas;
    }

    pub fn lemonade(&self) -> u8 {
        return self.Lemonade;
    }

    fn new() -> Demand {
        Demand {
            Burgers: 0,
            Pizzas: 0,
            Beers: 0,
            Sodas: 0,
            Lemonade: 0,
        }
    }
}

pub struct House {
    Id: u8,
    Demand: Demand,
}

impl House {
    pub fn new(id: u8) -> House {
        House {
            Id: id,
            Demand: Demand::new(),
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
    Id: u8,
    Demand: Demand,
    Type: AdvertType,
}
