struct Buffer<T> {
    data: Vec<T>,
}

impl<T: std::ops::Add<Output = T> + Copy> Buffer<T> {
    fn sum(&self) -> T {
        let mut total = self.data[0];
        for i in 1..self.data.len() {
            total = total + self.data[i];
        }
        total
    }
}

fn compare_string(x: &str, y: &str) -> bool {
    let x_bytes = x.as_bytes();
    let y_bytes = y.as_bytes();

    let len_diff = x_bytes.len().cmp(&y_bytes.len());

    if len_diff == std::cmp::Ordering::Greater {
        return true;
    } else if len_diff == std::cmp::Ordering::Less {
        return false;
    }

    for i in 0..x_bytes.len() {
        let x_byte = x_bytes[i];
        let y_byte = y_bytes.get(i).copied().unwrap_or(0);

        if x_byte > y_byte {
            return true;
        } else if x_byte < y_byte {
            return false;
        }
    }

    false
}

fn main() {
    let buffer = Buffer { data: vec![1, 2, 3, 4, 5] };
    let sum = buffer.sum();
    println!("Sum: {}", sum);

    let x = "abcd";
    let y = "abce";
    let result = compare_string(x, y);

    if result {
        println!("x > y");
    } else {
        println!("x <= y");
    }

    let a = vec!['a', 'b', 'c', 'd', 'e'];

    let new_vec: Vec<char> = a
        .iter()
        .map(|&c| (c as u8 + 1) as char)
        .collect();

    println!("{:?}", new_vec);
}