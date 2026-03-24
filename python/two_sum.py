class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        num_set = {}

        for i in range(len(nums)):
            n = nums[i]
            if target - n in num_set:
                return [num_set[target - n], i]

            num_set[n] = i