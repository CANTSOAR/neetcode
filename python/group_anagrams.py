class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        words = {}

        for s in strs:
            t = str(sorted(s))
            if t in words:
                words[t].append(s)
                continue
            
            words[t] = [s]

        return list(words.values())