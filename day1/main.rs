use std::fs;
fn main() {
    let file_content = fs::read_to_string("input.txt").unwrap();
    let number_vec : Vec<i16> = file_content.lines().map(|string_number| string_number.parse::<i16>().unwrap()).collect();
    part1(&number_vec);
    part2(&number_vec);
}
fn part1(number_vec : &Vec<i16>){
    let count = number_vec
      .windows(2)
      .filter(|x| x[0] < x[1])
      .count();
    println!("{:?}", count); 
}

fn part2(number_vec : &Vec<i16>){
      let averaged_numbers : Vec<i16> = number_vec
      .windows(3)
      .map("x" x.into_iter().sum()).collect();
      part1(&averaged_numbers);
}
