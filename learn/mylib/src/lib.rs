pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub mod add_salary{
    pub fn study(name:String){
        println!("Oriented Salary {}", name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
