// This declaration will look for a file named `fs_mod.rs` and will
// insert its contents inside a module named `fs_mod` under this scope
mod fs_mod;


fn main() {
    // Modules allow disambiguation between items that have the same name.
    function();
    mod_visibility::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    mod_visibility::indirect_access();
    mod_visibility::nested::function();
    mod_visibility::call_public_function_in_mod_visibility(); // declared using `pub(in path)` syntax

    // pub(crate) items can be called from anywhere in the same crate
    mod_visibility::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_mod_visibility` is private
    // mod_visibility::nested::public_function_in_mod_visibility();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //mod_visibility::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //mod_visibility::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //mod_visibility::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //mod_visibility::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line

    // Public structs with public fields can be constructed as usual
    let open_box = mod_struct_visibility::OpenBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    //let closed_box = mod_struct_visibility::ClosedBox { _contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = mod_struct_visibility::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box._contents);
    // TODO ^ Try uncommenting this line

    deeply::nested::function();
    // Easier access to `deeply::nested::function`
    // Bind the `deeply::nested::function` path to `other_function`.
    use deeply::nested::function as other_function;
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    my::indirect_call();

    fs_mod::function();
    fs_mod::indirect_access();

    fs_mod::nested::function();
}

fn function() {
    println!("called `function()`");
}

// A module named `mod_visibility`
mod mod_visibility {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `mod_visibility::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `mod_visibility::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `mod_visibility::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `mod_visibility::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `mod_visibility::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::mod_visibility) fn public_function_in_mod_visibility() {
            print!("called `mod_visibility::nested::public_function_in_mod_visibility()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `mod_visibility::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `mod_visibility::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_mod_visibility() {
        print!("called `mod_visibility::call_public_function_in_mod_visibility()`, that\n> ");
        nested::public_function_in_mod_visibility();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `mod_visibility::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `mod_visibility::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `mod_visibility::private_nested::restricted_function()`");
        }
    }
}

mod mod_struct_visibility {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        _contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                _contents: contents,
            }
        }
    }
}



mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}


mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");
        
        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();
        
        // We can also use `self` to access another module inside `my`:
        self::cool::function();
        
        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();
        
        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

 



