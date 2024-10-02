/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord + Clone>(array: &mut [T]){
    // pointer?

    // // insertion sort
    // if array.len() <= 1 { return; }
    // for _ in 0..array.len() {
    //     let val = array[0].clone();
    //     for i in 1..=array.len() {
    //         if (i == array.len() || val < array[i]) {
    //             shift_left(array, i - 1);
    //             array[i - 1] = val.clone();
    //         } else {
    //             continue;
    //         }
    //     }
    // }

    for (mut i) in 1..array.len() {
        while i > 0 && array[i - 1] > array [i] {
            array.swap(i-1, i);
            i -= 1;
        }
    }

    // insert_first_val_at(array, array[0].clone(), array.len() - 1)
}

// given index is never more than array length
// fn insert_first_val_at<T: Clone>(array: &mut [T], val: T, index: usize) {
//     shift_left(array, index);
//     array[index] = val;
// }


// given index is never more than array length
// fn shift_left<T: Clone>(array: &mut [T], index: usize) {
//     for i in 1..=index {
//         array[i - 1] = array[i].clone();
//     }
// }


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