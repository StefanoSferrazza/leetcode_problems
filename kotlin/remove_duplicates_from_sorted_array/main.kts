class Solution {
    fun removeDuplicates(nums: IntArray): Int {
        var currentIndex = 0
        var nextIndex = 1
        var current
        var next
        while (nextIndex <= nums.size) {
            current = nums[currentIndex]
            next = nums[nextIndex]
            if (current == next) {
                nextIndex++
            } else {
                nums[currentIndex+1]=next
                currentIndex ++
                nextIndex++
            }
        }
        return currentIndex+1
    }
}