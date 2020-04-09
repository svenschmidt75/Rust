using System.Collections.Generic;
using NUnit.Framework;

namespace Problem2_1
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

        public void RemoveDuplicates1()
        {
            // SS: remove duplicate nodes from a linked list
            // SS: If a temp. buffer is not allowed, we can, for each node,
            // search to the end and remove duplicate nodes along the way.
            // This approach has runtime O(n^2)
            
            if (Head == null) return;

            var values = new HashSet<int>();
            Node prev = null;
            var current = Head;

            while (current != null)
            {
                var value = current.Value;
                if (values.Contains(value))
                {
                    // SS: is a duplicate, remove node
                    Remove(prev);
                    current = prev;
                    
                    // TODO SS: need to update Tail
                }
                else
                {
                    values.Add(current.Value);
                }

                prev = current;
                current = current.Next;
            }
        }

        private void Remove(Node node)
        {
            // SS: delete node node.next, i.e.
            // A -> B -> C
            // delete node B
            var next = node.Next.Next;
            node.Next = next;
        }
    }

    [TestFixture]
    public class Test
    {
        [Test]
        public void Test11()
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
            linkedList.RemoveDuplicates1();

            // Assert
            Assert.AreEqual(4, node2.Next.Value);
        }

        [Test]
        public void Test12()
        {
            // Arrange
            var linkedList = new LinkedList();

            // SS: 1 - 2 - 1 - 4 - 5 - 6
            var node6 = new Node {Value = 6};
            var node5 = new Node {Value = 5, Next = node6};
            var node4 = new Node {Value = 4, Next = node5};
            var node3 = new Node {Value = 6, Next = node4};
            var node2 = new Node {Value = 2, Next = node3};
            var node1 = new Node {Value = 1, Next = node2};

            linkedList.Head = node1;
            linkedList.Tail = node6;

            // Act
            linkedList.RemoveDuplicates1();

            // Assert
            Assert.IsNull(node5.Next);
        }
        
    }
}