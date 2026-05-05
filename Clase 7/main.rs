use std::collections::VecDeque;

struct NodoLista {
    valor: usize,
    siguiente: Option<Box<NodoLista>>,
}

struct Lista {
    cabeza: Option<Box<NodoLista>>,
}

impl Lista {
    fn new() -> Self {
        Lista { cabeza: None }
    }
    fn insertar(&mut self, valor:usize) {
        let nuevo = Box::new(NodoLista{
            valor,
            siguiente: self.cabeza.take(),
        });
        self.cabeza = Some(nuevo);
    }
    fn iterar(&self) -> Vec<usize> {
        let mut resultado = Vec::new();
        let mut actual = &self.cabeza;
        //Recorrer la lista
        while let Some(nodo) = actual {
            resultado.push(nodo.valor);
            actual = &nodo.siguiente;
        }
        resultado
    }
}

struct Zona {
    id: usize,
    nombre: String,
}

struct Grafo {
    zonas: Vec<Zona>,
    adyacencia: Vec<Lista>,
}

impl Grafo {
    fn new() -> Self {
        Grafo {
            zonas: Vec::new(),
            adyacencia: Vec::new(),
        }
    }
    fn agregar_zona(&mut self, nombre:&str) {
        let id = self.zonas.len();
        self.zonas.push(Zona{
            id,
            nombre: nombre.to_string(),
        });
        self.adyacencia.push(Lista::new());
    }
    fn conectar(&mut self, a: usize, b: usize) {
        self.adyacencia[a].insertar(b);
        self.adyacencia[b].insertar(a);
    }
    fn mostrar_matriz(&self) {
        let n = self.zonas.len();
        let mut matriz = vec![vec![0;n];n];

        for i in 0..n{
            for vecino in self.adyacencia[i].iterar() {
                matriz[i][vecino] = 1;
            }
        }
        println!("\n Matriz de adyacencia ");

        for fila in matriz {
            for val in fila {
                print!("{}", val);
            }
            println!();
        }
    }
    fn bfs(&self, inicio: usize, objetivo: usize) {
        let n = self.zonas.len();
        let mut visitado = vec![false;n];
        let mut padre: Vec<Option<usize>> = vec![None;n];
        let mut cola = VecDeque::new();
        visitado[inicio] = true;
        cola.push_back(inicio);
        println!("\n BFS ");

        while let Some(actual) = cola.pop_front() {
            println!("Visitando {}", self.zonas[actual].nombre);

            if actual == objetivo {
                break;
            }

            for vecino in self.adyacencia[actual].iterar() {
                if !visitado[vecino] {
                    visitado[vecino] = true;
                    padre[vecino] = Some(actual);
                    cola.push_back(vecino);
                }
            }
        }
    }
}

fn main () {
    let mut g = Grafo::new();

    for i in 0..12 {
        g.agregar_zona(&format!("Zona {}", i));
    }
    g.conectar(0, 1);
    g.conectar(0, 2);
    g.conectar(0, 3);
    g.conectar(1, 4);
    g.conectar(1, 5);
    g.conectar(2, 5);
    g.conectar(2, 6);
    g.conectar(3, 6);
    g.conectar(3, 7);
    g.conectar(4, 8);
    g.conectar(5, 8);
    g.conectar(5, 9);
    g.conectar(6, 9);
    g.conectar(6, 10);
    g.conectar(7, 10);
    g.conectar(8, 11);
    g.conectar(9, 11);
    g.conectar(10, 11);

    g.mostrar_matriz();
    g.bfs(0, 11);
}