using System;
using NUnit.Framework;

namespace InOrderSuccessor.Test
{
    [TestFixture]
    public class Test
    {
        [Test]
        public void TestLeftLeaf()
        {
            // Arrange
            var node8 = new Node {Value = 8};
            var node9 = new Node {Value = 9};
            var node4 = new Node{Value = 4, Left = node8, Right = node9};
            node8.Parent = node4;
            node9.Parent = node4;

            var node10 = new Node {Value = 10};
            var node11= new Node {Value = 11};
            var node5 = new Node{Value = 5, Left = node10, Right = node11};
            node10.Parent = node5;
            node11.Parent = node5;

            var node12 = new Node {Value = 12};
            var node13= new Node {Value = 13};
            var node6 = new Node{Value = 6, Left = node12, Right = node13};
            node12.Parent = node6;
            node13.Parent = node6;
            
            var node14 = new Node {Value = 14};
            var node15= new Node {Value = 15};
            var node7 = new Node{Value = 7, Left = node14, Right = node15};
            node14.Parent = node7;
            node15.Parent = node7;

            var node2 = new Node{Value = 2, Left = node4, Right = node5};
            node4.Parent = node2;
            node5.Parent = node2;

            var node3 = new Node{Value = 3, Left = node6, Right = node7};
            node6.Parent = node3;
            node7.Parent = node3;
            
            var node1 = new Node{Value = 1, Left = node2, Right = node3};
            node2.Parent = node1;
            node3.Parent = node1;

            var bt = new BinaryTree {Root = node1};
            
            // Act
            int? inorderSuccessor = bt.InOrderSuccessor(node8);

            // Assert
            Assert.AreEqual(4, inorderSuccessor.Value);
        }
        
        [Test]
        public void TestRightLeaf()
        {
            // Arrange
            var node8 = new Node {Value = 8};
            var node9 = new Node {Value = 9};
            var node4 = new Node{Value = 4, Left = node8, Right = node9};
            node8.Parent = node4;
            node9.Parent = node4;

            var node10 = new Node {Value = 10};
            var node11= new Node {Value = 11};
            var node5 = new Node{Value = 5, Left = node10, Right = node11};
            node10.Parent = node5;
            node11.Parent = node5;

            var node12 = new Node {Value = 12};
            var node13= new Node {Value = 13};
            var node6 = new Node{Value = 6, Left = node12, Right = node13};
            node12.Parent = node6;
            node13.Parent = node6;
            
            var node14 = new Node {Value = 14};
            var node15= new Node {Value = 15};
            var node7 = new Node{Value = 7, Left = node14, Right = node15};
            node14.Parent = node7;
            node15.Parent = node7;

            var node2 = new Node{Value = 2, Left = node4, Right = node5};
            node4.Parent = node2;
            node5.Parent = node2;

            var node3 = new Node{Value = 3, Left = node6, Right = node7};
            node6.Parent = node3;
            node7.Parent = node3;
            
            var node1 = new Node{Value = 1, Left = node2, Right = node3};
            node2.Parent = node1;
            node3.Parent = node1;

            var bt = new BinaryTree {Root = node1};
            
            // Act
            int? inorderSuccessor = bt.InOrderSuccessor(node9);

            // Assert
            Assert.AreEqual(2, inorderSuccessor.Value);
        }

        [Test]
        public void TestRoot()
        {
            // Arrange
            var node8 = new Node {Value = 8};
            var node9 = new Node {Value = 9};
            var node4 = new Node{Value = 4, Left = node8, Right = node9};
            node8.Parent = node4;
            node9.Parent = node4;

            var node10 = new Node {Value = 10};
            var node11= new Node {Value = 11};
            var node5 = new Node{Value = 5, Left = node10, Right = node11};
            node10.Parent = node5;
            node11.Parent = node5;

            var node12 = new Node {Value = 12};
            var node13= new Node {Value = 13};
            var node6 = new Node{Value = 6, Left = node12, Right = node13};
            node12.Parent = node6;
            node13.Parent = node6;
            
            var node14 = new Node {Value = 14};
            var node15= new Node {Value = 15};
            var node7 = new Node{Value = 7, Left = node14, Right = node15};
            node14.Parent = node7;
            node15.Parent = node7;

            var node2 = new Node{Value = 2, Left = node4, Right = node5};
            node4.Parent = node2;
            node5.Parent = node2;

            var node3 = new Node{Value = 3, Left = node6, Right = node7};
            node6.Parent = node3;
            node7.Parent = node3;
            
            var node1 = new Node{Value = 1, Left = node2, Right = node3};
            node2.Parent = node1;
            node3.Parent = node1;

            var bt = new BinaryTree {Root = node1};
            
            // Act
            int? inorderSuccessor = bt.InOrderSuccessor(node1);

            // Assert
            Assert.AreEqual(12, inorderSuccessor.Value);
        }

        [Test]
        public void TestNode7()
        {
            // Arrange
            var node8 = new Node {Value = 8};
            var node9 = new Node {Value = 9};
            var node4 = new Node{Value = 4, Left = node8, Right = node9};
            node8.Parent = node4;
            node9.Parent = node4;

            var node10 = new Node {Value = 10};
            var node11= new Node {Value = 11};
            var node5 = new Node{Value = 5, Left = node10, Right = node11};
            node10.Parent = node5;
            node11.Parent = node5;

            var node12 = new Node {Value = 12};
            var node13= new Node {Value = 13};
            var node6 = new Node{Value = 6, Left = node12, Right = node13};
            node12.Parent = node6;
            node13.Parent = node6;
            
            var node14 = new Node {Value = 14};
            var node15= new Node {Value = 15};
            var node7 = new Node{Value = 7, Left = node14, Right = node15};
            node14.Parent = node7;
            node15.Parent = node7;

            var node2 = new Node{Value = 2, Left = node4, Right = node5};
            node4.Parent = node2;
            node5.Parent = node2;

            var node3 = new Node{Value = 3, Left = node6, Right = node7};
            node6.Parent = node3;
            node7.Parent = node3;
            
            var node1 = new Node{Value = 1, Left = node2, Right = node3};
            node2.Parent = node1;
            node3.Parent = node1;

            var bt = new BinaryTree {Root = node1};
            
            // Act
            int? inorderSuccessor = bt.InOrderSuccessor(node7);

            // Assert
            Assert.AreEqual(15, inorderSuccessor.Value);
        }

        [Test]
        public void Test1()
        {
            // Arrange
            var node8 = new Node {Value = 8};
            var node9 = new Node {Value = 9};
            var node4 = new Node{Value = 4, Left = node8, Right = node9};
            node8.Parent = node4;
            node9.Parent = node4;

            var node10 = new Node {Value = 10};
            var node11= new Node {Value = 11};
            var node5 = new Node{Value = 5, Left = node10, Right = node11};
            node10.Parent = node5;
            node11.Parent = node5;

            var node12 = new Node {Value = 12};
            var node13= new Node {Value = 13};
            var node6 = new Node{Value = 6, Left = node12, Right = node13};
            node12.Parent = node6;
            node13.Parent = node6;
            
            var node2 = new Node{Value = 2, Left = node4, Right = node5};
            node4.Parent = node2;
            node5.Parent = node2;

            var node3 = new Node{Value = 3, Left = node6};
            node6.Parent = node3;
            
            var node1 = new Node{Value = 1, Left = node2, Right = node3};
            node2.Parent = node1;
            node3.Parent = node1;

            var bt = new BinaryTree {Root = node1};
            
            // Act
            int? inorderSuccessor = bt.InOrderSuccessor(node3);

            // Assert
            Assert.IsNull(inorderSuccessor);
        }

        
    }
}