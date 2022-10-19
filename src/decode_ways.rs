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


pub fn num_decodings_ex(s:&str) -> i32 {

    let char_count = s.chars().count();

    let mut count = 0;
    let mut char_buffer = '0';
    let mut use_char_buffer = false;

    for (i, c) in s.chars().enumerate() {
        if i==0 && c=='0' {
            return 0;
        }

        let mut sub_str_decoded = false;

        if c>='1' && c<='9' {
            let sub_s = &s[i+1..char_count];
            let sub_result = num_decodings_ex(sub_s);
            if sub_result>0 {
                count = sub_result + 1;
            }
            sub_str_decoded  = true;

        }

        if use_char_buffer==true {
            if char_buffer=='1' {
                if sub_str_decoded == false {
                    let sub_s = &s[i+1..char_count];
                    let sub_result = num_decodings_ex(sub_s);
                    if sub_result >0 {
                        count = sub_result + 1;
                    }
                    sub_str_decoded = true;
                }
            }
            else if char_buffer=='2' {
                if c>='0' || c<='6' {
                    if sub_str_decoded == false {
                        let sub_s = &s[i+1..char_count];
                        let sub_result = num_decodings_ex(sub_s);
                        if sub_result >0 {
                            count = sub_result + 1;
                        }
                        sub_str_decoded = true;
                    }
                }
            }

            if sub_str_decoded == true {
                break;
            }
        }
        else {

            if c=='1' || c=='2'{
                char_buffer = c;
                use_char_buffer = true;
            }
            else {
                break;
            }
        }
    }

    return count;
}


pub fn num_decodings(s: String) -> i32 {

    let result = num_decodings_ex(s.as_str());

    return result;
        
}