using System;
using System.Collections.Generic;
using NUnit.Framework;

namespace DisjointSet
{
    public class Node
    {
        public int NodeId { get; set; }
        public int Height { get; set; }
        public Node ParentNode { get; set; }
    }

    public class DisjointSet
    {
        private readonly List<Node> _nodes;

        private DisjointSet(in int size)
        {
            _nodes = new List<Node>(size);
        }

        public static DisjointSet MakeSet(int size)
        {
            var disjointSet = new DisjointSet(size);
            for (int i = 0; i < size; i++)
            {
                var node = new Node
                {
                    NodeId = i
                    , Height = 0
                    , ParentNode = null 
                };
                disjointSet._nodes.Add(node);
            }
            return disjointSet;
        }

        public int Find(int idx)
        {
            // SS: Note that we are not updating the height/rank after path compression,
            // because there is not efficient way to do so.
            
            // SS: return the root node id
            Node node = _nodes[idx];
            
            // SS: find the root node (it is the one with ParentNode == null)
            var currentNode = node;
            while (currentNode.ParentNode != null)
            {
                currentNode = currentNode.ParentNode;
            }
            
            // SS: currentNode is root node
            var rootNode = currentNode;
            currentNode = node;

            // SS: path compression
            while (currentNode != rootNode)
            {
                var n = currentNode.ParentNode;
                currentNode.ParentNode = rootNode;
                currentNode = n;
            }

            return rootNode.NodeId;
        }

        public void Merge(int idx1, int idx2)
        {
            var node1RootId = Find(idx1);
            var node2RootId = Find(idx2);

            if (node1RootId == node2RootId)
            {
                // SS: both nodes are in the same set already
                return;
            }

            var root1 = _nodes[node1RootId];
            var root2 = _nodes[node2RootId];

            // SS: attach smaller subtree (lower rank i.e. height) to larger one
            if (root1.Height < root2.Height)
            {
                root1.ParentNode = root2;
            }
            else if (root1.Height > root2.Height)
            {
                root2.ParentNode = root1;
            }
            else
            {
                // SS: both subtrees are the same rank, so increase rank of the
                // node we are adding the other to
                root2.ParentNode = root1;
                root1.Height += 1;
            }
        }
        
    }


    [TestFixture]
    public class DisjointSetTest
    {
        [Test]
        public void TestFind()
        {
            // Arrange
            var disjointSet = DisjointSet.MakeSet(7);
            disjointSet.Merge(2, 0);
            disjointSet.Merge(0, 1);
            disjointSet.Merge(2, 3);
            
            // Act
            var idx = disjointSet.Find(0);

            // Assert
            Assert.AreEqual(2, idx);
        }
        
    }
    
}

