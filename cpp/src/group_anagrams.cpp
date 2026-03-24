#include <vector>
#include <string>
#include <unordered_map>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        unordered_map<string, vector<string>> words;

        for (const string &s : strs){
            string t = s;
            sort(t.begin(), t.end());
            words[t].push_back(s);
        }

        vector<vector<string>> result;
        for (const auto &pair : words) {
            result.push_back(pair.second);
        }

        return result;
    }
};
