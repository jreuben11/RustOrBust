#![allow(dead_code)]

use std::sync::Arc;

struct Foo {
    x: u32,
    y: u16,
}
struct Bar {
    a: u32,
    b: u16,
}
fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}


#[derive(Clone)]
struct ContainerA<T>(Arc<T>);
#[allow(noop_method_call)]
fn clone_containers_a<T>(foo: &ContainerA<i32>, bar: &ContainerA<T>) {
    let _foo_cloned: ContainerA<i32> = foo.clone();
    let _bar_cloned: &ContainerA<T> = bar.clone(); // autoref
}
struct ContainerB<T>(Arc<T>);
impl<T> Clone for ContainerB<T> /* where T: Clone */ {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
fn clone_containers_b<T>(foo: &ContainerB<i32>, bar: &ContainerB<T>) {
    let _foo_cloned: ContainerB<i32> = foo.clone();
    let _bar_cloned: ContainerB<T> = bar.clone();
}

fn main() {
    let _bar: Bar = reinterpret(Foo{x:1, y:2});
}
