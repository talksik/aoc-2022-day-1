async fn read_elf_calories(file: &str) -> Vec<u32> {
    // read file
    let data = std::fs::read_to_string(file).expect("Unable to read file");

    // put data into a vector
    let mut result: Vec<u32> = vec![];
    for line in data.lines() {
        if let Ok(num) = line.parse::<u32>() {
            result.push(num);
        } else {
            result.push(0);
        }
    }

    result
}

// (elf winner, winner calories)
fn get_elf_with_max_calories(inventory: &[u32]) -> (u32, u32) {
    let mut current_calories: u32 = 0;
    let mut last_max_calories: u32 = 0;
    // the current elf that is eating
    let mut current_elf: u32 = 1;
    let mut current_winner_elf: u32 = 1;

    for (index, &item) in inventory.iter().enumerate() {
        current_calories += item;

        if item == 0 || index == inventory.len() - 1 {
            if current_calories > last_max_calories {
                last_max_calories = current_calories;
                current_winner_elf = current_elf;
            }
            current_calories = 0;
            current_elf += 1;
        }
    }

    (current_winner_elf, last_max_calories)
}

// takes a list of calories and bunches them up per elf
fn consolidate_calories(inventory: &[u32]) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];

    let mut current_elf_calories: u32 = 0;
    for (index, &item) in inventory.iter().enumerate() {
        current_elf_calories += item;

        if item == 0 || index == inventory.len() - 1 {
            result.push(current_elf_calories);
            current_elf_calories = 0;
        }
    }

    result
}

// sort the array of calories by max first
fn sort_vec(arr: &mut [u32]) -> &mut [u32] {
    arr.sort_by(|a, b| b.cmp(a));
    arr
}

// (elf winners, winners' calories)
fn top_three_elves_calories(inventory: &[u32]) -> u32 {
    let mut consolidated = consolidate_calories(inventory);
    let sorted = sort_vec(&mut consolidated);

    let mut result: u32 = 0;
    for elf_calories in sorted.iter().take(3) {
        result += elf_calories;
    }

    result
}

// write test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_elf_with_max_calories() {
        assert_eq!(get_elf_with_max_calories(&[1, 2, 3, 4, 0, 45]).1, 45);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let inventory = read_elf_calories("./src/input.txt").await;
    let elf_with_max_calories = get_elf_with_max_calories(&inventory);

    let test_inventory = read_elf_calories("./src/test.txt").await;
    let test_elf_with_max_calories = get_elf_with_max_calories(&test_inventory);

    println!("Elf with max calories: {}", elf_with_max_calories.1);

    println!(
        "Test Elf with max calories: {}",
        test_elf_with_max_calories.1
    );

    let top_three_elves = top_three_elves_calories(&inventory);
    println!("Top three elves calories: {}", top_three_elves);

    let test_top_three = top_three_elves_calories(&test_inventory);
    println!("Test top three elves calories: {}", test_top_three);

    Ok(())
}
