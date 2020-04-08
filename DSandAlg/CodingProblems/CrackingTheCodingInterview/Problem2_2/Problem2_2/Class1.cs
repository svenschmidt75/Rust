using System.Collections.Generic;
using NUnit.Framework;

namespace Problem2_2
{
    public class Node
    {
        public int Value { get; set; }
        public Node Next { get; set; }
    }

    public class LinkedList
    {
        public Node Head { get; set; }
        public Node Tail { get; set; }

        public IEnumerable<int> ReturnKthToLastElement(int k)
        {
            // SS: remove duplicate nodes from a linked list
            var items = new List<int>();

            if (Head == null) return items;

            var current = Head;
            int cnt = 1;
            while (current != null && cnt < k)
            {
                current = current.Next;
                cnt++;
            }

            while (current != null)
            {
                items.Add(current.Value);
                current = current.Next;
            }

            return items;
        }

    }

    [TestFixture]
    public class Test
    {
        [Test]
        public void Test1()
        {
            // Arrange
            var linkedList = new LinkedList();

            // SS: 1 - 2 - 1 - 4 - 5 - 6
            var node6 = new Node {Value = 6};
            var node5 = new Node {Value = 5, Next = node6};
            var node4 = new Node {Value = 4, Next = node5};
            var node3 = new Node {Value = 1, Next = node4};
            var node2 = new Node {Value = 2, Next = node3};
            var node1 = new Node {Value = 1, Next = node2};

            linkedList.Head = node1;
            linkedList.Tail = node6;

            // Act
            var items = linkedList.ReturnKthToLastElement(3);

            // Assert
            CollectionAssert.AreEqual(new[]{1, 4, 5, 6}, items);
        }

    }
}