#region

using System;
using NUnit.Framework;

#endregion

namespace Problem_3_6
{
    public class Node
    {
        public string Name { get; set; }
        public int Age { get; set; }
        public Node Next { get; set; }
    }

    public class Queue
    {
        public Node Head { get; set; }
        public Node Tail { get; set; }

        public void InsertBack(string value, int age)
        {
            var node = new Node {Name = value, Age = age};

            if (Head == null)
            {
                Head = node;
                Tail = Head;
            }
            else
            {
                Tail.Next = node;
                Tail = node;
            }
        }

        public (string name, int age) Peek()
        {
            if (Head == null)
            {
                throw new InvalidOperationException();
            }

            return (Head.Name, Head.Age);
        }

        public string RemoveFront()
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

            return node.Name;
        }

        public bool IsEmpty()
        {
            return Head == null;
        }
    }

    public class AnimalShelter
    {
        private int _age;
        private readonly Queue _cats = new Queue();
        private readonly Queue _dogs = new Queue();

        public void EnqueueCat(string name)
        {
            _cats.InsertBack(name, _age++);
        }

        public string DequeueCat()
        {
            return _cats.RemoveFront();
        }

        public void EnqueueDog(string name)
        {
            _dogs.InsertBack(name, _age++);
        }

        public string DequeueDog()
        {
            return _dogs.RemoveFront();
        }

        public string Dequeue()
        {
            if (_cats.IsEmpty())
            {
                if (_dogs.IsEmpty())
                {
                    throw new InvalidOperationException();
                }

                return DequeueDog();
            }

            if (_dogs.IsEmpty())
            {
                if (_cats.IsEmpty())
                {
                    throw new InvalidOperationException();
                }

                return DequeueCat();
            }

            var catAge = _cats.Peek().age;
            var dogAge = _dogs.Peek().age;
            return catAge < dogAge ? DequeueCat() : DequeueDog();
        }
    }

    [TestFixture]
    public class Tests
    {
        [Test]
        public void Test()
        {
            // Arrange
            var animalShelter = new AnimalShelter();
            animalShelter.EnqueueCat("Puma");
            animalShelter.EnqueueDog("Quita");
            animalShelter.EnqueueCat("Millie");

            // Act/Assert
            var name = animalShelter.Dequeue();
            Assert.AreEqual("Puma", name);

            name = animalShelter.Dequeue();
            Assert.AreEqual("Quita", name);

            name = animalShelter.Dequeue();
            Assert.AreEqual("Millie", name);
        }
    }
}