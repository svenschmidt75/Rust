using NUnit.Framework;

namespace Problem_2_7
{
    public class Node
    {
        public int Value { get; set; }
        public Node Next { get; set; }
    }

    public class LinkedList
    {
        public Node Head { get; set; }

        public static Node FindIntersection(LinkedList list1, LinkedList list2)
        {
            /* SS: Find the intersection between two linked lists, if there is one.
             * Example: Given two linked lists,
             * list1 = 1, 2, 3, 4, 5 and
             * list2 = 7, 9, 3, 4, 5
             * 1 - 2 - 3 - 4 - 5
             * 7 - 9-/
             *
             * we return node 3, as that is the intersection node.
             * Intersection is by reference, not value, so both linked lists share
             * the same tail!
             *
             * Other approaches:
             * 1. reverse both lists and compare nodes from start of reversed lists.
             *    Last node to be reference-equal is the intersection node.
             *
             * 2. Nested loop, runtime O(a * b)
             *
             * 3. Loop over list1 and insert nodes into hash.
             *    Loop over list2, and the first node of list2 in hash is
             *    intersection.
            */

            if (list1?.Head == null || list2?.Head == null)
            {
                return null;
            }
            
            Node tail1 = list1.Tail(); // O(a)
            Node tail2 = list2.Tail(); // O(b)
            if (tail1 != tail2)
            {
                // SS: both linked lists have to have the same tail if they intersect...
                return null;
            }

            int length1 = list1.Length(); // O(a)
            int length2 = list2.Length(); // O(b)

            Node longerList;
            Node shorterList;

            int offset = length1 - length2;
            if (offset < 0)
            {
                longerList = list2.Head;
                shorterList = list1.Head;
                offset *= -1;
            }
            else
            {
                longerList = list1.Head;
                shorterList = list2.Head;
            }
            
            // SS: skip offset nodes from the longer list
            // SS: O(|a-b|)
            int cnt = 0;
            while (longerList != null && cnt < offset)
            {
                cnt++;
                longerList = longerList.Next;
            }

            // SS: both lists, longerList and shorterList have the same length now
            // SS: O(max(a, b))
            while (longerList != null)
            {
                if (longerList == shorterList)
                {
                    // SS: we found an intersection
                    return longerList;
                }

                longerList = longerList.Next;
                shorterList = shorterList.Next;
            }

            // SS: lists do not intersect
            return null;
        }

        private Node Tail()
        {
            // SS: Return the tail node.
            // Note: We do not have a tail property. With it, we'd be done in O(1) instead
            // of O(n).
            // Use 2-pointer approach...
            Node prev = null;
            Node current = Head;
            while (current != null)
            {
                prev = current;
                current = current.Next;
            }

            return prev;
        }
        
        private int Length()
        {
            Node current = Head;
            int cnt = 0;
            while (current != null)
            {
                cnt++;
                current = current.Next;
            }

            return cnt;
        }
        
    }

    [TestFixture]
    public class Test
    {
        [Test]
        public void TestSameLength()
        {
            // Arrange

            // SS: 1 - 2 - 1 - 4 - 5
            var linkedList1 = new LinkedList();
            var node5 = new Node {Value = 5};
            var node4 = new Node {Value = 4, Next = node5};
            var node3 = new Node {Value = 3, Next = node4};
            var node2 = new Node {Value = 2, Next = node3};
            var node1 = new Node {Value = 1, Next = node2};
            linkedList1.Head = node1;

            // SS: 1 - 2 - 1 - 4 - 5
            var linkedList2 = new LinkedList();
            var node8 = new Node {Value = 8, Next = node3};
            var node9 = new Node {Value = 9, Next = node8};
            linkedList2.Head = node9;

            // Act
            var intersectionNode = LinkedList.FindIntersection(linkedList1, linkedList2);

            // Assert
            Assert.AreEqual(3, intersectionNode.Value);
        }
        
        [Test]
        public void TestList2Longer()
        {
            // Arrange

            // SS: 1 - 2 - 1 - 4 - 5
            var linkedList1 = new LinkedList();
            var node5 = new Node {Value = 5};
            var node4 = new Node {Value = 4, Next = node5};
            var node3 = new Node {Value = 3, Next = node4};
            var node2 = new Node {Value = 2, Next = node3};
            var node1 = new Node {Value = 1, Next = node2};
            linkedList1.Head = node1;

            // SS: 1 - 2 - 1 - 4 - 5
            var linkedList2 = new LinkedList();
            var node8 = new Node {Value = 8, Next = node3};
            var node9 = new Node {Value = 9, Next = node8};
            var node10 = new Node {Value = 10, Next = node9};
            linkedList2.Head = node10;

            // Act
            var intersectionNode = LinkedList.FindIntersection(linkedList1, linkedList2);

            // Assert
            Assert.AreEqual(3, intersectionNode.Value);
        }
        
        [Test]
        public void TestList1Longer()
        {
            // Arrange

            // SS: 1 - 2 - 1 - 4 - 5
            var linkedList1 = new LinkedList();
            var node5 = new Node {Value = 5};
            var node4 = new Node {Value = 4, Next = node5};
            var node3 = new Node {Value = 3, Next = node4};
            var node1 = new Node {Value = 1, Next = node3};
            linkedList1.Head = node1;

            // SS: 1 - 2 - 1 - 4 - 5
            var linkedList2 = new LinkedList();
            var node8 = new Node {Value = 8, Next = node3};
            var node9 = new Node {Value = 9, Next = node8};
            linkedList2.Head = node9;

            // Act
            var intersectionNode = LinkedList.FindIntersection(linkedList1, linkedList2);

            // Assert
            Assert.AreEqual(3, intersectionNode.Value);
        }

    }
}