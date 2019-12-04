trait Pass {
    fn has_double(&self) -> bool;
    fn has_solo_double(&self) -> bool;
}
impl Pass for i32 {
    fn has_double(&self) -> bool {
        let mut i = *self;
        while i != 0 {
            if i % 10 == (i/10) % 10 {
                return true;
            }
            i /= 10;
        }
        false
    }
    fn has_solo_double(&self) -> bool {
        let mut i = *self * 10;
        while i != 0 {
            let d = (i/10) % 10;
            if i%10 != d && (i/100)%10 == d && (i/1000)%10 != d {
                return true;
            }
            i /= 10;
        }
        false
    }
}

struct PasswordIter(i32);
impl Iterator for PasswordIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let mut i = self.0;
        let mut tens = 1;
        let mut ones = 1;

        while i % 10 == 9 {
            i /= 10;
            tens *= 10;
            ones = (ones*10)+1;
        }
        i += 1;
        i = i*tens + (i % 10) * (ones / 10);
        self.0 = i;
        Some(i)
    }
}

fn main() {
    let from = 171309;
    let to = 643603;

    let between = move |min, max| PasswordIter(0).skip_while(move |i| i<min).take_while(move |i| i<max);
    let part1 = between(&from, &to).filter(Pass::has_double).count();
    let part2 = between(&from, &to).filter(Pass::has_solo_double).count();
    
    println!("{}", part1);
    println!("{}", part2);
}
