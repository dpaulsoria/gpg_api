#!/bin/bash

# Cargar variables de entorno
source .env

# Crear la base de datos
sqlx database create

# Ejecutar migraciones
sqlx migrate run

# Ejecutar la aplicacion
cargo run
