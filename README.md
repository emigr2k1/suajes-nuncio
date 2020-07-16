# Código fuente de suajesnuncio.com

## Estructura del proyecto.

- Los archivos html que deben ser modificados se encuentran en
  `/util/templates/templates/`.

- Los archivos públicos se encuentran en la carpeta raíz. Esto por simplicidad
  ya que el servidor donde se aloja la página web automáticamente enruta las
  direcciones URL a la carpeta correspondiente. Es decir, la dirección URL
  `http://suajesnuncio.com` corresponde a la carpeta raíz (donde
  automáticamente se ejecuta el archivo index.html), y la dirección URL
  `http://suajesnuncio.com/js/galleria` corresponde a la carpeta `galleria` dentro de la carpeta `js`.

## Instrucciones de lanzamiento

- Para reflejar los cambios en las plantillas html y generar los archivos html
  que serán vistos por el usuario, es necesario ejecutar el script
  `build_templates.sh` que se encuentra en la carpeta raíz. Este script depende
  de que el compilador de `Rust` esté instalado, al igual que el administrador
  de paquetes `Cargo`.

    - Nota: Si se encuentra en un sistema operativo `Windows`, es necesario
      instalar `Cygwin` que trae las herramientas de `Unix` a `Windows`.

- El servidor contratado por el cliente cuenta con el protocólo FTP y éste será
  utilizado para transferir los archivos. Antes de transferir los archivos, los
  siguientes tienen que ser excluidos:

    - `util/`
    - `build_templates.sh`
    - `.git`
    - `.gitignore`

    No excluirlos no tiene ningún efecto que comprometa la seguridad o
    integridad de Suajes Nuncio. Estos archivos solo son excluidos por buenas
    prácticas.

## Más información

- El cliente otorgó permiso para mostrar el código fuente de la página para
  propósitos laborales. El código fuente de la página no contiene ninguna
  información sensible y solo es utilizado para mostrar contenido estático que
  puede ser visto desde cualquier navegador, utilizando funciones como la
  consola, `Inspeccionar elemento` o `Ver código fuente de página`.
