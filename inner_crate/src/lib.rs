pub fn add(left: usize, right: usize) -> usize {
    left + right
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

pub struct SomeStruct {
    a:u8,
    b:u8,
    c:u16
}


impl SomeStruct {
    pub fn new(a:u8,b:u8,c:u16)->SomeStruct {
        SomeStruct {
            a,
            b,
            c,
        }
    }

    pub fn sum(&self) -> u32 {
        let mut sum = 0;
        sum = sum+self.a as u32;
        sum = sum+self.b as u32;
        sum = sum+self.c as u32;
        return sum
    }
}