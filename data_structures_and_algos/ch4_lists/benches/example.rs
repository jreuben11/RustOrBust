#[macro_use]
extern crate bencher;

use rand::thread_rng;
use rand::Rng;
use std::collections::LinkedList;
use bencher::Bencher;

fn a(bench: &mut Bencher) {
    bench.iter(|| {
        (0..1000).fold(0, |x, y| x + y)
    })
}

fn b(bench: &mut Bencher) {
    const N: usize = 1024;
    bench.iter(|| {
        vec![0u8; N]
    });
 
    bench.bytes = N as u64;
}

const LIST_ITEMS: u64 = 15_000;


fn bench_std_linked_list_find(b: &mut Bencher) {
    let mut list = std::collections::LinkedList::new();
    let items: Vec<String> = (0..LIST_ITEMS).map(|i| format!("INSERT INTO mytable VALUES ({})", i).to_owned()).collect();
    for item in items.iter() {
        list.push_back(item.clone());
    }
    let mut rng = thread_rng();

    b.iter(|| {
        let r = rng.gen_range(0..LIST_ITEMS as usize);
        list.iter().find(|&x| x == &items[r]).expect("NOT FOUND")
    });
}



fn bench_vec_find(b: &mut Bencher) {
    let mut list = vec![];

    for i in 0..LIST_ITEMS {
        list.push((i, format!("INSERT INTO mytable VALUES ({})", i).to_owned()));
    }
    let mut rng = thread_rng();

    b.iter(|| {
        let r = rng.gen_range(0..LIST_ITEMS);
        list.iter().find(|&x| x.0 == r).expect("NOT FOUND")
    });
}

benchmark_group!(benches, a, b, bench_std_linked_list_find, bench_vec_find);
benchmark_main!(benches);