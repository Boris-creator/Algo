use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sort(mut arr: Vec<u8>) -> Vec<u8> {
    let len = arr.len();
    merge_sort(&mut arr, 0, len);
    arr
}
fn merge_sort<T: Ord + Copy>(arr: &mut Vec<T>, start: usize, end: usize) {
    if end - start == 1 {
        return;
    }
    let middle = (start + end) / 2;
    merge_sort(arr, start, middle);
    merge_sort(arr, middle, end);
    merge(arr, start, middle, end);
}
fn merge<T: Ord + Copy>(arr: &mut Vec<T>, start: usize, middle: usize, end: usize) {
    let mut index = start;
    let mut buffer1 = Vec::new();
    let mut buffer2 = Vec::new();
    for i in start..middle {
        buffer1.push(arr[i]);
    }
    for i in middle..end {
        buffer2.push(arr[i]);
    }

    while !buffer1.is_empty() && !buffer2.is_empty() {
        if buffer1[0] <= buffer2[0] {
            arr[index] = buffer1.remove(0);
        } else {
            arr[index] = buffer2.remove(0);
        }
        index += 1;
    }
    while !buffer1.is_empty() {
        arr[index] = buffer1.remove(0);
        index += 1;
    }
    while !buffer2.is_empty() {
        arr[index] = buffer2.remove(0);
        index += 1;
    }
}
