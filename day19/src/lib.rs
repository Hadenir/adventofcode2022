mod parser;

use std::collections::HashMap;

use parser::parse_input;

pub fn solve_part_1(input: &str) -> u32 {
    let blueprints = parse_input(input);

    let mut quality_levels = Vec::new();
    for blueprint in &blueprints {
        let maximum_demand: HashMap<ResourceType, u32> = blueprint.robots
            .iter()
            .flat_map(|robot| &robot.cost)
            .fold(HashMap::new(), |mut maxs, res| {
                maxs.entry(res.resource_type).and_modify(|v| { *v = (*v).max(res.count); }).or_insert(res.count);
                maxs
            });


        let key = (0, HashMap::new(), HashMap::from([(ResourceType::Ore, 1)]));
        let mut states = Vec::from([key]);
        let mut geodes = vec![];
        'outer: while let Some((time, mut resources, production)) = states.pop() {

            if time == 24 {
                geodes.push(*resources.get(&ResourceType::Geode).unwrap_or(&0));
                continue;
            }

            dbg!(&production);
            for robot in blueprint.robots.iter().rev() {
                if production.contains_key(&robot.output) && production[&robot.output] >= maximum_demand[&robot.output] {
                    continue;
                }

                if can_afford(&resources, robot) {
                    let (mut resources, mut production) = (resources.clone(), production.clone());
                    produce(&mut resources, &production);
                    build(robot, &mut resources, &mut production);
                    states.push((time + 1, resources, production));

                    if robot.output == ResourceType::Obsidian || robot.output == ResourceType::Geode {
                        continue 'outer;
                    }
                }
            }

            produce(&mut resources, &production);

            states.push((time + 1, resources, production));
        }

        dbg!(blueprint.id);
        dbg!(geodes.iter().max().unwrap());
        quality_levels.push(geodes.into_iter().max().unwrap() * blueprint.id);
    }

    dbg!(&quality_levels);
    quality_levels.into_iter().sum()
}

pub fn solve_part_2(input: &str) -> usize {
    todo!()
}

fn produce(resources: &mut HashMap<ResourceType, u32>, production: &HashMap<ResourceType, u32>) {
    for (&res_type, &amount) in production.iter() {
        *resources.entry(res_type).or_default() += amount;
    }
}

fn can_afford(resources: &HashMap<ResourceType, u32>, robot: &Robot) -> bool {
    robot.cost.iter().all(|cost| {
        resources.contains_key(&cost.resource_type) && resources[&cost.resource_type] >= cost.count
    })
}

fn build(
    robot: &Robot,
    resources: &mut HashMap<ResourceType, u32>,
    production: &mut HashMap<ResourceType, u32>,
) {
    robot.cost.iter().for_each(|cost| {
        resources
            .entry(cost.resource_type)
            .and_modify(|amount| *amount -= cost.count);
    });

    *production.entry(robot.output).or_default() += 1;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct Resource {
    resource_type: ResourceType,
    count: u32,
}

#[derive(Debug, Clone)]
struct Robot {
    output: ResourceType,
    cost: Vec<Resource>,
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    robots: Vec<Robot>,
}

impl Resource {
    fn new(resource_type: ResourceType, count: u32) -> Self {
        Self {
            resource_type,
            count,
        }
    }
}

impl Robot {
    fn new(output: ResourceType, cost: Vec<Resource>) -> Self {
        Self { output, cost }
    }
}

impl Blueprint {
    fn new(id: u32, robots: Vec<Robot>) -> Self {
        Self { id, robots }
    }
}
