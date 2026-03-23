#include <vector>
#include <unordered_set>

using namespace std;

class Solution {
public:
    bool containsDuplicate(vector<int>& nums) {
        unordered_set<int> nums_set;
        for (int n : nums) {
            if (!nums_set.insert(n).second) {
                return true;
            }
        }
        return false;
    }
};
