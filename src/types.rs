pub struct Demand {
    burgers: u8,
    pizzas: u8,
    beers: u8,
    sodas: u8,
    lemonade: u8,
}

impl Demand {
    pub fn hasDemand(&self) -> bool {
        return (self.burgers() + self.pizzas() + self.beers() + self.sodas() + self.lemonade()) > 0;
    }

    pub fn demandArray(&self) -> [u8; 5] {
        return [self.burgers(),
                self.pizzas(),
                self.beers(),
                self.sodas(),
                self.lemonade()];
    }

    pub fn setDemandArray(&mut self, demandArr: [u8; 5]) {
        self.burgers = demandArr[0];
        self.pizzas = demandArr[1];
        self.beers = demandArr[2];
        self.sodas = demandArr[3];
        self.lemonade = demandArr[4];
    }

    pub fn setBurgers(&mut self, amount: u8) {
        self.burgers = amount;
    }

    pub fn burgers(&self) -> u8 {
        return self.burgers;
    }

    pub fn setPizzas(&mut self, amount: u8) {
        self.pizzas = amount;
    }

    pub fn pizzas(&self) -> u8 {
        return self.pizzas;
    }

    pub fn setBeers(&mut self, amount: u8) {
        self.beers = amount;
    }

    pub fn beers(&self) -> u8 {
        return self.beers;
    }

    pub fn setSodas(&mut self, amount: u8) {
        self.sodas = amount;
    }

    pub fn sodas(&self) -> u8 {
        return self.sodas;
    }

    pub fn setLemonade(&mut self, amount: u8) {
        self.lemonade = amount;
    }

    pub fn lemonade(&self) -> u8 {
        return self.lemonade;
    }

    pub fn new() -> Demand {
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
