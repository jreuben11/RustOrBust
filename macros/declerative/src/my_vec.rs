macro_rules! my_vec {
    () => [
      Vec::new()
    ];
    (make an empty vec) => (
      Vec::new()
    );
    {$x:expr} => {
      {
        let mut v = Vec::new();
        v.push($x);
        v
      }
    };
    [$($x:expr),+] => (
      {
        let mut v = Vec::new();
        $(
          v.push($x);
        )+
        v
      }
    )
}
