fn main() {
//     print::run();
//     // println!("ssss");
//
//     println!("{name} = kossher", name = "kosssherssss");
//
//     let kossher = "s";
//
//     println!("max i32 is {}", i32::MAX);
//
//     // Primitive str = Immutable fixed-length string somewhere in memory
// // String = Growable, heap-allocated data structure - Use when you need to modify or own string data
//
//     let mut hello = String::from("kossher");
//     println!("length is {}", hello.len());
//
//     hello.push('s');
//     hello.push_str("ssssssss");
//
//     println!("contains = {}", hello.contains("shit"));
//
//     //loop
//     for word in hello.split_whitespace() {
//         println!("{}", word);
//     }
//
//     let arrr: [i8; 5] = [1, 2, 3, 4, 5];
//     println!("{:?}", arrr);
//
//     println!("size : {}", std::mem::size_of_val(&arrr));
//
//     let mut nums1: Vec<i8> = vec![1, 2, 3, 4];
//     let mut nums2 = [1, 2, 3, 4];
//     println!("size : {}", std::mem::size_of_val(&nums1));
//     println!("size : {}", std::mem::size_of_val(&nums2));
//
//     let slice1: &[i8] = &nums1[1..2];
//     let slice2: &[i8] = &nums2[1..2];
//
//     println!("{:?}", slice1);
//     println!("{:?}", slice2);
//
//     for n in nums1.iter_mut() {
//         *n += 2;
//         println!("{}", n);
//     }
//
//     fuck("ssss", "xxxx");

    // let vec1 = vec![1, 2, 3];
    // let vec2 = &vec1;
    // println!("{:?}", vec1);

    let mut p = Person::new("as", "sb");
    println!("{}", p.full_name());
    p.set_last_name("ksss");
    println!("{}", p.full_name());
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }
}

// fn fuck(greet: &str, shit: &str) {
//     println!("{} {}", greet, shit);
// }
//
// fn add(a: i8, b: i8) -> i8 {
//     a + b
// }