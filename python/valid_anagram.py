class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) == len(t):
            letters = {}

            for l in s:
                if l in letters:
                    letters[l] += 1
                else:
                    letters[l] = 1

            for l in t:
                if l not in letters:
                    return False

                letters[l] -= 1

            for v in letters.values():
                if v:
                    return False

            return True

        return False