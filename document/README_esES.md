# rust-godot-startup













## Capítulo 1 Problemas comunes

### Sección 1 Problemas encontrados

Como programador principiante, durante mi primera experiencia con Rust y Godot en conjunto para el desarrollo, me encontré con los siguientes problemas. Dediqué mucho tiempo a resolver una serie de errores extraños causados por la configuración del entorno durante la creación de mi primer proyecto. Siendo principiante, no debería haber invertido tanto tiempo en problemas de entorno.

Creo que algunos desarrolladores, al usar este método por primera vez, pueden crear un repositorio Git de Rust-Godot vacío, descargarlo, compilarlo y ejecutarlo.

Sin embargo, entiendo que este enfoque puede tener algunas limitaciones.

1. No es adecuado para la mayoría de los desarrolladores principiantes.

2. En China, debido a problemas de red, es imposible acceder a repositorios como GitHub con normalidad.

Por lo tanto, dediqué unos 10 días a desarrollar una herramienta para la compilación de proyectos con un solo clic, integrando los componentes de Rust y Godot, desde los comandos hasta todo el proceso de compilación.







### Sección 2: Resolución de problemas

Se ha compilado una herramienta unificada para la creación de proyectos Rust Godot.

Permite crear un proyecto Rust Godot rápido y ejecutable con un solo clic mediante sus propias operaciones de despliegue.

Solo necesitas realizar algunas configuraciones sencillas.

Por ejemplo:

1. Ruta de Cargo

2. Ruta de Godot

3. Espacio de trabajo

4. Nombre del directorio raíz de Rust

5. Nombre de gdext

6. Si deseas crear un proyecto de demostración

Tras completar estas configuraciones, puedes crear un proyecto Rust Godot con un solo clic.

Todo el proceso tarda solo 2 minutos.









### Sección 3 Descripción del software

1. Este software utiliza el sistema operativo Windows. Puede compilar el código para generar archivos ejecutables para otros sistemas operativos.

2. Este software es compatible con 9 idiomas, incluyendo inglés, chino simplificado, chino tradicional, japonés, coreano, alemán, francés, español e italiano.

3. El archivo de configuración solo necesita configurarse una vez; la configuración posterior se almacena en caché localmente.















## Capítulo dos: Introducción a la interfaz

### Sección 01 Interfaz principal


<img width="1326" height="872" alt="image-20260331181015689" src="https://github.com/user-attachments/assets/285f37f1-ebd1-4a7a-b825-6642dec4f364" />


La interfaz principal, en la parte superior, contiene principalmente varios menús de opciones donde puedes realizar selecciones.









### Sección 02 Selección de configuración

<img width="1310" height="862" alt="image-20260331181135850" src="https://github.com/user-attachments/assets/2b2c807f-eb2b-42b8-8114-22274c437dca" />



Puedes elegir la ruta a Rust y la ruta al archivo de Godot.

Dónde:

1. La ruta a Rust se refiere a la carpeta donde se encuentra Cargo.

2. La ruta a Godot se refiere a la carpeta donde se encuentra el archivo ejecutable de Godot.



Por supuesto, también puede configurar el idioma. Al hacer clic en la opción que aparece a continuación, se configurará el idioma del software.

<img width="1309" height="864" alt="image-20260331181355306" src="https://github.com/user-attachments/assets/95f006af-56d7-420d-8812-8ec167019b71" />













### Sección 3 Creación del proyecto

En la interfaz del proyecto, puede comenzar a crear su proyecto.


<img width="1304" height="859" alt="image-20260331181508488" src="https://github.com/user-attachments/assets/e6860c28-5a09-4313-875b-04d5c18f432d" />



Sigue la secuencia de botones para completar la compilación del proyecto. Espera hasta que la barra de progreso alcance el 100% para finalizar la compilación completa del proyecto.





El proceso de compilación se muestra a continuación:

<img width="1308" height="853" alt="image-20260331181707163" src="https://github.com/user-attachments/assets/043a58bb-3dd7-4930-bb1e-a7550e836b97" />


<img width="1301" height="859" alt="image-20260331181754200" src="https://github.com/user-attachments/assets/c8c37427-1b43-4702-92fc-4b13660ae7ab" />






Cuando todas las operaciones hayan finalizado, el sistema mostrará el mensaje "Todas las operaciones han finalizado", lo que indica que el proceso de compilación ha terminado.











### Sección 4 Acerca del software

Si encuentra algún problema al usar el software, puede consultar la documentación correspondiente en la sección "Acerca del software".

<img width="1302" height="861" alt="image-20260331181959363" src="https://github.com/user-attachments/assets/5887aba2-e2e7-41e6-ae7b-44d47f156cec" />



Instrucciones de uso

<img width="1303" height="861" alt="image-20260331182024547" src="https://github.com/user-attachments/assets/6488cf81-b53a-4bff-9000-8fab523b1a3a" />


















## Capítulo 3 Presentación del proyecto

### Sección 01 Documentos del proyecto

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### Sección 02 archivo godot

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />















### Sección 3: Resultados de la ejecución de Godot

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











