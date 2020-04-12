#region

using System;
using NUnit.Framework;

#endregion

namespace Problem_3_2
{
    public class StackWithMin
    {
        private const int BufferSize = 1000;
        private readonly (int item, int minIndex)[] _buffer = new (int, int)[BufferSize];
        private int _minIndex = -1;
        private int _position;

        public void Push(int item)
        {
            if (_position == BufferSize - 1)
            {
                throw new StackOverflowException();
            }

            /* O(1) Min method 1:
             * Given a stack with N elements. We assume that on average, the min element is popped off
             * after removing half the elements in the stack, i.e. after N/2 elements have been poppod
             * off, then after N/4, etc.
             * When N/2 elements are removed and we need to update the min element, we make a linear
             * search through the buffer of the remaining N/2 elements, etc.
             * We find that after popping off every element, we have cost
             * N/2 + N/4 + N/8 + N/16 + ... to update the min element.
             * In total, 2N, so on average, the cost to update the min element after popping off an item
             * is O(2N / N) => O(1) amortized time.
             */

            // SS: Method 2: Push the item together with an index pointing to the current min element
            if (_minIndex == -1)
            {
                _minIndex = _position;
                _buffer[_position++] = (item, _minIndex);
            }
            else
            {
                _buffer[_position++] = (item, _minIndex);
                if (item < _buffer[_minIndex].item)
                {
                    _minIndex = _position - 1;
                }
            }
        }

        public int Pop()
        {
            if (IsEmpty())
            {
                throw new InvalidOperationException();
            }

            _position--;
            (var item, var minIndex) = _buffer[_position];

            if (_minIndex == _position)
            {
                // SS: update min index
                _minIndex = minIndex;
            }

            return item;
        }

        public int Min()
        {
            if (_minIndex == -1)
            {
                throw new InvalidOperationException();
            }

            var (item, minIndex) = _buffer[_minIndex];
            return item;
        }

        private bool IsEmpty()
        {
            return _position == 0;
        }
    }


    [TestFixture]
    public class StackTest
    {
        [Test]
        public void Test()
        {
            // Arrange
            var stack = new StackWithMin();
            stack.Push(1);

            // Act
            var item = stack.Pop();

            // Assert
            Assert.AreEqual(1, item);
        }

        [Test]
        public void TestPopMin()
        {
            // Arrange
            var stack = new StackWithMin();
            stack.Push(5);
            stack.Push(7);
            stack.Push(3);
            stack.Push(4);
            stack.Push(1);

            // Act
            stack.Pop();
            Assert.AreEqual(3, stack.Min());

            stack.Pop();
            Assert.AreEqual(3, stack.Min());

            stack.Pop();
            Assert.AreEqual(5, stack.Min());
        }

        [Test]
        public void TestPushMin()
        {
            // Arrange
            var stack = new StackWithMin();
            stack.Push(5);
            Assert.AreEqual(5, stack.Min());

            stack.Push(7);
            Assert.AreEqual(5, stack.Min());

            stack.Push(3);
            Assert.AreEqual(3, stack.Min());

            stack.Push(4);
            Assert.AreEqual(3, stack.Min());

            stack.Push(1);
            Assert.AreEqual(1, stack.Min());
        }
    }
}