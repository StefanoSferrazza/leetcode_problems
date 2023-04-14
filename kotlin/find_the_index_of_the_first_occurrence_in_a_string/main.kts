class Solution {
    fun strStr(haystack: String, needle: String): Int {
        var currentIndex = 0
        while (currentIndex < haystack.length) {
            if (haystack[currentIndex] == needle[0]) {
                var current_temp_index = currentIndex
                var equal = true
                for (c in needle) {
                    if (current_temp_index >= haystack.length || c != haystack[current_temp_index]) {
                        equal = false
                        break
                    }
                    current_temp_index++
                }
                if (equal) {
                    return currentIndex
                }
            }
            currentIndex++
        }
        return -1
    }
}