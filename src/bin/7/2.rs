use std::{collections::HashMap, error::Error};

#[derive(Debug)]
struct Node {
  name: String,
  children: Vec<(usize, usize)>,
}

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("7.txt")?;
  let mut forest = HashMap::new();
  let mut storage = Vec::new();

  for line in buf.lines() {
    let line: Vec<&str> = line.split_whitespace().collect();
    let mut index = 0;
    let mut bag = String::new();
    while index < line.len() {
      let value = line[index];
      if value == "bags" {
        break;
      }
      if bag.len() > 0 {
        bag.push(' ');
      }
      bag.push_str(value);
      index += 1;
    }
    index += 2;

    let parent = *forest.entry(bag.clone()).or_insert({
      storage.push(Node {
        name: bag.clone(),
        children: Vec::new(),
      });
      storage.len() - 1
    });

    bag.clear();
    if line[index] == "no" {
      continue;
    }
    let mut count: usize = line[index].parse()?;
    index += 1;
    while index < line.len() {
      let value = line[index];
      if value == "bag," || value == "bags," || value == "bags." || value == "bag." {
        let node = *forest.entry(bag.clone()).or_insert({
          storage.push(Node {
            name: bag.clone(),
            children: Vec::new(),
          });
          storage.len() - 1
        });
        storage[parent].children.push((node, count));
        bag.clear();
        index += 1;
        if index < line.len() {
          count = line[index].parse()?;
          index += 1;
        }
        continue;
      }
      if bag.len() > 0 {
        bag.push(' ');
      }
      bag.push_str(value);
      index += 1;
    }
  }

  let value = forest.get(&"shiny gold".to_string());
  if let Some(value) = value {
    let mut bags = Vec::new();
    let mut to_search = vec![(*value, 1)];
    while let Some((node, count)) = to_search.pop() {
      to_search.extend(storage[node].children.iter().map(|c| (c.0, count * c.1)));
      bags.push((storage[node].name.clone(), count));
    }
    println!("{:?}", bags);
    println!("{:?}", bags.iter().map(|b| b.1).sum::<usize>() - 1);
    return Ok(());
  }
  println!("{}", 0);

  Ok(())
}
