class Solution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        var a: Int = 0
        var b: Int = 0
        var hit = false
        val numsIndexMap = mutableMapOf<Int, Int>()

        for (index in nums.indices){
            val num = nums[index]
            val complementary = target - num

            numsIndexMap[complementary]?.let {
                a = index
                b = it
                hit = true
            }

            if (hit){
                break
            }

            numsIndexMap[num] = index
        }

        return intArrayOf(a, b)
    }
}