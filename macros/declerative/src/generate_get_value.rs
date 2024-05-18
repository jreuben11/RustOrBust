macro_rules! generate_get_value_string {
    ($struct_type:ident) => {
        generate_get_value_string!($struct_type, String);
    };
    ($struct_type:ident,$return_type:ident) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    };
}

macro_rules! generate_from {
    ($struct_type:ident) => {
        generate_from!($struct_type, String);
    };
    ($struct_type:ident,$return_type:ty) => {
        impl From<$struct_type> for $return_type {
            fn from(f: $struct_type) -> Self {
                f.value
            }
        }
    };
}

macro_rules! generate_newtypes_methods {
    ($struct_type:ident) => {
        generate_get_value_string!($struct_type, String);
        generate_from!($struct_type, String);
    };
    ($struct_type:ident,$return_type:ty) => {
        generate_get_value_string!($struct_type, $return_type);
        generate_from!($struct_type, $return_type);
    };
}
