class Solution {
    fun isPalindrome(x: Int): Boolean {
        if (x < 0 || x !=0 && x%10 == 0){
            return false
        }

        var y = x
        var mirror = 0
        while (y > mirror){
            mirror = mirror * 10 + y % 10
            y /= 10
        }

        return y==mirror || y==mirror/10
    }
}
