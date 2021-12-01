use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parameters {
    pub small_to_large_dist: i32,
    pub large_to_small_dist: i32,
    pub reachable_space: Snakes,
    pub reachable_food: Snakes,
    pub head_tail_dist: Snakes,
    pub length: Snakes,
    pub health: Snakes,
    pub dist_food_lthan_health: Snakes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Snakes(i32, i32);

impl Parameters {
    pub fn create_next_round(&self, count: usize, mutate_range: i32) -> Vec<Parameters> {
        let mut out = vec![];
        for _ in 0..count {
            out.push(*self);
            out.split_last_mut().unwrap().0.mutate(mutate_range);
        }
        out
    }
    fn mutate(&mut self, mutate_range: i32) {
        self.small_to_large_dist += fastrand::i32((-mutate_range)..mutate_range);
        self.large_to_small_dist += fastrand::i32((-mutate_range)..mutate_range);
        self.reachable_food.mutate(mutate_range);
        self.reachable_space.mutate(mutate_range);
        self.head_tail_dist.mutate(mutate_range);
        self.length.mutate(mutate_range);
        self.health.mutate(mutate_range);
        self.dist_food_lthan_health.mutate(mutate_range);
    }
}

impl Snakes {
    pub fn mutate(&mut self, mutate_range: i32) {
        self.0 += fastrand::i32((-mutate_range)..mutate_range);
        self.1 += fastrand::i32((-mutate_range)..mutate_range);
    }
}
