class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        l1 = len(haystack)
        l2 = len(needle)
        
        for i in range(len(haystack)):
            j = i + l2

            if l1 < j:
                break
            
            if haystack[i: j] != needle:
                continue

            return i

        return -1


import unittest

class Tests(unittest.TestCase):
        
    def test_0(self):
        input_1 = "sadbutsad"
        input_2 = "sad"
        
        self.assertEqual(0, Solution().strStr(input_1, input_2))

    def test_1(self):
        input_1 = "leetcode"
        input_2 = "leeto"
        
        self.assertEqual(-1, Solution().strStr(input_1, input_2))


if __name__ == '__main__':
    unittest.main()
