// Constants must be typed and not left to compiler to figure out
const DEFAULT_ARRAY_LENGTH: usize = 5;

/// We are using a generic type T here since we want value to be generic.
///
/// This reads as: MultiDimensionalArray is generic over T (type of its elements) T
/// and
/// ARRAY_LENGTH (number of elements representing vector as boxed array of N COUNTs).
///
/// Essentially, we are enabling a core-only implementation but utilizing stack and stack allocations.
///
/// A const generic parameter may be any integer, char or bool types
/// here we also give it a default value.
struct MultiDimensionalArray<T, const ARRAY_LENGTH: usize = DEFAULT_ARRAY_LENGTH> {
    data: Box<[Box<[T; ARRAY_LENGTH]>]>,
    default_value: T,
    size: usize,
    sub_start: usize,
    pos_sub: usize,
}

// for "heapless" vector, allocate
// 1 - in static memory
// 2 - function's stack frame
// augment with a usize to track how many elements it holds
//

impl<T: std::marker::Copy + std::fmt::Debug, const ARRAY_LENGTH: usize>
    MultiDimensionalArray<T, ARRAY_LENGTH>
{
    fn new(size: usize, default_value: T) -> Self {
        // Boxed array that takes in default_value and ARRAY_LENGTH
        // [[1, 2, 3], 2]
        let data = (0..size)
            .map(|_| Box::new([default_value; ARRAY_LENGTH]))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        MultiDimensionalArray {
            data,
            default_value,
            size,
            sub_start: 0,
            pos_sub: 0,
        }
    }

    fn next(&mut self) -> Option<T> {
        if self.sub_start == self.size {
            println!("No more elements left");
            return None;
        }

        println!(
            "ARRAY NUMBER {:?} POSITION IN ARRAY {:?}",
            self.pos_sub, self.sub_start
        );

        let sub_array = *self.data[self.sub_start];
        let res = sub_array[self.pos_sub];

        if self.pos_sub + 1 == ARRAY_LENGTH {
            self.sub_start += 1;
            self.pos_sub = 0;
        } else {
            self.pos_sub += 1;
        }

        Some(res)
    }
}

fn main() {
    // create an iterator which clones all of the elements
    // will error since iterator cannot be collected into into a fixed-sized array
    //
    // iter() - creates iterate over &T (by reference)
    // iter_mut() - creates iterate over &mut T (by mutable reference)
    // into_iter() - creates iterate over T (by value)
    // array
    // let array: [i8; 3] = [1, 2, 3].iter().cloned().collect();

    // using turbofish ::<_, 3> operator here to create to set sub array length
    let mut multi_array = MultiDimensionalArray::<_, 3>::new(3, 1);

    // println!("{:?}", multi_array.data);

    // for val in multi_array.next() {
    // println!("Aha {:?}", val);
    //     for inner_val in val.iter() {
    //         println!("Yes {:?}", inner_val);
    //     }
    // }

    // TODO: we could use Index and IndexMut here to make it cleaner to access elements in the array
    let a = &mut multi_array.data[0];
    a[0] = 10;

    while let Some(val) = multi_array.next() {
        println!("{:?}", val);
    }

    // multi_array.next();
    // multi_array.next();
    // multi_array.next();

    // println!("---");

    // multi_array.next();
    // multi_array.next();
    // multi_array.next();

    // println!("---");

    // multi_array.next();
    // multi_array.next();
    // multi_array.next();

    // println!("---");

    // multi_array.next();
    // multi_array.next();
    // multi_array.next();
}
