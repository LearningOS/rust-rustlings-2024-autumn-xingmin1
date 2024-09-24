/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Copy + PartialOrd>(array: &mut [T]) {
    //TODO
    if array.is_empty() {
        return;
    }
    quick_sort(0, array.len() - 1, array);
}

fn quick_sort<T: Copy + PartialOrd>(left: usize, right: usize, num: &mut [T]) {
    if right <= left {
        return;
    }

    let (mut i, mut j) = (left.wrapping_sub(1), right + 1);
    let x = median_three(num[left], num[left + (right - left) / 2], num[right]); 
    loop {
        loop {
            i = i.wrapping_add(1);
            if x <= num[i] {
                break;
            }
        }
        loop {
            j -= 1;
            if num[j] <= x {
                break;
            }
        }
        if i < j {
            num.swap(i, j);
        } else {
            break;
        }
    }
    quick_sort(left, j, num);
    quick_sort(j + 1, right, num);
}

fn median_three<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    if b <= a && a <= c {
        return a;
    } else if a <= b && b <= c {
        return b;
    }
    c
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