---
description: Analiza el schema de la base de datos y sugiere mejoras de optimización y seguridad
---

# Análisis Completo del Schema de Base de Datos

Por favor, analiza el archivo `database/schema.sql` y proporciona un reporte completo que incluya:

## 1. Análisis de Índices

- Identifica columnas que deberían tener índices pero no los tienen
- Detecta índices redundantes o innecesarios
- Sugiere índices compuestos para queries comunes
- Verifica que las foreign keys tengan índices apropiados

## 2. Análisis de Constraints

- Verifica que todas las foreign keys tengan `ON DELETE` y `ON UPDATE` apropiados
- Identifica columnas que deberían tener `NOT NULL` pero no lo tienen
- Sugiere `CHECK` constraints para validación a nivel de base de datos
- Revisa constraints `UNIQUE` necesarios

## 3. Análisis de Tipos de Datos

- Verifica que los tipos de columnas sean apropiados para los datos que almacenan
- Identifica columnas con tipos demasiado grandes (ej: VARCHAR(500) cuando VARCHAR(100) es suficiente)
- Sugiere tipos más eficientes donde aplique (ej: ENUM vs VARCHAR)

## 4. Normalización

- Identifica posibles problemas de normalización
- Detecta duplicación de datos
- Sugiere refactorizaciones si es necesario

## 5. Performance

- Identifica queries potencialmente lentos basado en el schema
- Sugiere particionamiento si es necesario
- Recomienda índices parciales para queries frecuentes con filtros

## 6. Seguridad

- Verifica que no haya datos sensibles sin encriptar
- Identifica campos que deberían tener validación adicional
- Revisa permisos implícitos en el schema

## 7. Mejores Prácticas

- Verifica convenciones de nomenclatura consistentes
- Identifica tablas sin `created_at`/`updated_at` que deberían tenerlos
- Sugiere triggers o functions útiles

## Formato de Respuesta

Organiza tus hallazgos en:

1. ✅ **Aspectos Correctos**: Qué está bien implementado
2. ⚠️ **Mejoras Recomendadas**: Cambios sugeridos con prioridad (Alta/Media/Baja)
3. 🚨 **Problemas Críticos**: Issues que deben corregirse antes de producción
4. 💡 **Optimizaciones Futuras**: Mejoras para considerar cuando escale el proyecto

Para cada sugerencia, proporciona:
- Descripción del problema
- Razón por la cual es importante
- SQL específico para implementar la solución
- Impacto estimado en performance o seguridad