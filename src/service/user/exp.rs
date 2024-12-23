use crate::model::user::exp::ExpReward;

impl ExpReward {
    pub fn sum_without_coin(&self) -> u32 {
        let mut sum = 0;
        [self.login, self.watch, self.share]
            .into_iter()
            .for_each(|x| sum += x as u32 * 5);

        sum += self.coins;
        sum
    }
}
