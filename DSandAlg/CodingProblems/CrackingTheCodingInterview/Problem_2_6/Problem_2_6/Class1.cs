#region

using System.Collections.Generic;
using NUnit.Framework;

#endregion

namespace Problem_2_6
{
    public class Node
    {
        public char Value { get; set; }
        public Node Next { get; set; }
    }

    public class LinkedList
    {
        public Node Head { get; set; }

        public bool IsPalindrome1()
        {
            var length = Length();
            var middleNode = length / 2;

            var stack = new Stack<char>();

            var current = Head;
            for (var i = 0; i < length; i++)
            {
                var value = current.Value;
                if (i < middleNode)
                {
                    stack.Push(value);
                }
                else
                {
                    if (i == middleNode)
                    {
                        if (length % 2 == 0)
                        {
                            // SS: middle node and even length
                            var p = stack.Pop();
                            if (p != value)
                            {
                                return false;
                            }
                        }
                    }
                    else
                    {
                        var p = stack.Pop();
                        if (p != value)
                        {
                            return false;
                        }
                    }
                }

                current = current.Next;
            }

            return true;
        }

        public bool IsPalindrome2()
        {
            var length = Length();
            var (isPalindrome, _) = IsPalindromeRecursive(length, 0, Head);
            return isPalindrome;
        }

        private (bool, Node) IsPalindromeRecursive(int length, int n, Node node)
        {
            if (n == length / 2)
            {
                if (n == length / 2 && length % 2 == 1)
                {
                    // SS: middle node and odd
                    return (true, node.Next);
                }

                // SS: even
                return (true, node);
            }

            var nextNode = node;
            var isValid = false;
            if (n < length / 2)
            {
                (isValid, nextNode) = IsPalindromeRecursive(length, n + 1, node.Next);
            }

            return (isValid && node.Value == nextNode.Value, nextNode.Next);
        }

        private int Length()
        {
            var current = Head;
            var cnt = 0;
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
        public void TestEven1()
        {
            // Arrange
            var linkedList = new LinkedList();
            var node4 = new Node {Value = 'a'};
            var node3 = new Node {Value = 'b', Next = node4};
            var node2 = new Node {Value = 'b', Next = node3};
            var node1 = new Node {Value = 'a', Next = node2};
            linkedList.Head = node1;

            // Act
            var isPalindrome = linkedList.IsPalindrome1();

            // Assert
            Assert.True(isPalindrome);
        }

        [Test]
        public void TestEven2()
        {
            // Arrange
            var linkedList = new LinkedList();
            var node4 = new Node {Value = 'a'};
            var node3 = new Node {Value = 'b', Next = node4};
            var node2 = new Node {Value = 'b', Next = node3};
            var node1 = new Node {Value = 'a', Next = node2};
            linkedList.Head = node1;

            // Act
            var isPalindrome = linkedList.IsPalindrome1();

            // Assert
            Assert.True(isPalindrome);
        }

        [Test]
        public void TestNotAPalindrome1()
        {
            // Arrange
            var linkedList = new LinkedList();
            var node4 = new Node {Value = 'o'};
            var node3 = new Node {Value = 'b', Next = node4};
            var node2 = new Node {Value = 'b', Next = node3};
            var node1 = new Node {Value = 'a', Next = node2};
            linkedList.Head = node1;

            // Act
            var isPalindrome = linkedList.IsPalindrome1();

            // Assert
            Assert.False(isPalindrome);
        }

        [Test]
        public void TestNotAPalindrome2()
        {
            // Arrange
            var linkedList = new LinkedList();
            var node4 = new Node {Value = 'o'};
            var node3 = new Node {Value = 'b', Next = node4};
            var node2 = new Node {Value = 'b', Next = node3};
            var node1 = new Node {Value = 'a', Next = node2};
            linkedList.Head = node1;

            // Act
            var isPalindrome = linkedList.IsPalindrome2();

            // Assert
            Assert.False(isPalindrome);
        }

        [Test]
        public void TestOdd1()
        {
            // Arrange
            var linkedList = new LinkedList();
            var node5 = new Node {Value = 'm'};
            var node4 = new Node {Value = 'a', Next = node5};
            var node3 = new Node {Value = 'd', Next = node4};
            var node2 = new Node {Value = 'a', Next = node3};
            var node1 = new Node {Value = 'm', Next = node2};
            linkedList.Head = node1;

            // Act
            var isPalindrome = linkedList.IsPalindrome1();

            // Assert
            Assert.True(isPalindrome);
        }

        [Test]
        public void TestOdd2()
        {
            // Arrange
            var linkedList = new LinkedList();
            var node5 = new Node {Value = 'm'};
            var node4 = new Node {Value = 'a', Next = node5};
            var node3 = new Node {Value = 'd', Next = node4};
            var node2 = new Node {Value = 'a', Next = node3};
            var node1 = new Node {Value = 'm', Next = node2};
            linkedList.Head = node1;

            // Act
            var isPalindrome = linkedList.IsPalindrome2();

            // Assert
            Assert.True(isPalindrome);
        }
    }
}