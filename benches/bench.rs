use criterion::{criterion_group, criterion_main, Criterion};
use box_any::BoxAny;
use std::any::Any;

fn box_any(containers: &mut Vec<BoxAny>, n: usize, checked: bool) {
    if checked {
        for i in 0..n {
            push_into_box_any(containers, i);
        }    
    } else {
        for i in 0..n {
            push_into_box_any_unchecked(containers, i);
        }    
    }
}

fn new_containers_box_any(n: usize) -> Vec<BoxAny> {
    let v_u8 = new_box_any_vec::<u8>(n);
    let v_u16 = new_box_any_vec::<u16>(n);
    let v_u32 = new_box_any_vec::<u32>(n);
    let v_u64 = new_box_any_vec::<u64>(n);
    let containers: Vec<BoxAny> = vec![v_u8, v_u16, v_u32, v_u64];
    containers
}

fn clear_containers_box_any(containers: &mut Vec<BoxAny>) {
    containers[0].downcast_mut::<Vec<u8>>().unwrap().clear();
    containers[1].downcast_mut::<Vec<u16>>().unwrap().clear();
    containers[2].downcast_mut::<Vec<u32>>().unwrap().clear();
    containers[3].downcast_mut::<Vec<u64>>().unwrap().clear();
}

fn push_into_box_any(containers: &mut Vec<BoxAny>, index: usize) {
    containers[0].downcast_mut::<Vec<u8>>().unwrap().push((index & 0xFF) as u8);
    containers[1].downcast_mut::<Vec<u16>>().unwrap().push((index & 0xFFFF) as u16);
    containers[2].downcast_mut::<Vec<u32>>().unwrap().push(index as u32);
    containers[3].downcast_mut::<Vec<u64>>().unwrap().push(index as u64);
}

fn push_into_box_any_unchecked(containers: &mut Vec<BoxAny>, index: usize) {
    unsafe {
        containers[0].downcast_mut_unchecked::<Vec<u8>>().push((index & 0xFF) as u8);
        containers[1].downcast_mut_unchecked::<Vec<u16>>().push((index & 0xFFFF) as u16);
        containers[2].downcast_mut_unchecked::<Vec<u32>>().push(index as u32);
        containers[3].downcast_mut_unchecked::<Vec<u64>>().push(index as u64);
    };
}


fn new_box_any_vec<T: 'static>(capacity: usize) -> BoxAny {
    Box::new(Vec::<T>::with_capacity(capacity)).into()
}

trait Array {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> Array for Vec<T> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn dyn_box(containers: &mut Vec<Box<dyn Array>>, n: usize) {
    for i in 0..n {
        push_into_dyn_box(containers, i);
    }
}

fn new_containers_dyn_box(n: usize) -> Vec<Box<dyn Array>> {
    let v_u8 = new_dyn_box_vec::<u8>(n);
    let v_u16 = new_dyn_box_vec::<u16>(n);
    let v_u32 = new_dyn_box_vec::<u32>(n);
    let v_u64 = new_dyn_box_vec::<u64>(n);
    let containers: Vec<Box<dyn Array>> = vec![v_u8, v_u16, v_u32, v_u64];
    containers
}

fn clear_containers_dyn_box(containers: &mut Vec<Box<dyn Array>>) {
    containers[0].as_any_mut().downcast_mut::<Vec<u8>>().unwrap().clear();
    containers[1].as_any_mut().downcast_mut::<Vec<u16>>().unwrap().clear();
    containers[2].as_any_mut().downcast_mut::<Vec<u32>>().unwrap().clear();
    containers[3].as_any_mut().downcast_mut::<Vec<u64>>().unwrap().clear();
}

fn push_into_dyn_box(containers: &mut Vec<Box<dyn Array>>, index: usize) {
    containers[0].as_any_mut().downcast_mut::<Vec<u8>>().unwrap().push((index & 0xFF) as u8);
    containers[1].as_any_mut().downcast_mut::<Vec<u16>>().unwrap().push((index & 0xFFFF) as u16);
    containers[2].as_any_mut().downcast_mut::<Vec<u32>>().unwrap().push(index as u32);
    containers[3].as_any_mut().downcast_mut::<Vec<u64>>().unwrap().push(index as u64);
}

fn new_dyn_box_vec<T>(capacity: usize) -> Box<Vec<T>> {
    Box::new(Vec::with_capacity(capacity))
}

fn criterion_benchmark(c: &mut Criterion) {
    let n = 1 * 1000 * 1000;
    let mut containers_dyn_box = new_containers_dyn_box(n);
    let mut containers_box_any = new_containers_box_any(n);
    c.bench_function("dyn box checked", |b| b.iter(|| {
        clear_containers_dyn_box(&mut containers_dyn_box);
        dyn_box(&mut containers_dyn_box, n);
    }));
    c.bench_function("box_any checked", |b| b.iter(|| {
        clear_containers_box_any(&mut containers_box_any);
        box_any(&mut containers_box_any, n, true);
    }));
    c.bench_function("box_any checked", |b| b.iter(|| {
        clear_containers_box_any(&mut containers_box_any);
        box_any(&mut containers_box_any, n, false);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
