#region

using System;
using NUnit.Framework;

#endregion

namespace Problem50
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
        public int Length { get; private set; }

        public int Peek()
        {
            if (Head == null)
            {
                throw new InvalidOperationException();
            }

            return Head.Value;
        }

        public int RemoveFront()
        {
            if (Head == null)
            {
                throw new InvalidOperationException();
            }

            var node = Head;
            Head = Head.Next;

            if (Head == null)
            {
                Tail = null;
            }

            Length--;

            return node.Value;
        }

        public bool IsEmpty()
        {
            return Head == null;
        }

        public void InsertSortedDescending(int value)
        {
            var node = new Node {Value = value};

            if (Head == null)
            {
                Head = node;
                Tail = Head;
            }
            else
            {
                Node prev = null;
                var current = Head;
                while (current != null && current.Value > value)
                {
                    prev = current;
                    current = current.Next;
                }

                if (prev == null)
                {
                    node.Next = Head;
                    Head = node;
                }
                else
                {
                    prev.Next = node;
                    node.Next = current;
                }
            }

            Length++;
        }

        public void InsertSortedAscending(int value)
        {
            var node = new Node {Value = value};

            if (Head == null)
            {
                Head = node;
                Tail = Head;
            }
            else
            {
                Node prev = null;
                var current = Head;
                while (current != null && current.Value < value)
                {
                    prev = current;
                    current = current.Next;
                }

                if (prev == null)
                {
                    node.Next = Head;
                    Head = node;
                }
                else
                {
                    prev.Next = node;
                    node.Next = current;
                }
            }

            Length++;
        }
    }

    public class MedianInfiniteSeriesOfIntegers
    {
        private readonly LinkedList _ascendingList = new LinkedList();
        private readonly LinkedList _descendingList = new LinkedList();

        public float Median
        {
            get
            {
                var deltaLength = _descendingList.Length - _ascendingList.Length;
                if (deltaLength == 0)
                {
                    var median = _descendingList.Peek();
                    median += _ascendingList.Peek();
                    return median / 2.0f;
                }

                if (deltaLength == 1)
                {
                    var median = _descendingList.Peek();
                    return median;
                }

                if (deltaLength == -1)
                {
                    var median = _ascendingList.Peek();
                    return median;
                }

                throw new InvalidOperationException();
            }
        }

        public void Insert(int value)
        {
            if (_descendingList.IsEmpty())
            {
                _descendingList.InsertSortedDescending(value);
            }
            else if (_ascendingList.IsEmpty())
            {
                _ascendingList.InsertSortedAscending(value);
            }
            else
            {
                var descTop = _descendingList.Peek();
                if (descTop > value)
                {
                    _descendingList.InsertSortedDescending(value);
                }
                else
                {
                    _ascendingList.InsertSortedAscending(value);
                }

                // SS: balance both lists
                var deltaLength = _descendingList.Length - _ascendingList.Length;
                if (deltaLength >= 2)
                {
                    var head = _descendingList.RemoveFront();
                    _ascendingList.InsertSortedAscending(head);
                }
                else if (deltaLength <= -2)
                {
                    var head = _ascendingList.RemoveFront();
                    _descendingList.InsertSortedDescending(head);
                }
            }
        }
    }

    [TestFixture]
    public class Tests
    {
        [Test]
        public void TestInsertSortedAscendingBeforeHead()
        {
            // Arrange
            var linkedList = new LinkedList();
            linkedList.InsertSortedAscending(7);

            // Act
            linkedList.InsertSortedAscending(5);

            // Assert
            Assert.AreEqual(5, linkedList.Head.Value);
            Assert.AreEqual(7, linkedList.Head.Next.Value);
        }

        [Test]
        public void TestInsertSortedAscendingBetweenHeadAndTail()
        {
            // Arrange
            var linkedList = new LinkedList();
            linkedList.InsertSortedAscending(8);
            linkedList.InsertSortedAscending(12);

            // Act
            linkedList.InsertSortedAscending(9);

            // Assert
            Assert.AreEqual(8, linkedList.Head.Value);
            Assert.AreEqual(9, linkedList.Head.Next.Value);
            Assert.AreEqual(12, linkedList.Head.Next.Next.Value);
        }

        [Test]
        public void TestInsertSortedAscendingHead()
        {
            // Arrange
            var linkedList = new LinkedList();

            // Act
            linkedList.InsertSortedAscending(7);

            // Assert
            Assert.AreEqual(7, linkedList.Head.Value);
        }

        [Test]
        public void TestInsertSortedAscendingTail()
        {
            // Arrange
            var linkedList = new LinkedList();
            linkedList.InsertSortedAscending(5);

            // Act
            linkedList.InsertSortedAscending(7);

            // Assert
            Assert.AreEqual(5, linkedList.Head.Value);
            Assert.AreEqual(7, linkedList.Head.Next.Value);
        }

        [Test]
        public void TestInsertSortedDescendingBeforeHead()
        {
            // Arrange
            var linkedList = new LinkedList();
            linkedList.InsertSortedDescending(7);

            // Act
            linkedList.InsertSortedDescending(9);

            // Assert
            Assert.AreEqual(9, linkedList.Head.Value);
            Assert.AreEqual(7, linkedList.Head.Next.Value);
        }

        [Test]
        public void TestInsertSortedDescendingBetweenHeadAndTail()
        {
            // Arrange
            var linkedList = new LinkedList();
            linkedList.InsertSortedDescending(7);
            linkedList.InsertSortedDescending(3);

            // Act
            linkedList.InsertSortedDescending(5);

            // Assert
            Assert.AreEqual(7, linkedList.Head.Value);
            Assert.AreEqual(5, linkedList.Head.Next.Value);
            Assert.AreEqual(3, linkedList.Head.Next.Next.Value);
        }

        [Test]
        public void TestInsertSortedDescendingHead()
        {
            // Arrange
            var linkedList = new LinkedList();

            // Act
            linkedList.InsertSortedDescending(7);

            // Assert
            Assert.AreEqual(7, linkedList.Head.Value);
        }

        [Test]
        public void TestInsertSortedDescendingTail()
        {
            // Arrange
            var linkedList = new LinkedList();
            linkedList.InsertSortedDescending(7);

            // Act
            linkedList.InsertSortedDescending(5);

            // Assert
            Assert.AreEqual(7, linkedList.Head.Value);
            Assert.AreEqual(5, linkedList.Head.Next.Value);
        }

        [Test]
        public void TestMedian1()
        {
            // Arrange
            var medianOfSeries = new MedianInfiniteSeriesOfIntegers();

            // Act - Assert
            medianOfSeries.Insert(7);
            Assert.AreEqual(7, medianOfSeries.Median);

            medianOfSeries.Insert(8);
            Assert.That(medianOfSeries.Median, Is.EqualTo(7.5));

            medianOfSeries.Insert(9);
            Assert.That(medianOfSeries.Median, Is.EqualTo(8));

            medianOfSeries.Insert(2);
            Assert.AreEqual(7.5, medianOfSeries.Median);

            medianOfSeries.Insert(3);
            Assert.That(medianOfSeries.Median, Is.EqualTo(7));

            medianOfSeries.Insert(12);
            Assert.That(medianOfSeries.Median, Is.EqualTo(7.5));

            medianOfSeries.Insert(1);
            Assert.That(medianOfSeries.Median, Is.EqualTo(7));

            medianOfSeries.Insert(4);
            Assert.That(medianOfSeries.Median, Is.EqualTo(5.5));
        }
    }
}