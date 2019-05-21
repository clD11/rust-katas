macro_rules! def_setters {
    (
        $($Something:item),*
    ) => {
            age: u32,
            first_name: String,
            second_name: String

            pub fn first_name(&mut self, n: String) -> &mut Self {
                self.first_name = n;
                self
            }
    }
}

struct Something {

}

#[test]
fn test_builder_macro() {
    let name = String::from("test");
    let Something::first_name(name);
}