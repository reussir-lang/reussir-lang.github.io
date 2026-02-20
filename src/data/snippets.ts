export const REVERSE_SNIPPET = `enum List<T> {
    Nil,
    Cons(T, List<T>)
}

fn reverse_impl(list : List<i32>, acc : List<i32>) -> List<i32> {
    match list {
        List::Nil => acc,
        List::Cons(x, xs) => reverse_impl(xs, List::Cons{x, acc})
    }
}`;

export const TREE_MIRROR_SNIPPET = `enum Tree {
    Leaf(i32),
    Node(Tree, Tree)
}

fn mirror(t : Tree) -> Tree {
    match t {
        Tree::Leaf(x) => Tree::Leaf{x},
        Tree::Node(l, r) => Tree::Node{mirror(r), mirror(l)}
    }
}`;

export const FIBONACCI_SNIPPET = `struct [value] Matrix<T : Num> {
    m00: T, m01: T, m10: T, m11: T
}

fn matmul<T : Num>(a : Matrix<T>, b : Matrix<T>) -> Matrix<T> { ... }

fn fibonacci_logarithmic_impl<T : Integral>(n: T, a: Matrix<T>, b: Matrix<T>) -> T {
    if n == 0 { a.m01 } else { ... }
}`;

export interface Snippet {
  label: string;
  code: string;
}

export const snippets: Snippet[] = [
  { label: "list_reverse.rr", code: REVERSE_SNIPPET },
  { label: "tree_mirror.rr", code: TREE_MIRROR_SNIPPET },
  { label: "fibonacci-generic.rr", code: FIBONACCI_SNIPPET },
];
