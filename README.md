# box_any
A wrap Box without generic type that allows dynamic dispatch / downcast.

Allows dynamic dispatch like Box<dyn Any> but without using fat pointer.
Provides a dynamic box type `BoxAny`, which contains a Box<T> value.
`BoxAny' is similar to Box<dyn Any> from `std::any::Any` but it doesn't use
fat pointer to dynamic dispatch.

## Example
```
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
