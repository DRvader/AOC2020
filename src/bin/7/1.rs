use std::{
  collections::{HashMap, HashSet},
  error::Error,
};

struct Node {
  name: String,
  parents: HashSet<usize>,
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
    index += 3;

    let parent = *forest.entry(bag.clone()).or_insert({
      storage.push(Node {
        name: bag.clone(),
        parents: HashSet::new(),
      });
      storage.len() - 1
    });

    bag.clear();
    while index < line.len() {
      let value = line[index];
      if value == "bag," || value == "bags," || value == "bags." || value == "bag." {
        let node = *forest.entry(bag.clone()).or_insert({
          storage.push(Node {
            name: bag.clone(),
            parents: HashSet::new(),
          });
          storage.len() - 1
        });
        storage[node].parents.insert(parent);
        bag.clear();
        index += 2;
        continue;
      } else if value == "other" {
        break;
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
    let mut bags = HashSet::new();
    let mut to_search = vec![*value];
    let mut searched = Vec::new();
    while let Some(node) = to_search.pop() {
      if searched.contains(&node) {
        continue;
      }
      searched.push(node);
      to_search.extend(storage[node].parents.iter());
      bags.insert(storage[node].name.clone());
    }
    println!("{:?}", bags);
    println!("{:?}", bags.len() - 1);
    return Ok(());
  }
  println!("{}", 0);

  Ok(())
}
