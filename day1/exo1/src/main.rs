use core::{iter::Iterator, option::Option::Some};

fn distance(a : &i32, b : &i32) -> i32
{
    if a < b
    {
        return b - a
    }
    else
    {
        return a - b
    }
}

fn main() {
    let file = core::include_str!("list.txt");
    // file.spli
    let (mut col1, mut col2) : (Vec<i32> , Vec<i32>) =

    file.lines().filter_map(|line| {
        let mut parts = line.split_whitespace();
        let num1 : i32 = parts.next().unwrap().parse().ok().unwrap();
        let num2 : i32 = parts.next().unwrap().parse().ok().unwrap();
        Some((num1, num2))
    }).unzip();


    col1.sort();
    col2.sort();
    let mut dist : i32 = 0;
    for(a,b) in col1.iter().zip(col2.iter())
    {
        dist += distance(&a,&b);
    }
    println!("Distance is {} ", dist);
}
