mod types;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod tests {
    #[cfg(test)]
    use quickcheck::*;

    #[test]
    fn a_new_house_has_no_demand() {
        let t = ::types::House::new(1);
        assert!(!t.demand.hasDemand());
    }

    #[test]
    quickcheck!{
        fn demand_should_allow_for_getting_a_set_burger(burgs: u8) -> bool {
            let mut demand = ::types::Demand::new();

            let safeBurger = burgs % 7;

            demand.setBurgers(safeBurger);
            demand.burgers() == safeBurger
        }
    }

    #[cfg(test)]
    #[derive(Clone, Debug)]
    struct DemandArray {
        burgers: u8,
        pizzas: u8,
        beers: u8,
        sodas: u8,
        lemonade: u8,
    }

    #[cfg(test)]
    impl Arbitrary for DemandArray {
        fn arbitrary<G: Gen>(g: &mut G) -> DemandArray {
            DemandArray {
                burgers: (g.next_u32() as u8) % 7,
                pizzas: (g.next_u32() as u8) % 7,
                beers: (g.next_u32() as u8) % 7,
                sodas: (g.next_u32() as u8) % 7,
                lemonade: (g.next_u32() as u8) % 7,
            }
        }
    }

    #[cfg(test)]
    impl DemandArray {
        #[cfg(test)]
        pub fn to_sized_array(&self) -> [u8; 5] {
            return [self.burgers,
                    self.pizzas,
                    self.beers,
                    self.sodas,
                    self.lemonade];
        }
    }

    #[test]
    quickcheck!{
        fn demand_we_set_we_can_get(arbitraryDemand: DemandArray) -> bool {
            let demandArr = arbitraryDemand.to_sized_array();

            let mut demand = ::types::Demand::new();

            demand.setDemandArray(demandArr);
            let storedDemand = demand.demandArray();

            return demandArr == storedDemand;
        }
    }
}
