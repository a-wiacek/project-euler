use std::collections::VecDeque;

pub fn euler122() -> String {
    let bound = 200;
    let mut ans: Vec<Option<usize>> = vec![None; bound];
    let mut queue: VecDeque<Vec<u8>> = VecDeque::new();
    queue.push_back(vec![1]);
    while ans.iter().any(Option::is_none) && !queue.is_empty() {
        let queue_item = queue.pop_front().unwrap();
        let &last = queue_item.last().unwrap();
        let last_ans = queue_item.len() - 1;
        let i = last as usize - 1;
        if let Some(x) = ans[i] {
            // Optimal solution is achieved through optimal solutions for lower numbers
            if x < last_ans {
                continue;
            }
        }
        ans[i] = Some(last_ans);
        for x in queue_item
            .iter()
            .take_while(|&&x| x <= bound as u8 - last)
            .map(|&x| x + last)
        {
            let mut new_queue_item = queue_item.clone();
            new_queue_item.push(x);
            queue.push_back(new_queue_item);
        }
    }
    ans.into_iter()
        .map(Option::unwrap)
        .sum::<usize>()
        .to_string()
}
