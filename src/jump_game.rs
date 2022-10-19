/*
You are given an integer array nums. You are initially positioned at the array's first index, 
and each element in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.
 

Example 1:
Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.

Example 2:
Input: nums = [3,2,1,0,4]
Output: false
Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
 

Constraints:
    1 <= nums.length <= 104
    0 <= nums[i] <= 105
 */

struct ZeroPod
{
    pos:usize,
    count:i32
}

fn can_jump_over_zero(pos:&ZeroPod, pre_pos:&ZeroPod, nums: &Vec<i32>) -> bool {

    let end_index:i32 =  pre_pos.pos as i32 + pre_pos.count; 

    for i in (end_index as usize..pos.pos).rev() {
        let max_pos:i32 = i as i32 + nums[i]  - pos.count;
        if max_pos >= pos.pos as i32 {
            return true;
        }
    }

    return false;

}



pub fn can_jump(nums: Vec<i32>) -> bool {

    if nums.len() == 0 || nums.len() == 1  {
        return true;
    }

    if nums[0] == 0 {
        return false;
    }

    let mut zero_vec: Vec<ZeroPod> = Vec::new();
    let mut zero_count = 0;
    for i in 1..nums.len() {
        let pos_value = nums[i];
        let pre_pos_value = nums[i-1];
        if pos_value==0 && i != nums.len()-1 {
            if pre_pos_value==0 {
                zero_vec[zero_count-1].count+=1;
            }
            else{
                let zero_info:ZeroPod = ZeroPod{pos:i, count:1};
                zero_vec.push(zero_info);
                zero_count += 1;
            }

        }
    }

    if zero_count == 0 {
        return true;
    }

    for i in 0..zero_count {

        let pos = &zero_vec[i];
        let pre_pos:&ZeroPod = &ZeroPod{pos:0, count:0};

        if can_jump_over_zero(pos, pre_pos, &nums) == false {
            return false;
        }
    }
    
    return true;
        
}


/*解法思路：
    说白了，这个游戏就是不要踩到0上，踩到0上，就肯定无法走到数组的最终
    所以：遍历出来所有的0，如果是连续的0，算是一个位置，但是要增加一个数量，方便后面计算
         然后从每个0的位置，往回找，看看有没有能跳过这个0的可能，有的话，这个0就不是障碍，
         如果没有办法跳过这个0，那铁定遍历不到终点，返回假
         当把所有的0都看了一遍之后，都可以跳过，那就一定能走到终点，返回真

*/

