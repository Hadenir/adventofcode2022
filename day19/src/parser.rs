use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Vec<Blueprint> {
    let (_, blueprints) = blueprint_list(input).expect("Failed to parse puzzle input");
    blueprints
}

fn integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn resource_type(input: &str) -> IResult<&str, ResourceType> {
    alt((
        map(tag("ore"), |_| ResourceType::Ore),
        map(tag("clay"), |_| ResourceType::Clay),
        map(tag("obsidian"), |_| ResourceType::Obsidian),
        map(tag("geode"), |_| ResourceType::Geode),
    ))(input)
}

fn resource(input: &str) -> IResult<&str, Resource> {
    map(
        separated_pair(integer, space1, resource_type),
        |(count, res_type)| Resource::new(res_type, count),
    )(input)
}

fn resource_list(input: &str) -> IResult<&str, Vec<Resource>> {
    separated_list1(delimited(space1, tag("and"), space1), resource)(input)
}

fn robot(input: &str) -> IResult<&str, Robot> {
    map(
        separated_pair(
            delimited(tag("Each "), resource_type, tag(" robot")),
            space1,
            delimited(tag("costs "), resource_list, char('.')),
        ),
        |(output, cost)| Robot::new(output, cost),
    )(input)
}

fn blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(
        separated_pair(
            delimited(tag("Blueprint "), integer, char(':')),
            space1,
            separated_list1(space1, robot),
        ),
        |(id, robots)| Blueprint::new(id, robots),
    )(input)
}

fn blueprint_list(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list0(line_ending, blueprint)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let input = "123";

        let (rem, int) = integer(input).unwrap();

        assert_eq!(int, 123);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_resource_type() {
        let input = "ore";

        let (rem, res_type) = resource_type(input).unwrap();

        assert_eq!(res_type, ResourceType::Ore);
        assert!(rem.is_empty());

        let input = "clay";

        let (rem, res_type) = resource_type(input).unwrap();

        assert_eq!(res_type, ResourceType::Clay);
        assert!(rem.is_empty());

        let input = "obsidian";

        let (rem, res_type) = resource_type(input).unwrap();

        assert_eq!(res_type, ResourceType::Obsidian);
        assert!(rem.is_empty());

        let input = "geode";

        let (rem, res_type) = resource_type(input).unwrap();

        assert_eq!(res_type, ResourceType::Geode);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_resource() {
        let input = "12 obsidian";

        let (rem, resource) = resource(input).unwrap();

        assert_eq!(resource.count, 12);
        assert_eq!(resource.resource_type, ResourceType::Obsidian);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_resource_list() {
        let input = "1 ore and 2 clay";

        let (rem, resources) = resource_list(input).unwrap();

        assert_eq!(resources[0].count, 1);
        assert_eq!(resources[0].resource_type, ResourceType::Ore);
        assert_eq!(resources[1].count, 2);
        assert_eq!(resources[1].resource_type, ResourceType::Clay);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_robot() {
        let input = "Each ore robot costs 2 ore.";

        let (rem, robot) = robot(input).unwrap();

        assert_eq!(robot.output, ResourceType::Ore);
        assert_eq!(robot.cost.len(), 1);
        assert_eq!(robot.cost[0].count, 2);
        assert_eq!(robot.cost[0].resource_type, ResourceType::Ore);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_blueprint() {
        let input = "Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 9 obsidian.";

        let (rem, blueprint) = blueprint(input).unwrap();

        assert_eq!(blueprint.id, 1);
        assert_eq!(blueprint.robots.len(), 4);
        assert_eq!(blueprint.robots[0].output, ResourceType::Ore);
        assert_eq!(blueprint.robots[1].output, ResourceType::Clay);
        assert_eq!(blueprint.robots[2].output, ResourceType::Obsidian);
        assert_eq!(blueprint.robots[3].output, ResourceType::Geode);
        assert!(rem.is_empty());
    }
}
