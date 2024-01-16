# box_any
A Box wrapper without generic type that allows dynamic dispatch / downcast
but without using fat pointer.

Provides a dynamic box type `BoxAny`, which contains a `Box<T>` value.
`BoxAny` is similar to Box<dyn Any> but it doesn't use
fat pointer to dynamic dispatch.

## Benchmarks
In this benchmark [bench](./benches/bench.rs) we create a Vec container composed by 4 Vec with different types (u8, u16, u32, u64) and then we insert 1M integer for each Vec type (`Vec<u8>, Vec<u16>, Vec<u32>, Vec<u64>`). To do this we use and compare two different approach:
- dyn box ([dyn Trait standard approach](https://bennett.dev/rust/downcast-trait-object/)): we create a Vec container with type: `Vec<Box<dyn Array>>` and we use downcast + push method to insert 1M integer for each Vec type. 
- box_any: we create a Vec container with type `Vec<BoxAny>` and we use BoxAny downcast + push method to insert 1M integer for each Vec type.

As we can see from the result, using `BoxAny` approach to downcast and fill each Vec  is faster then using `dyn Trait` approach.
```
cargo bench
    Finished bench [optimized] target(s) in 0.03s
     Running unittests src/lib.rs (target/release/deps/box_any-d326196ff75eb72f)

running 1 test
test tests::test ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-06d6896e0a110c0e)
dyn box checked         time:   [14.393 ms 14.410 ms 14.431 ms]
                        change: [-3.8738% -2.8496% -2.0529%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

box_any checked         time:   [5.5236 ms 5.6059 ms 5.7449 ms]
                        change: [-8.4525% -5.6854% -2.4171%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  8 (8.00%) high mild
  5 (5.00%) high severe

box_any checked #2      time:   [4.2085 ms 4.2173 ms 4.2287 ms]
                        change: [-6.8000% -6.2270% -5.7308%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe

```

## Example
```rust
use std::any:: TypeId;
use box_any::BoxAny;

let box_u32 = Box::new(032);
let box_string = Box::new(String::from("Hello World"));
let box_any_u32: BoxAny = box_u32.clone().into();
let box_any_string: BoxAny = box_string.clone().into();
let box_any_vec: Vec<BoxAny> = vec![box_any_u32, box_any_string];
assert!(box_any_vec[1].is::<String>());
let string_2: &String = box_any_vec[1].downcast_ref::<String>().unwrap();
assert_eq!(string_2, box_string.as_ref());
```
