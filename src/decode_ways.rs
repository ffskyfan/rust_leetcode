/*
A message containing letters from A-Z can be encoded into numbers using the following mapping:

'A' -> "1"
'B' -> "2"
...
'Z' -> "26"
To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:

"AAJF" with the grouping (1 1 10 6)
"KJF" with the grouping (11 10 6)
Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".

Given a string s containing only digits, return the number of ways to decode it.

The test cases are generated so that the answer fits in a 32-bit integer.

 

Example 1:
Input: s = "12"
Output: 2
Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).


Example 2:
Input: s = "226"
Output: 3
Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).

Example 3:
Input: s = "06"
Output: 0
Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").
 

Constraints:
    1 <= s.length <= 100
    s contains only digits and may contain leading zero(s).

*/


pub fn num_decodings_ex(s:&str,result_vec: &mut Vec<i32>) -> i32 {

    let char_count = s.chars().count();
    if char_count == 0 {
        return 0;
    }

    let mut count_one = 0;

    let mut chars = s.chars();
    let first_char= chars.next();
    if first_char>=Some('1') && first_char<=Some('9') {
        let sub_s = &s[1..char_count];
        if sub_s.is_empty() != true {
            if result_vec[sub_s.len()] != -1 {
                count_one = result_vec[sub_s.len()];
            }
            else {
                let sub_result = num_decodings_ex(sub_s, result_vec);
                if sub_result>0 {
                    count_one = sub_result;
                }
            }
        }
        else {
            count_one+=1
        }

    }

    let mut count_two = 0;

    let second_char = chars.next();
    if second_char == None{
        return count_one;
    }
    else {
        if first_char==Some('1') {
            let sub_s = &s[2..char_count];
            if sub_s.is_empty() != true {
                if result_vec[sub_s.len()] != -1 {
                    count_two = result_vec[sub_s.len()];
                }
                else {
                    let sub_result = num_decodings_ex(sub_s, result_vec);
                    result_vec[sub_s.len()] = sub_result;
                    if sub_result>0 {
                        count_two = sub_result;
                    }
                }
            }
            else {
                count_two+=1
            }
        }
        else if first_char==Some('2') {
            if second_char>=Some('0') && second_char<=Some('6') {
                let sub_s = &s[2..char_count];
                if sub_s.is_empty() != true {
                    if result_vec[sub_s.len()] != -1 {
                        count_two = result_vec[sub_s.len()];
                    }
                    else {
                        let sub_result = num_decodings_ex(sub_s, result_vec);
                        result_vec[sub_s.len()] = sub_result;
                        if sub_result>0 {
                            count_two = sub_result;
                        }
                    }
                }
                else {
                    count_two+=1
                }
            }
        }

    }


    return count_one+count_two;
}


pub fn num_decodings(s: String) -> i32 {

    let mut resulr_vec: Vec<i32> = Vec::new();
    resulr_vec.resize(s.len()+1, -1);
    let result = num_decodings_ex(s.as_str(), &mut resulr_vec);

    return result;
        
}

/*解法思路：
    首先每个字符串开头都是可以分成单个字符和两个字符来分析的，单个字符如果可以解析成字母，那么就把后面的字符串
    作为参数，重复目前的解析；如果是两个字符的话，如果成功，也是把后面的字符作为参数，重复这套解析方法。
    这里的重点是会产生很多的重复操作，为了让重复的操作只执行一次，我们建立一个数组，以字符串的长度为长度，
    每个长度的字符串的分析结果都存在这里，如果出现重复的计算，就直接到这里查表。

*/
