
/// Um nó da fila.
/// Cada nó contém um elemento e uma referência para o próximo nó.
pub struct Node<T> {
    // O elemento armazenado no nó.
    elem: T,
    // Ponteiro para o próximo nó.
    // Utilizamos Option<Box<Node<T>>> para indicar que pode haver (Some) ou não (None) um próximo nó.
    next: Option<Box<Node<T>>>,
}

/// A fila (queue) propriamente dita.
/// Mantém um ponteiro para o primeiro nó (head) e para o último nó (tail).
pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    // tail será um ponteiro cru (raw pointer) para o último nó da fila.
    // Usamos *mut Node<T> para indicar que é um ponteiro mutável.
    tail: Option<*mut Node<T>>,
}