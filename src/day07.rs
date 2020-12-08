use petgraph::{algo::has_path_connecting, graph::NodeIndex};
use petgraph::{visit::EdgeRef, Graph};

pub type BagGraph = Graph<&'static str, usize>;

pub fn first_star(graph: &BagGraph) -> usize {
  let shiny_gold = graph
    .node_indices()
    .find(|&i| graph[i] == "shiny gold")
    .unwrap();

  graph
    .node_indices()
    .filter(|&i| graph[i] != "shiny gold")
    .filter(|&i| has_path_connecting(graph, i, shiny_gold, None))
    .count()
}

pub fn second_star(graph: &BagGraph) -> usize {
  let shiny_gold = graph
    .node_indices()
    .find(|&i| graph[i] == "shiny gold")
    .unwrap();

  count_inside(graph, shiny_gold) - 1
}

fn count_inside(graph: &BagGraph, idx: NodeIndex<u32>) -> usize {
  let edges = graph.edges(idx);

  let mut sum = 1;

  for edge in edges {
    let target = edge.target();
    let count = *edge.weight();

    sum += count * count_inside(graph, target);
  }

  sum
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use super::*;
  use regex::Regex;

  // drab indigo bags contain 4 clear fuchsia bags, 3 plaid orange bags.
  fn input() -> BagGraph {
    let ctr = Regex::new(r"^(?P<color>[a-z]+ [a-z]+) bags contain").unwrap();
    let cnt = Regex::new(r"(?P<count>\d+) (?P<color>[a-z]+ [a-z]+) bags*").unwrap();

    let lines = include_str!("../data/07.txt").lines();

    let mut graph = BagGraph::new();
    let mut nodes = HashMap::new();

    for line in lines {
      let cap = ctr.captures(line).unwrap();
      let container = cap.name("color").unwrap().as_str();

      let root = *nodes
        .entry(container)
        .or_insert_with(|| graph.add_node(container));

      for cap in cnt.captures_iter(line) {
        let count: usize = cap.name("count").unwrap().as_str().parse().unwrap();
        let bag = cap.name("color").unwrap().as_str();

        let node = *nodes.entry(bag).or_insert_with(|| graph.add_node(bag));

        graph.add_edge(root, node, count);
      }
    }

    graph
  }

  #[test]
  fn part_01() {
    let data = input();
    let actual = first_star(&data);

    assert_eq!(actual, 302);
  }

  #[test]
  fn part_02() {
    let data = input();
    let actual = second_star(&data);

    assert_eq!(actual, 4165);
  }
}
