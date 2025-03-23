// Definição da estrutura de Pilha (Stack)
pub struct Stack<T> {
    // A implementação será adicionada após criar os testes
    elementos: Vec<T>,
    capacidade_maxima: Option<usize>,
}

// A implementação será adicionada após criar os testes
impl<T> Stack<T> {
    // A implementação será adicionada após criar os testes
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nova_pilha_esta_vazia() {
        let pilha: Stack<i32> = Stack::nova();
        assert!(pilha.esta_vazia());
    }

}



// Implementação dos métodos para a estrutura Stack
impl<T> Stack<T> {
    // Método para criar uma nova pilha sem limite de capacidade
    pub fn nova() -> Self {
        Stack {
            Self.elementos:None,
            Self.capacidade_maxima: None,
        }
    }

}