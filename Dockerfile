# Stage 1
FROM rust as builder
#Con workdir estamos definiendo donde vamos a trabajar dentro del contenedor
WORKDIR /app
# Copiamos el contenido de nuestro proyecto dentro del contenedor para generar el binario
ADD . .
# Generamos el binario
RUN cargo build --release
# Stage 2
FROM debian:buster-slim
WORKDIR /app
# Copiamos el binario 
COPY --from=builder /app/target/release/rest-api ./rest-api
# Ejecutamos el binario 
CMD ["./rest-api"]
