struct Brigada {
    id: i32,
    siguiente: Option<Box<Brigada>>,
}

struct ListaEnlazada {
    cabeza: Option<Box<Brigada>>,
}

impl ListaEnlazada {
    fn new() -> Self {
        ListaEnlazada {cabeza:None}
    }
    fn insertar_inicio(&mut self, id:i32){
        let nuevo = Box::new(Brigada {
            id,
            siguiente: self.cabeza.take(),
        });
        self.cabeza = Some(nuevo);
    }
    fn insertar_final(&mut self, id:i32){
        let nuevo = Box::new(Brigada {
            id,
            siguiente: None,
        });
        //Si lista vacía
        if self.cabeza.is_none() {
            self.cabeza = Some(nuevo);
            return;
        }
        //Si lista no vacía
        let mut actual = self.cabeza.as_mut().unwrap();
        while actual.siguiente.is_some() {
            actual = actual.siguiente.as_mut().unwrap();
        }
        actual.siguiente = Some(nuevo)
    }
    fn eliminar(&mut self, id:i32) {
        //Caso especial: eliminar cabeza
        if let Some(ref mut cabeza) = self.cabeza {
            if cabeza.id == id {
                self.cabeza = cabeza.siguiente.take();
                return;
            }
        }
        let mut actual = self.cabeza.as_mut();
        while let Some(nodo) = actual {
            if let Some(ref mut siguiente) = nodo.siguiente {
                if siguiente.id == id {
                    nodo.siguiente = siguiente.siguiente.take();
                    return;
                }
            }
            actual = nodo.siguiente.as_mut();
        }
    }
    fn mostrar(&self) {
        let mut actual = self.cabeza.as_ref();
        println!("Estado actual de las brigadas: ");
        while let Some(nodo) = actual {
            println!("Brigada {}", nodo.id); 
            actual = nodo.siguiente.as_ref();
        }
    }
}

fn main() {
    let mut sistema = ListaEnlazada::new();
    sistema.insertar_inicio(4);
    sistema.insertar_inicio(8);
    sistema.insertar_inicio(15);
    sistema.insertar_final(12);
    sistema.eliminar(8);
    sistema.mostrar()
}