mod types;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod tests {
    #[test]
    fn a_new_house_has_no_demand() {
        let t = ::types::House::new(1);
        assert!(!t.demand.hasDemand());
    }

    #[test]
    quickcheck!{
        fn demand_should_allow_for_getting_a_set_burger(burgs: u8) -> bool {
            let mut demand = ::types::Demand::new();
            demand.setBurgers(burgs);
            demand.burgers() == burgs
        }
    }
}
