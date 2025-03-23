// src/lib.rs

// Módulo de testes: esse código será compilado e executado apenas quando rodarmos os testes.
#[cfg(test)]
mod tests {
    // Traz para o módulo as definições do escopo superior (como a estrutura Queue) para os testes.
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        // "let mut queue = Queue::new();"
        //
        // - "let" declara uma nova variável.
        // - "mut" indica que essa variável é mutável, ou seja, pode ter seu valor alterado.
        // - "Queue::new()" chama o método associado "new" para criar uma nova instância da nossa fila.
        let mut queue = queue::new();

        // Adiciona (enqueue) elementos na fila.
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        // Como uma fila funciona em modo FIFO (First In, First Out),
        // o primeiro elemento inserido deve ser o primeiro a sair.
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        // Quando a fila estiver vazia, dequeue deve retornar None.
        assert_eq!(queue.dequeue(), None);
    }
}
