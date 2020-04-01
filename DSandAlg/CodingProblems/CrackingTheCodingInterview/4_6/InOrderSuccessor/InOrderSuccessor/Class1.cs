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
            if (node.Right != null)
            {
                return DescendInOrder(node.Right).Value;
            }

            if (node.Parent.Left == node)
            {
                return node.Parent.Value;
            }

            return node.Parent.Parent?.Value;

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