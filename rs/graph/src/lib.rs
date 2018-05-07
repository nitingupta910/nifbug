pub fn foo() -> u32 {
    println!("graph: foo called");
    9
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
