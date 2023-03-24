import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

internal class SolutionTest{

    @Test
    fun test1(){
        val solution = Solution()

        val input = arrayOf("flower","flow","flight")
        val response = solution.longestCommonPrefix(input)

        assertEquals("fl", response)
    }

    @Test
    fun test2(){
        val solution = Solution()

        val input = arrayOf("dog","racecar","car")
        val response = solution.longestCommonPrefix(input)

        assertEquals("", response)
    }

    @Test
    fun test3(){
        val solution = Solution()

        val input = arrayOf("cigarettes","cigar","car")
        val response = solution.longestCommonPrefix(input)

        assertEquals("c", response)
    }

}