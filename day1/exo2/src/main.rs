fn main() {
    let file = core::include_str!("../../exo1/src/list.txt");

    let (mut col1, mut col2) : (Vec<i32> , Vec<i32>) =

    file.lines().filter_map(|line| {
        let mut parts = line.split_whitespace();
        let num1 : i32 = parts.next().unwrap().parse().ok().unwrap();
        let num2 : i32 = parts.next().unwrap().parse().ok().unwrap();
        Some((num1, num2))
    }).unzip();

    col1.sort();
    col2.sort();

    let mut score : i32 = 0;
    for a in col1.iter(){
        let matches: i32 = col2.iter().filter(|&x| x == a).count() as i32;
        score += matches * a;
    }

    println!("{}", score); // Prints 3 (two 2s and one 3)
}
