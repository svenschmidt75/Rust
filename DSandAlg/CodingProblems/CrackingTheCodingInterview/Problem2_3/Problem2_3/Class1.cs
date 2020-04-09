using NUnit.Framework;

namespace Problem2_3
{
    public class Node
    {
        public int Value { get; set; }
        public Node Next { get; set; }
    }

    public class LinkedList
    {
        public Node Head { get; set; }

        public static void DeleteNode(Node node)
        {
            // SS: Delete the node in the linked list, not head, not tail.
            // No access to the linked list is provided, all we have is the node...
            var next = node.Next;
            node.Next = next.Next;
            node.Value = next.Value;
        }
        
    }

    [TestFixture]
    public class Test
    {
        [Test]
        public void TestDelete()
        {
            // Arrange

            // SS: 1 - 2 - 1 - 4 - 5
            var linkedList = new LinkedList();
            var node5 = new Node {Value = 5};
            var node4 = new Node {Value = 4, Next = node5};
            var node3 = new Node {Value = 3, Next = node4};
            var node2 = new Node {Value = 2, Next = node3};
            var node1 = new Node {Value = 1, Next = node2};
            linkedList.Head = node1;

            // Act
            LinkedList.DeleteNode(node3);

            // Assert
            Assert.AreEqual(2, linkedList.Head.Next.Value);
            Assert.AreEqual(4, linkedList.Head.Next.Next.Value);
            Assert.AreEqual(5, linkedList.Head.Next.Next.Next.Value);
        }
        
    }
}