/// 2468. 根据限制分割消息
/// 
/// 给你一个字符串 message 和一个正整数 limit 。
/// 你需要根据 limit 将 message 分割 成一个或多个 部分 。每个部分的结尾都是 "<a/b>" ，其中 "b" 用分割出来的总数 替换，
///  "a" 用当前部分所在的编号 替换 ，编号从 1 到 b 依次编号。除此以外，除了最后一部分长度 小于等于 limit 以外，
/// 其他每一部分（包括结尾部分）的长度都应该 等于 limit 。
/// 你需要确保分割后的结果数组，删掉每部分的结尾并 按顺序 连起来后，能够得到 message 。同时，结果数组越短越好。
/// 请你返回 message  分割后得到的结果数组。如果无法按要求分割 message ，返回一个空数组。
/// 