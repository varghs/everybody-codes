use std::fmt::Display;
use itertools::Itertools;

#[derive(Clone, Copy)]
enum Enemy {
    AncientAnt = 0,
    BadassBeetle = 1,
    CreepyCockroach = 3,
    DiabolicalDragonfly = 5
}

impl TryFrom<char> for Enemy {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Enemy::AncientAnt),
            'B' => Ok(Enemy::BadassBeetle),
            'C' => Ok(Enemy::CreepyCockroach),
            'D' => Ok(Enemy::DiabolicalDragonfly),
            _ => Err("Unexpected token.".to_string())
        }
    }
}

pub fn part1(s: &str) -> impl Display {
    s.chars()
        .map(|c: char| c.try_into().map(|e: Enemy| e as i32).unwrap())
        .sum::<i32>()
}

fn score_part2(enemy1: char, enemy2: char) -> i32 {
    match (enemy1, enemy2) {
        ('x', 'x') => 0,
        ('x', a) => a.try_into().map(|e: Enemy| e as i32).unwrap(),
        (a, 'x') => a.try_into().map(|e: Enemy| e as i32).unwrap(),
        (a, b) => a.try_into().map(|e: Enemy| e as i32).unwrap() + b.try_into().map(|e: Enemy| e as i32).unwrap() + 2
    }
}

pub fn part2(s: &str) -> impl Display {
    s.chars().chunks(2).into_iter()
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .map(|(enemy1, enemy2)| score_part2(enemy1, enemy2)).sum::<i32>()
}

fn score_part3(enemy1: char, enemy2: char, enemy3: char) -> i32 {
    match (enemy1, enemy2, enemy3) {
        ('x', 'x', 'x') => 0,
        (a, 'x', 'x') => a.try_into().map(|e: Enemy| e as i32).unwrap(),
        ('x', a, 'x') => a.try_into().map(|e: Enemy| e as i32).unwrap(),
        ('x', 'x', a) => a.try_into().map(|e: Enemy| e as i32).unwrap(),
        ('x', a, b) => score_part2(a, b),
        (a, 'x', b) => score_part2(a, b),
        (a, b, 'x') => score_part2(a, b),
        (a, b, c) => score_part2(a, b) + c.try_into().map(|e: Enemy| e as i32).unwrap() + 4
    }
}

pub fn part3(s: &str) -> impl Display {
    s.chars().chunks(3).into_iter()
        .map(|mut x| (x.next().unwrap(), x.next().unwrap(), x.next().unwrap()))
        .map(|(enemy1, enemy2, enemy3)| score_part3(enemy1, enemy2, enemy3)).sum::<i32>()
}