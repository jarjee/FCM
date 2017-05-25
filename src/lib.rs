mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn a_new_house_has_no_demand() {
        let t = ::types::House::new(1);
        assert!(!t.demand.hasDemand());
    }
}
