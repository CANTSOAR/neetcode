#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {

        unordered_map<int, int> num_map;
        
        for (int i = 0; i < nums.size(); i++){
            int n = nums[i];
            if (num_map.count(target - n)){
                return {num_map[target - n], i};
            }
            num_map[n] = i;
        }
    }
};
