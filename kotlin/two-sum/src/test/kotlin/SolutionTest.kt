import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test


internal class SolutionTest {

    @Test
    fun test1(){
        val solution = Solution()
        val nums = intArrayOf(2, 7, 11, 15)
        val target = 9

        val response = solution.twoSum(nums, target)

        assertEquals(response.sort(), intArrayOf(0, 1).sort())
    }

    @Test
    fun test2(){
        val solution = Solution()
        val nums = intArrayOf(3, 2, 4)
        val target = 6

        val response = solution.twoSum(nums, target)

        assertEquals(response.sort(), intArrayOf(1, 2).sort())
    }

    @Test
    fun test3(){
        val solution = Solution()
        val nums = intArrayOf(3, 3)
        val target = 6

        val response = solution.twoSum(nums, target)

        assertEquals(response.sort(), intArrayOf(0, 1).sort())
    }

}