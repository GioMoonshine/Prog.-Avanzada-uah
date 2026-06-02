/*
fn sort(arr: &mut Vec<i32>, n: usize) {
    for i in 1..n {
        // n elements

        let key = arr[i]; // The current integer to be inserted
                          // println!("{}th element {}", i, key);
                          // Inserts a[i] into the sorted array A[1:i-1]
        let mut j = (i - 1) as i32;

        // Ensure that the current comparison target does not go out of array bounds
        // Find A[j] > key in the sorted part, and end the while loop when j reaches 0

        // If the target comparison is out of bounds (less than the first element), stop
        // For descending order, change > key to < key
        while j >= 0 && arr[j as usize] > key {

            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1; // Move left for the next comparison
        }
        // When the while loop ends, j is one position to the left of where key will be inserted
        arr[(j + 1) as usize] = key;
    }
}

fn bubble_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..arr.len()-1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }

    return arr;
}

fn merge(mut arr: Vec<i32>, left: usize, mid: usize, right: usize) -> Vec<i32> {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut L1 = arr.clone();
    let mut R1 = arr.clone();
    let L = &L1[left..mid];
    let R = &R1[mid..right];
    /* Merge the temp arrays back into arr[l..r]*/
    let mut i = 0; // Initial index of first subarray
    let mut j = 0; // Initial index of second subarray
    let mut k = left; // Initial index of merged subarray
    while i < n1 && j < n2 {
        if L[i] < R[j] {
            arr[k] = L[i];
            i = i + 1;
        } else {
            arr[k] = R[j];
            j = j + 1;
        }
        k = k + 1;
    }
    while i < n1 {
        arr[k] = L[i];
        i = i + 1;
        k = k + 1;
    }
    /* Copy the remaining elements of R[], if there
    are any */
    while j < n2 {
        arr[k] = R[j];
        j = j + 1;
        k = k + 1;
    }
    arr
}

fn merge_sort(mut arr: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort(arr, left, mid);
        arr = merge_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    arr
}

fn main() {
    let mut test_array1 = vec![10, 8, 3, 5, 6, 1, 12];
    let n = test_array1.len();
    println!("Vector original: {:?}", test_array1);
    sort(&mut test_array1, n);
    println!("Vector modificado(Insertion): {:?}", test_array1);

    let mut test_array2 = vec![10, 8, 3, 5, 6, 1, 12];
    println!("Vector original: {:?}", test_array2);
    bubble_sort(&mut test_array2);
    println!("Vector modificado(Bubble): {:?}", test_array2);

    let mut test_array3 = vec![10, 8, 3, 5, 6, 1, 12];
    println!("Vector original: {:?}", test_array3);
    let arr3 = merge_sort(test_array3.clone(), 0, test_array3.len());
    println!("Vector modificado(Merge): {:?}", arr3);
}
*/
fn insercion(v: &mut Vec<i32>){
    let n = v.len();

    for i in 1..n{
        let actual = v[i];
        let mut j = i;
        while j>0 && v[j-1] > actual {
            v[j] = v[j-1];
            j-= 1;
        }
        v[j] = actual;
    }
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in 0..n {
        for j in 0..(n - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    let mid = len / 2;

    // Ordenar cada mitad
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    // Mezclar las dos mitades ordenadas
    let mut merged = Vec::with_capacity(len);

    let (left, right) = arr.split_at(mid);
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        merged.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        merged.push(right[j]);
        j += 1;
    }

    arr.copy_from_slice(&merged);
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];

    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}
/*
fn main(){
    let mut datos = vec![42, 7, 19, 3, 25, 1, 18];

    println!("Vector original:");
    println!("{:?}", datos);

    quick_sort(&mut datos);

    println!("Vector ordenado:");
    println!("{:?}", datos);
}

*/

fn find_value(arr:Vec<i32>, n:i32) {
    let l = arr.len();
    let mut found = false;
    for i in 0..l {
        if arr[i] == n {
            println!("Valor encontrado en posición: {:?}", i);
            found = true;
            break;
        }
    }
    if !found {
        println!("Valor no encontrado");
    }
}

fn draw(n: usize) {
    for i in 0..n+1 {
        println!("{:+<1$}", "", i);
    }
}

fn main() {
    let mut array = vec![4, 6, 5, 8, 2];
    find_value(array.clone(), 5);

    draw(4);
}