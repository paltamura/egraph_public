# egraph_public
Egraph: Sample simple version

```rustup default nightly```

## Ejemplo simple de operaciones básicas con modelos y con ordenes:

Nota: Está armado solo para entender la interacción. El ejemplo no tiene sentido desde el punto de vista conceptual.

### Comandos servidor

#### Instanciar server:
`$ ./elayer ws --run`

### Comandos cliente cli

#### Construir y persistir modelo basado en el modelo de ejemplo definido en código como model1:
`$ ./elayer createmodel --modeltype model1 --modelname ordensimple`

#### Obtener los nombres de los functors de todas las capas del modelo:
`$ ./elayer getfunctors --modelname ordensimple`

#### Obtener todos los morfismos de un functor:
`$ ./elayer getallmorphisms --modelname ordensimple --functorname input_layer_inst_functor`

#### Editar un morfismo específico:
`$ ./elayer editmorphism --modelname ordensimple --functorname input_layer_inst_functor --morphismid 0`

#### Crear orden basada en un modelo entrenado ordensimple:
`$ ./elayer createorder --modelname ordensimple`

#### Ejecutar orden por id:
`$ ./elayer executeorder --orderid OID-0`

#### Obtener log de ejecución de una orden por id:
`$ ./elayer getorderlog --orderid OID-0`
