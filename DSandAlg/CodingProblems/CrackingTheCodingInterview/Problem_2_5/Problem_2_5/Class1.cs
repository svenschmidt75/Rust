using System;
using NUnit.Framework;

namespace Problem2_5
{
    public class Node
    {
        public int Value { get; set; }
        public Node Next { get; set; }
    }

    public class LinkedList
    {
        public Node Head { get; set; }

        public static LinkedList SumList(LinkedList list1, LinkedList list2)
        {
            int left = ListToNumber(list1);
            int right = ListToNumber(list2);
            int add = left + right;
            
            var addList = new LinkedList();
            var current = addList.Head;
            int digits = (int) Math.Log10(add);
            int cnt = digits;

            while (cnt >= 0)
            {
                int power = (int) Math.Pow(10, cnt);
                int d = add / power;
                var node = new Node{Value = d, Next = current};
                current = node;
                add -= d * power;
                cnt--;
            }

            addList.Head = current;
            
            return addList;
        }

        public static LinkedList SumListReverse(LinkedList list1, LinkedList list2)
        {
            int left = ListToNumberReverse(list1);
            int right = ListToNumberReverse(list2);
            int add = left + right;
            
            var addList = new LinkedList();
            var current = addList.Head;
            Node  head = null;
            int digits = (int) Math.Log10(add);
            int cnt = digits;

            while (cnt >= 0)
            {
                int power = (int) Math.Pow(10, cnt);
                int d = add / power;
                var node = new Node{Value = d};
                if (current == null)
                {
                    head = node;
                    current = node;
                }
                else
                {
                    current.Next = node;
                    current = node;
                }
                add -= d * power;
                cnt--;
            }

            addList.Head = head;
            
            return addList;
        }

        public static int ListToNumber(LinkedList list)
        {
            // SS: Turn ll (7 - 1 - 6) into number 617
            int number = 0;
            if (list.Head == null)
            {
                return number;
            }

            var node = list.Head;
            int cnt = 0;
            while (node != null)
            {
                int value = node.Value;
                int power = (int) Math.Pow(10, cnt);
                number += value * power;

                cnt++;
                node = node.Next;
            }

            return number;
        }
        
        public static int ListToNumberReverse(LinkedList list)
        {
            // SS: Turn ll (6 - 1 - 7) into number 617
            int number = 0;
            if (list.Head == null)
            {
                return number;
            }

            var node = list.Head;
            while (node != null)
            {
                int value = node.Value;
                number = number * 10 + value;
                node = node.Next;
            }

            return number;
        }

    }

    
    
    
    [TestFixture]
    public class Test
    {
        [Test]
        public void TestListToNumber()
        {
            // Arrange
            var linkedList = new LinkedList();

            // SS: 7 - 1 - 6
            var node3 = new Node {Value = 6};
            var node2 = new Node {Value = 1, Next = node3};
            var node1 = new Node {Value = 7, Next = node2};

            linkedList.Head = node1;

            // Act
            var number = LinkedList.ListToNumber(linkedList);
            
            // Assert
            Assert.AreEqual(617, number);
        }

        [Test]
        public void TestListToNumberReverse()
        {
            // Arrange
            var linkedList = new LinkedList();

            // SS: 6 - 1 - 7
            var node3 = new Node {Value = 7};
            var node2 = new Node {Value = 1, Next = node3};
            var node1 = new Node {Value = 6, Next = node2};

            linkedList.Head = node1;

            // Act
            var number = LinkedList.ListToNumberReverse(linkedList);
            
            // Assert
            Assert.AreEqual(617, number);
        }

        [Test]
        public void TestAdd()
        {
            // Arrange
            var ll1 = new LinkedList();
            var node3 = new Node {Value = 6};
            var node2 = new Node {Value = 1, Next = node3};
            var node1 = new Node {Value = 7, Next = node2};
            ll1.Head = node1;

            var ll2 = new LinkedList();
            node3 = new Node {Value = 2};
            node2 = new Node {Value = 9, Next = node3};
            node1 = new Node {Value = 5, Next = node2};
            ll2.Head = node1;

            // Act
            var addList = LinkedList.SumList(ll1, ll2);
            
            // Assert
            Assert.AreEqual(2, addList.Head.Value);
            Assert.AreEqual(1, addList.Head.Next.Value);
            Assert.AreEqual(9, addList.Head.Next.Next.Value);
            Assert.IsNull(addList.Head.Next.Next.Next);
        }

        [Test]
        public void TestAddReverse()
        {
            // Arrange
            var ll1 = new LinkedList();
            var node3 = new Node {Value = 7};
            var node2 = new Node {Value = 1, Next = node3};
            var node1 = new Node {Value = 6, Next = node2};
            ll1.Head = node1;

            var ll2 = new LinkedList();
            node3 = new Node {Value = 5};
            node2 = new Node {Value = 9, Next = node3};
            node1 = new Node {Value = 2, Next = node2};
            ll2.Head = node1;

            // Act
            var addList = LinkedList.SumListReverse(ll1, ll2);
            
            // Assert
            Assert.AreEqual(9, addList.Head.Value);
            Assert.AreEqual(1, addList.Head.Next.Value);
            Assert.AreEqual(2, addList.Head.Next.Next.Value);
            Assert.IsNull(addList.Head.Next.Next.Next);
        }

    }
}