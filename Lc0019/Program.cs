using System;
using static System.Console;

namespace Lc0019
{
    class Program
    {
        static void Main(string[] args)
        {
            var result = new Solution().RemoveNthFromEnd(new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5))))), 2);
            WriteLine(result);
        }
    }

    //  * Definition for singly-linked list.
    public class ListNode
    {
        public int val;
        public ListNode next;
        public ListNode(int val = 0, ListNode next = null)
        {
            this.val = val;
            this.next = next;
        }
    }

    public class Solution
    {
        public ListNode RemoveNthFromEnd(ListNode head, int n)
        {
            var dummy = new ListNode(next: head);
            var slow = dummy;
            var fast = head;
            var c = 0;

            while (fast != null)
            {
                c += 1;
                fast = fast.next;
                if (c - n > 0) slow = slow.next;
            }

            slow.next = slow.next.next;
            return dummy.next;
        }
    }
}
