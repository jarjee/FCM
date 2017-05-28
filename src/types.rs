pub struct Demand {
    demand: u16,
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

    #[cfg(test)]
    pub fn demandArray(&self) -> [u8; 5] {
        let mut demandStore = self.demand;

        let burgers = (demandStore % 7) as u8;
        demandStore /= 7;
        let pizzas = (demandStore % 7) as u8;
        demandStore /= 7;
        let beers = (demandStore % 7) as u8;
        demandStore /= 7;
        let sodas = (demandStore % 7) as u8;
        demandStore /= 7;
        let lemonade = demandStore as u8;

        return [burgers, pizzas, beers, sodas, lemonade];
    }

    #[cfg(test)]
    pub fn setDemandArray(&mut self, demandArr: [u8; 5]) {
        self.demand = demandArr[4] as u16;
        self.demand *= 7;
        self.demand += demandArr[3] as u16;
        self.demand *= 7;
        self.demand += demandArr[2] as u16;
        self.demand *= 7;
        self.demand += demandArr[1] as u16;
        self.demand *= 7;
        self.demand += demandArr[0] as u16;
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
            demand: 0,
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
