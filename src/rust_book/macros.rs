macro_rules! add_one {
    // rule
    ($input:expr) => { $input + 1 } // expansion
}


/** SOME BUILDER ATTEMPT **/

macro_rules! def_builder {
    ( $src_name:ident => $dest_name:ident {
        $( $attr_name:ident : $attr_type:ty = $attr_default:expr ),*
    }) => {
        struct $dest_name {
            $( $attr_name : $attr_type ),*
        }

        struct $src_name {
            $( $attr_name : Option<$attr_type> ),*
        }

        impl $src_name {
            pub fn new() -> $src_name {
                $src_name {
                    $(
                        $attr_name : $attr_default
                    ),*
                }
            }

            pub fn build(self) -> Result<$dest_name, &'static str> {
                let err = "Argument missing";

                $(
                    let $attr_name = try!(self.$attr_name.ok_or(err));
                )*

                Ok($dest_name {
                    $( $attr_name : $attr_name ),*
                })
            }

            $(
                fn $attr_name(mut self, value: $attr_type) -> Self {
                    self.$attr_name = Some(value);
                    self
                }
            )*
        }
    }
}

#[test]
fn test_builder_macro() {
    assert_eq!(add_one!(1), 2);
}

#[test]
fn builder_test() {

}