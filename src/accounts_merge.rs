use std::collections::{HashMap, HashSet};
/// 721. 账户合并
/// 给定一个列表 accounts，每个元素 accounts[i] 是一个字符串列表，
/// 其中第一个元素 accounts[i][0] 是 名称 (name)，其余元素是 emails 表示该账户的邮箱地址。
/// 现在，我们想合并这些账户。如果两个账户都有一些共同的邮箱地址，则两个账户必定属于同一个人。
/// 请注意，即使两个账户具有相同的名称，它们也可能属于不同的人，
/// 因为人们可能具有相同的名称。一个人最初可以拥有任意数量的账户，但其所有账户都具有相同的名称。
/// 合并账户后，按以下格式返回账户：每个账户的第一个元素是名称，其余元素是 按字符 ASCII 顺序排列 的邮箱地址。
/// 账户本身可以以 任意顺序 返回。
pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    vec![]
}

#[test]
fn test_1() {
    let accounts = vec![
        vec![
            "John".to_string(),
            "johnsmith@mail.com".to_string(),
            "john00@mail.com".to_string(),
        ],
        vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
        vec![
            "John".to_string(),
            "johnsmith@mail.com".to_string(),
            "john_newyork@mail.com".to_string(),
        ],
        vec!["Mary".to_string(), "mary@mail.com".to_string()],
    ];
    let ans = vec![
        vec![
            "John".to_string(),
            "john00@mail.com".to_string(),
            "john_newyork@mail.com".to_string(),
            "johnsmith@mail.com".to_string(),
        ],
        vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
        vec!["Mary".to_string(), "mary@mail.com".to_string()],
    ];
    assert_eq!(accounts_merge(accounts.clone()), ans);
}
