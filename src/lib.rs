mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let t = ::types::House::new(1);
        t.Demand::hasDemand();
    }
}
