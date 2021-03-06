#[cfg(test)]
mod tests {
    #[test]
    fn playground() {
        struct Closure<F, T> {
            data: (u8, u16),
            func: F,
            func_r: T,
        }

        impl<F, T> Closure<F, T>
        where
            F: Fn(&(u8, u16)) -> u8,
            for<'a> T: Fn(&'a (u8, u16)) -> &'a u8,
        {
            fn call(&self, r: u8) -> u8 {
                (self.func)(&(self.data.0 + r, self.data.1))
            }

            fn call_r<'a>(&'a self) -> &'a u8 {
                (self.func_r)(&self.data)
            }
        }

        fn do_it(data: &(u8, u16)) -> u8 {
            data.0 + 10
        }

        fn h_it(data: &(u8, u16)) -> u8 {
            data.0 + 20
        }

        fn k_it(data: &(u8, u16)) -> &u8 {
            &data.0
        }

        fn some_condition() -> bool {
            true
        }

        fn as_str<'a>(data: &'a u32) -> String {
            return format!("{}", data);
        }

        let mut data = vec![1, 2, 3];
        let x = &data[0];

        if some_condition() {
            println!("{}", x); // This is the last use of `x` in this branch
            data.push(4); // So we can push here
        } else {
            // There's no use of `x` in here, so effectively the last use is the
            // creation of x at the top of the example.
            data.push(5);
        }

        let clo = Closure {
            data: (0, 1),
            func: do_it,
            func_r: k_it,
        };
        let other = Closure {
            data: (0, 1),
            func: h_it,
            func_r: k_it,
        };

        println!("{}", clo.call(1));
        println!("{}", other.call(2));

        // assert!(false);
    }
}
