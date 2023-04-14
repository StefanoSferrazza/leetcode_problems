class Solution {
    fun plusOne(digits: IntArray): IntArray {
        return incrementByOne(digits, digits.size-1)
    }

    fun incrementByOne(digits: IntArray, pos: Int): IntArray{
        if (pos<0){
            val newArray: IntArray = IntArray(digits.size+1)
            for (i in (digits.size-1)..0){
                newArray[i+1]=digits[i]
            }
            newArray[0]=1
            return newArray
        }
        else {
            when (digits[pos]){
                9 -> {
                    digits[pos] = 0
                    return incrementByOne(digits, pos-1)
                }
                else -> {
                    digits[pos] += 1
                    return digits
                }
            }
        }
    }
}