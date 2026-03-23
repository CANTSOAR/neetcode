# class Solution:
#     def hasDuplicate(self, nums: List[int]) -> bool:
#         nums_dict = {}
#         for n in nums:
#             if n in nums_dict:
#                 return True
            
#             nums_dict[n] = 0

#         return False

# set better than dict because it doesnt store values for no reason
class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        nums_set = set()
        for n in nums:
            if n in nums_set:
                return True
            
            nums_set.add(n)

        return False