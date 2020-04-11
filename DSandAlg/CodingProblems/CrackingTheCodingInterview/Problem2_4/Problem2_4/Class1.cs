using NUnit.Framework;

namespace Problem2_4
{
    public class Node
    {
        public int Value { get; set; }
        public Node Next { get; set; }
    }

    public class LinkedList
    {
        public Node Head { get; set; }

        public void Partition(int n)
        {
            /* SS: partition a linked list such that all elements < n appear
             * before any elements >= n.
            */
            if (Head == null)
            {
                return;
            }

            Node ltStart = null;
            Node lt = null;
            Node geStart = null;
            Node ge = null;

            Node currentNode = Head;
            while (currentNode != null)
            {
                var value = currentNode.Value;
                if (value < n)
                {
                    if (ltStart == null)
                    {
                        ltStart = currentNode;
                        lt = ltStart;
                    }
                    else
                    {
                        lt.Next = currentNode;
                        lt = currentNode;
                    }
                }
                else
                {
                    if (geStart == null)
                    {
                        geStart = currentNode;
                        ge = geStart;
                    }
                    else
                    {
                        ge.Next = currentNode;
                        ge = currentNode;
                    }
                }

                currentNode = currentNode.Next;
            }

            // SS: combine
            lt.Next = geStart;
            ge.Next = null;
            Head = ltStart;
        }
        
        
    }

    [TestFixture]
    public class Test
    {
        [Test]
        public void TestSameLength()
        {
            // Arrange

            // SS: 1 - 2 - 3 - 4 - 5
            var linkedList = new LinkedList();
            var node5 = new Node {Value = 1};
            var node4 = new Node {Value = 2, Next = node5};
            var node3 = new Node {Value = 3, Next = node4};
            var node2 = new Node {Value = 4, Next = node3};
            var node1 = new Node {Value = 5, Next = node2};
            linkedList.Head = node1;

            // Act
            linkedList.Partition(4);

            // Assert
            Node node = linkedList.Head;
            for (int i = 0; i < 5; i++)
            {
                if (i < 3)
                {
                    Assert.True(node.Value < 4);
                }
                else
                {
                    Assert.True(node.Value >= 4);
                }

                node = node.Next;
            }
        }
        
    }
}