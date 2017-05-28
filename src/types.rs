pub struct Demand {
    demand: u16,
}

impl Demand {
    pub fn hasDemand(&self) -> bool {
        return (self.burgers() + self.pizzas() + self.beers() + self.sodas() + self.lemonade()) > 0;
    }

    #[cfg(test)]
    pub fn demandArray(&self) -> [u8; 5] {
        return [self.burgers(),
                self.pizzas(),
                self.beers(),
                self.sodas(),
                self.lemonade()];
    }

    #[cfg(test)]
    pub fn setDemandArray(&mut self, demandArr: [u8; 5]) {
        self.setBurgers(demandArr[0]);
        self.setPizzas(demandArr[1]);
        self.setBeers(demandArr[2]);
        self.setSodas(demandArr[3]);
        self.setLemonade(demandArr[4]);
    }

    pub fn setBurgers(&mut self, amount: u8) {
        self.demand = (self.demand - self.burgers() as u16) + amount as u16;
    }

    pub fn burgers(&self) -> u8 {
        return (self.demand % 7) as u8;
    }

    pub fn setPizzas(&mut self, amount: u8) {
        self.demand = (self.demand - self.pizzas() as u16 * 7) + amount as u16 * 7;
    }

    pub fn pizzas(&self) -> u8 {
        return (((self.demand) / 7) % 7) as u8;
    }

    pub fn setBeers(&mut self, amount: u8) {
        self.demand = (self.demand - self.beers() as u16 * 49) + amount as u16 * 49;
    }

    pub fn beers(&self) -> u8 {
        return (((self.demand) / 49) % 7) as u8;
    }

    pub fn setSodas(&mut self, amount: u8) {
        self.demand = (self.demand - self.sodas() as u16 * 343) + amount as u16 * 343;
    }

    pub fn sodas(&self) -> u8 {
        return (((self.demand) / 343) % 7) as u8;
    }

    pub fn setLemonade(&mut self, amount: u8) {
        self.demand = (self.demand - self.lemonade() as u16 * 2401) + amount as u16 * 2401;
    }

    pub fn lemonade(&self) -> u8 {
        return (((self.demand) / 2401) % 7) as u8;
    }

    pub fn new() -> Demand {
        Demand { demand: 0 }
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
