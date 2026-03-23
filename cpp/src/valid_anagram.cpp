#include <string>
#include <unordered_map>

using namespace std;

class Solution {
public:
    bool isAnagram(string s, string t) {
        unordered_map<char, int> letters;
 
        if (s.length() == t.length()){
            for (char l : s){
                if (letters.count(l)) {
                    letters[l]++;
                    continue;
                }
                letters[l] = 1;
            }

            for (char l : t){
                if (!letters.count(l)){
                    return false;
                }

                letters[l] -= 1;
                if (letters[l] < 0) {
                    return false;
                }
            }
            
            return true;
        }

        return false;
    }
};