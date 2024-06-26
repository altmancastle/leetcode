/// 1944. 队列中可以看到的人数
///
/// 有 n 个人排成一个队列，从左到右 编号为 0 到 n - 1 。给你以一个整数数组 heights ，每个整数 互不相同，heights[i] 表示第 i 个人的高度。
/// 一个人能 看到 他右边另一个人的条件是这两人之间的所有人都比他们两人 矮 。更正式的，第 i 个人能看到第 j 个人的条件是 i < j 且
///  min(heights[i], heights[j]) > max(heights[i+1], heights[i+2], ..., heights[j-1]) 。
/// 请你返回一个长度为 n 的数组 answer ，其中 answer[i] 是第 i 个人在他右侧队列中能 看到 的 人数 。
///
///
pub fn can_see_persons_count_v2(heights: Vec<i32>) -> Vec<i32> {
    let mut st = vec![];
    let mut res = vec![0; heights.len()];
    for (i, &x) in heights.iter().rev().enumerate() {
        while let Some(&p) = st.last() {
            res[i] += 1;
            if p > x {
                break;
            }
            st.pop();
        }
        st.push(x);
    }
    res.into_iter().rev().collect()
}

#[test]
fn can_see_persons_count_test() {
    assert_eq!(
        can_see_persons_count_v2(vec![10, 6, 8, 5, 11, 9]),
        vec![3, 1, 2, 1, 1, 0]
    );
}
