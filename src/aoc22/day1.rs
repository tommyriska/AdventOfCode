use std::collections::{BTreeMap, HashMap};
use std::fs;
use crate::utils;

pub fn solve() {
    partOne()
}

fn partOne() {
    // Read the input file and convert it to a string
    let input = fs::read_to_string("input/aoc22/day1.txt").unwrap();
    // Split the input string into lines
    let lines: Vec<&str> = input.lines().collect();

    // Initialize a HashMap to store the total calories for each Elf
    let mut elf_calories = BTreeMap::new();

    // Initialize a variable to keep track of the current Elf
    let mut current_elf = None;

    // Parse the lines in the input file
    for line in lines {
        // If the line is empty, it indicates the start of a new Elf's inventory
        if line.trim() == "" {
            current_elf = None;
            continue;
        }

        // If the line is not empty, it contains the calories of a food item
        // If we haven't encountered a new Elf yet, it must be the first line for the current Elf
        if current_elf.is_none() {
            current_elf = Some(line.trim());
            elf_calories.insert(current_elf.unwrap(), 0);
        }

        // Add the calories of the food item to the total calories for the current Elf
        let calories = elf_calories.get_mut(current_elf.unwrap()).unwrap();
        *calories += line.trim().parse::<i32>().unwrap();
    }

    // Find the Elf carrying the most calories
    let mut max_calories = 0;
    let mut max_elf = None;
    for (elf, calories) in elf_calories.iter() {
        if *calories > max_calories {
            max_calories = *calories;
            max_elf = Some(elf);
        }
    }


    let answer = format!("The Elf carrying the most calories is {} with a total of {} calories", max_elf.unwrap(), max_calories);
    utils::printAnswer(1, answer.as_str());
}

fn part_two() {

}