use std::{collections::HashMap, env, fs, process::exit};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Formula {
    amount: u64,
    ingredients: Vec<(String, u64)>,
}

fn parse_input(file_string: &str) -> Result<HashMap<String, Formula>> {
    let mut res: HashMap<String, Formula> = HashMap::new();

    for line in file_string.lines() {
        let mut parts = line.split(" => ");

        let ingredients = parts
            .next()
            .ok_or_else(|| anyhow!(""))?
            .split(", ")
            .map(|i| -> Result<(String, u64)> {
                let mut parts = i.split(" ");
                let amount = parts
                    .next()
                    .ok_or_else(|| anyhow!("Error parsing amount input"))?
                    .parse::<u64>()?;
                let name = parts
                    .next()
                    .ok_or_else(|| anyhow!("Error parsing name input"))?
                    .to_string();
                Ok((name, amount))
            })
            .collect::<Result<Vec<(String, u64)>>>()?;

        let mut parts = parts
            .next()
            .ok_or_else(|| anyhow!("Error parsing parts input"))?
            .split(' ');
        let amount = parts
            .next()
            .ok_or_else(|| anyhow!("Error parsing amount input"))?
            .parse::<u64>()?;
        let name = parts
            .next()
            .ok_or_else(|| anyhow!("Error parsing name input"))?
            .to_string();
        res.insert(
            name,
            Formula {
                amount,
                ingredients,
            },
        );
    }

    Ok(res)
}

fn part_1(input: &HashMap<String, Formula>) -> Result<u64> {
    fn get_ore_needed(
        name: &str,
        amount: u64,
        input: &HashMap<String, Formula>,
        leftovers: &mut HashMap<String, u64>,
    ) -> Result<u64> {
        if name == "ORE" {
            return Ok(amount);
        }

        let formula = input
            .get(name)
            .ok_or_else(|| anyhow!("{name} not in input mapping"))?;
        let mut ore_needed = 0;
        let mut amount = amount;

        if let Some(leftover) = leftovers.get_mut(name) {
            if *leftover >= amount {
                *leftover -= amount;
                return Ok(0);
            }

            amount -= *leftover;
            *leftover = 0;
        }

        let formula_amount = formula.amount;
        let times = amount.div_ceil(formula_amount);

        for (ingredient_name, ingredient_amount) in &formula.ingredients {
            ore_needed +=
                get_ore_needed(ingredient_name, ingredient_amount * times, input, leftovers)?;
        }

        let leftover = formula_amount * times - amount;
        *leftovers.entry(name.to_string()).or_insert(0) += leftover;

        Ok(ore_needed)
    }

    get_ore_needed("FUEL", 1, input, &mut HashMap::new())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;
    let input = parse_input(&contents)?;

    println!("Part 1: {:?}", part_1(&input)?);
    // println!("Part 2: {:?}", part_2(&input));

    Ok(())
}
