use std::collections::{BTreeMap, HashMap};
use std::fs;
use crate::{printAnswer, utils};

pub fn solve() {
    // Initialize a HashMap to store the total calories for each Elf
    let mut elf_calories = HashMap::new();

    // Initialize a variable to keep track of the current Elf
    let mut current_elf = None;
    let mut elf_id = 0;

    // Parse the lines in the input file
    for line in utils::get_input_as_lines("input/aoc22/day1.txt") {
        // If the line is empty, it indicates the start of a new Elf's inventory
        if line.trim() == "" {
            current_elf = None;
            elf_id += 1;
            continue;
        }

        // If the line is not empty, it contains the calories of a food item
        // If we haven't encountered a new Elf yet, it must be the first line for the current Elf
        if current_elf.is_none() {
            current_elf = Some(elf_id);
            elf_calories.insert(current_elf.unwrap(), 0);
        }

        // Add the calories of the food item to the total calories for the current Elf
        let calories = elf_calories.get_mut(&current_elf.unwrap()).unwrap();
        *calories += line.trim().parse::<i32>().unwrap();
    }

    let mut elf_calories_vec: Vec<(&i32, &i32)> = elf_calories.iter().collect();
    elf_calories_vec.sort_by(|(_, a), (_, b)| b.cmp(a));
    let calories= elf_calories_vec.iter().next().unwrap().1;
    let answer_day1 = format!("The Elf with the most calories has {} calories", calories);
    printAnswer(1, 1, answer_day1.as_str());

    let mut values: Vec<i32> = elf_calories.values().cloned().collect();
    values.sort_by(|a, b| b.cmp(a));
    let top_three: i32 = values.iter().take(3).sum();
    let answer_day2 = format!("The top three elfs are carrying a total of {} calories", top_three);
    printAnswer(1, 2, answer_day2.as_str())
}