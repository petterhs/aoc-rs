fn day1_run() {
    println!("Hello, day 1");

    //read input file
    let input = std::fs::read_to_string("input/1").unwrap();

    //split input into lines
    let lines: Vec<&str> = input.split("\n").collect();

    //Array of the three elves with the highest total calories
    let mut top_3_elves: [(u32, u32); 3] = [(0, 0), (0, 0), (0, 0)];

    let mut elf_calories = 0;
    let mut elf_number = 0;
    for line in lines {
        if line.len() == 0 {
            //Check if this elf is in the top 3
            if elf_calories > top_3_elves[0].1 {
                top_3_elves[0] = (elf_number, elf_calories);
                top_3_elves.sort_by(|a, b| a.1.cmp(&b.1));
            }
            elf_number += 1;
            elf_calories = 0;
        } else {
            let calories: u32 = line.parse().unwrap();
            elf_calories += calories;
        }
    }
    let mut total_calories = 0;
    println!("Top 3 elves: ");
    for elf in top_3_elves.iter() {
        print!("Nr. {}, Calories: {} \n", elf.0, elf.1);
        total_calories += elf.1;
    }
    println!("Total calories: {}", total_calories);
}

fn main() {
    day1_run();
}
