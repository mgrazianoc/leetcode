import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

internal class SolutionTest{

    @Test
    fun test1(){
        val solution = Solution()
        val input = 121

        assertTrue(solution.isPalindrome(input))
    }

    @Test
    fun test2(){
        val solution = Solution()
        val input = -121

        assertFalse(solution.isPalindrome(input))
    }

    @Test
    fun test3(){
        val solution = Solution()
        val input = 1221

        assertTrue(solution.isPalindrome(input))
    }

}