namespace InOrderSuccessor
{
    public class Node
    {
        public int Value { get; set; }
        public Node Left { get; set; }
        public Node Right { get; set; }
        public Node Parent { get; set; }

        public bool IsLeaf()
        {
            return Left == null && Right == null;
        }
    }

    public class BinaryTree
    {
        public Node Root { get; set; }

        public int? InOrderSuccessor(Node node)
        {
            if (node == Root)
            {
                // SS: descend down the right subtree
                var n = DescendInOrder(Root.Right);
                return n?.Value;
            }
            
            if (node.IsLeaf())
            {
                if (node.Parent.Left == node)
                {
                    return node.Parent.Value;
                }

                // SS: node is right child of parent
                while (node.Parent != null && node.Parent.Right == node)
                {
                    node = node.Parent;
                }
                
                // SS: either root or in left subtree
                if (node.Parent == null)
                {
                    // SS: node is root, no sucessor
                    return null;
                }
                
                return node.Parent.Value;
            }
            else
            {
                // SS: node is not a leaf node
                if (node.Right != null)
                {
                    var n = DescendInOrder(node.Right);
                    return n?.Value;
                }
                else
                {
                    if (node.Parent.Left == node)
                    {
                        return node.Parent.Value;
                    }
                    
                    // SS: node is right child and node does not have a right subtree, so
                    // return null
                    return null;
                }
            }
        }

        private Node DescendInOrder(Node node)
        {
            if (node.Left != null)
            {
                return DescendInOrder(node.Left);
            }
            return node;
        }
    }
}