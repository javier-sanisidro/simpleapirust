## Generar imagen docker del proyecto
```
docker build --pull --rm -f "Dockerfile" -t simpleapirust:latest "."
```
## Correr imagen docker del proyecto

```
docker run -d -p 80:80 simpleapirust:latest   
```

http://localhost/

## Es necesario que cuando ejecutemos el comando docker run -e .... nuestro programa de rust tiene que utilizar esos parametros respectivamente.
```
docker run -d -p 80:80 -e RUST_PORT='80' -e HOST='0.0.0.0' simpleapirust:latest   
```
## Anhadir anhdir varios endpoints uno que guarde un parametro que le enviamos en bbdd (Mysql, sqlite, mongo, al gusto) y otro endpoint que los liste.

## Preparar docker-compose.yml para levantar tanto la api como la bbdd y que trabajen juntos.