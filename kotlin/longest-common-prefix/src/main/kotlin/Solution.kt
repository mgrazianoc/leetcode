import java.lang.Integer.min

class Solution {
    fun longestCommonPrefix(strs: Array<String>): String {
        var acc = ""
        var count = 0

        // O(kn)
        while (true) {
            try {
                val elementIndex = count % strs.size
                acc += strs[elementIndex][count].toString()
                count += 1
            }catch (e: Exception){
                break
            }
        }

        val responses = mutableListOf<String>()
        // O(kn)
        for (str in strs) {
            var response = ""

            for (i in 0 until min(str.length, acc.length)) {
                if (str[i] == acc[i]) {
                    response += str[i]
                    continue
                }
                break
            }
            responses.add(response)
        }

        return responses.minBy { it.length }
    }
}