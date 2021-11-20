///<h2>The Solution of Joseph`s Survival</h2>
///<h2>约瑟夫生存算法</h2>
///据说著名犹太历史学家Josephus有过以下的故事：在罗马人占领乔塔帕特后，39 个犹太人与Josephus及他的朋友躲到一个洞中，39个犹太人决定宁愿死也不要被敌人抓到，于是决定了一个自杀方式，41个人排成一个圆圈，由第1个人开始报数，每报数到第3人该人就必须自杀，然后再由下一个重新报数，直到所有人都自杀身亡为止。然而Josephus 和他的朋友并不想遵从。首先从一个人开始，越过k-2个人（因为第一个人已经被越过），并杀掉第k个人。接着，再越过k-1个人，并杀掉第k个人。这个过程沿着圆圈一直进行，直到最终只剩下一个人留下，这个人就可以继续活着。问题是，给定了和，一开始要站在什么地方才能避免被处决。Josephus要他的朋友先假装遵从，他将朋友与自己安排在第16个与第31个位置，于是逃过了这场死亡游戏。
pub fn calculate() {
    let mut list = [1; 41];
    let mut times_to_kill = 0;
    let mut dead_index = 0;
    let mut survivors = list.len();
    'outer: loop {
        for i in 0..list.len() {
            times_to_kill += list[i];
            if times_to_kill % 3 == 0 && list[i] > 0 {
                list[i] = 0;
                dead_index += 1;
                survivors -= 1;
                println!("{}:{}", dead_index, i + 1);
                times_to_kill = 0;

                if survivors <= 0 {
                    break 'outer;
                }
            }
        }
    }
}