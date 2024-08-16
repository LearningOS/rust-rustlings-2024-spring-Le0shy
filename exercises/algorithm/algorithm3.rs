/*
	sort
 	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::cmp::min;

fn sort<T:Ord>(array: &mut [T]){
	//TODO
    for i in 0.. array.len() - 1 {
        let mut min = &array[i];
        let mut min_idx = i;
        let mut j = i + 1;
        while j < array.len() {
            if array[j] < *min {
                min_idx = j;
                min = &array[j];
            }
            j += 1;
        }
        array.swap(i, min_idx);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}