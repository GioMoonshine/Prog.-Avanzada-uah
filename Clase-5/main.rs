pub struct Nodo {
    id: i32,
    izq: Option<Box<Nodo>>,
    der: Option<Box<Nodo>>,
}

pub struct ArbolBinario {
    raiz: Option<Box<Nodo>>,
}

impl ArbolBinario {
    pub fn new() -> Self {
        ArbolBinario {raiz:None}
    }
    pub fn insertar(&mut self, n: i32) {
        self.raiz = Self::insertar_rec(self.raiz.take(), n);
    }
    pub fn insertar_rec(nodo: Option<Box<Nodo>>, n:i32) -> Option<Box<Nodo>> {
        match nodo {
            None => {
                Some(Box::new(Nodo {
                    id: n,
                    izq: None,
                    der: None,
                }))
            }
            Some(mut actual) => {
                if n < actual.id {
                    actual.izq = Self::insertar_rec(actual.izq.take(), n);
                } else {
                    actual.der = Self::insertar_rec(actual.der.take(), n);
                }
                Some(actual)
            }
        }
    }

    fn inorden(&self) {
        Self::inorden_rec(&self.raiz);
        println!();
    }
    fn inorden_rec(nodo: &Option<Box<Nodo>>) {
        if let Some(actual) = nodo {
            Self::inorden_rec(&actual.izq);
            print!("{}", actual.id);
            Self::inorden_rec(&actual.der);
        }
    }
}

fn main() {
    let mut arbol = ArbolBinario::new();
    arbol.insertar(3);
    arbol.insertar(2);
    arbol.insertar(7);
    arbol.insertar(4);
    arbol.insertar(9);

    arbol.inorden();
}