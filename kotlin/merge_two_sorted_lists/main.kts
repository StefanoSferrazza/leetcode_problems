/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.
 * class ListNode(var `val`: Int) {
 *     var next: ListNode? = null
 * }
 */
class Solution {
    fun mergeTwoLists(list1: ListNode?, list2: ListNode?): ListNode? {
        when {
            list1 == null && list2 == null -> return null
            list1 == null -> return list2
            list2 == null -> return list1
            else -> {
                if (list1.`val` < list2.`val`) {
                    val next = mergeTwoLists(list1.next, list2)
                    list1.next = next
                    return list1
                } else {
                    val next = mergeTwoLists(list1, list2.next)
                    list2.next = next
                    return list2
                }
            }
        }
    }
}

class ListNode(var `val`: Int) {
    var next: ListNode? = null
}