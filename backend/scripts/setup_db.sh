#!/bin/bash

# Script para configurar la base de datos PostgreSQL de LumaStack

set -e  # Exit on error

echo "🗄️  Configurando base de datos LumaStack..."

# Configuración por defecto
DB_USER="${DB_USER:-lumastack}"
DB_PASSWORD="${DB_PASSWORD:-password}"
DB_NAME="${DB_NAME:-lumastack}"
DB_HOST="${DB_HOST:-localhost}"
DB_PORT="${DB_PORT:-5432}"

# Colores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Usando configuración:${NC}"
echo "  DB_USER: $DB_USER"
echo "  DB_NAME: $DB_NAME"
echo "  DB_HOST: $DB_HOST"
echo "  DB_PORT: $DB_PORT"
echo ""

# Verificar si PostgreSQL está corriendo
if ! pg_isready -h "$DB_HOST" -p "$DB_PORT" > /dev/null 2>&1; then
    echo "❌ PostgreSQL no está corriendo en $DB_HOST:$DB_PORT"
    echo ""
    echo "Por favor inicia PostgreSQL primero:"
    echo "  - macOS: brew services start postgresql@14"
    echo "  - Linux: sudo systemctl start postgresql"
    echo "  - Docker: docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=postgres postgres:14"
    exit 1
fi

echo -e "${GREEN}✓${NC} PostgreSQL está corriendo"

# Crear usuario si no existe (usando el usuario postgres por defecto)
echo "Creando usuario '$DB_USER'..."
psql -h "$DB_HOST" -p "$DB_PORT" -U postgres -tc \
    "SELECT 1 FROM pg_user WHERE usename = '$DB_USER'" | \
    grep -q 1 || \
    psql -h "$DB_HOST" -p "$DB_PORT" -U postgres -c \
    "CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';"

echo -e "${GREEN}✓${NC} Usuario '$DB_USER' verificado"

# Crear base de datos si no existe
echo "Creando base de datos '$DB_NAME'..."
psql -h "$DB_HOST" -p "$DB_PORT" -U postgres -tc \
    "SELECT 1 FROM pg_database WHERE datname = '$DB_NAME'" | \
    grep -q 1 || \
    psql -h "$DB_HOST" -p "$DB_PORT" -U postgres -c \
    "CREATE DATABASE $DB_NAME OWNER $DB_USER;"

echo -e "${GREEN}✓${NC} Base de datos '$DB_NAME' verificada"

# Grant privileges
psql -h "$DB_HOST" -p "$DB_PORT" -U postgres -c \
    "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"

echo -e "${GREEN}✓${NC} Privilegios otorgados"

# Actualizar .env con la URL de la base de datos
DATABASE_URL="postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"

if [ -f ".env" ]; then
    # Actualizar DATABASE_URL en .env
    if grep -q "DATABASE_URL=" .env; then
        sed -i.bak "s|DATABASE_URL=.*|DATABASE_URL=$DATABASE_URL|" .env
        rm .env.bak
        echo -e "${GREEN}✓${NC} .env actualizado con DATABASE_URL"
    else
        echo "DATABASE_URL=$DATABASE_URL" >> .env
        echo -e "${GREEN}✓${NC} DATABASE_URL agregado a .env"
    fi
else
    echo "⚠️  Archivo .env no encontrado, créalo copiando .env.example"
fi

echo ""
echo -e "${GREEN}✅ Base de datos configurada exitosamente!${NC}"
echo ""
echo "Para aplicar las migraciones, ejecuta:"
echo "  cd backend && cargo run"
echo ""
echo "O usa sqlx-cli:"
echo "  cargo install sqlx-cli --no-default-features --features postgres"
echo "  sqlx migrate run"
