/// 983. 最低票价
/// 在一个火车旅行很受欢迎的国度，你提前一年计划了一些火车旅行。在接下来的一年里，你要旅行的日子将以一个名为 days 的数组给出。
/// 每一项是一个从 1 到 365 的整数。
/// 火车票有 三种不同的销售方式 ：
/// 一张 为期一天 的通行证售价为 costs[0] 美元；
/// 一张 为期七天 的通行证售价为 costs[1] 美元；
/// 一张 为期三十天 的通行证售价为 costs[2] 美元。
/// 通行证允许数天无限制的旅行。 例如，如果我们在第 2 天获得一张 为期 7 天 的通行证，
/// 那么我们可以连着旅行 7 天：第 2 天、第 3 天、第 4 天、第 5 天、第 6 天、第 7 天和第 8 天。
/// 返回 你想要完成在给定的列表 days 中列出的每一天的旅行所需要的最低消费。

pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {}

#[test]
fn test_1() {
    assert_eq!(mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);
}

#[test]
fn test_2() {
    assert_eq!(
        mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
        17
    );
}
