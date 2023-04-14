class Solution {
    fun climbStairs(n: Int): Int {
        val map = hashMapOf<Int,Int>()
        map.put(1,1)
        map.put(2,2)
        return climbStairsMemoization(n, map)
    }

    fun climbStairsMemoization(n: Int, map: HashMap<Int,Int>): Int {
        if (map.containsKey(n)) {
            return map.getOrDefault(n,0);
        } else {
            val value = climbStairsMemoization(n - 1, map) + climbStairsMemoization(n - 2, map)
            map.put(n, value)
            return value;
        }
    }
}