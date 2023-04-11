// compare https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/heap_sort.rs
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_heap(arr: Vec<u8>) -> Vec<u8> {
    heapify(&arr)
}
fn heapify<T: Ord + fmt::Display + Copy>(arr: &Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let mut index = 0;
    while index < arr.len() {
        result.push(arr[index]);
        bubble_up(&mut result, index);
        index+=1;
    }
    result
}
fn bubble_up<T: Ord>(arr: &mut Vec<T>, index: usize) {
    if index == 0 {
        return;
    }
    let parent_index = get_parent(index);
    if arr[index] > arr[parent_index] {
        return;
    }
    arr.swap(index, parent_index);
    bubble_up(arr, parent_index);
}
fn get_parent(index: usize) -> usize {
    (index - 1) / 2
}

