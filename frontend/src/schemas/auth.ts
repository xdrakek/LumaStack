import { z } from 'zod'

/**
 * Schema de validación para login
 */
export const loginSchema = z.object({
  username: z
    .string()
    .min(1, 'El usuario es requerido')
    .min(3, 'El usuario debe tener al menos 3 caracteres')
    .max(50, 'El usuario no puede exceder 50 caracteres')
    .trim()
    .default(''),
  password: z
    .string()
    .min(1, 'La contraseña es requerida')
    .min(8, 'La contraseña debe tener al menos 8 caracteres')
    .max(100, 'La contraseña no puede exceder 100 caracteres')
    .default('')
})

export type LoginInput = z.infer<typeof loginSchema>

/**
 * Schema de validación para registro
 */
export const registerSchema = z
  .object({
    username: z
      .string()
      .min(1, 'El usuario es requerido')
      .min(3, 'El usuario debe tener al menos 3 caracteres')
      .max(50, 'El usuario no puede exceder 50 caracteres')
      .regex(/^[a-zA-Z0-9_-]+$/, 'El usuario solo puede contener letras, números, guiones y guiones bajos')
      .trim(),
    email: z
      .string()
      .min(1, 'El email es requerido')
      .email('Email inválido')
      .max(255, 'El email no puede exceder 255 caracteres')
      .trim()
      .toLowerCase(),
    password: z
      .string()
      .min(1, 'La contraseña es requerida')
      .min(8, 'La contraseña debe tener al menos 8 caracteres')
      .max(100, 'La contraseña no puede exceder 100 caracteres')
      .regex(/[A-Z]/, 'La contraseña debe contener al menos una mayúscula')
      .regex(/[a-z]/, 'La contraseña debe contener al menos una minúscula')
      .regex(/[0-9]/, 'La contraseña debe contener al menos un número'),
    confirmPassword: z
      .string()
      .min(1, 'Debes confirmar la contraseña')
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: 'Las contraseñas no coinciden',
    path: ['confirmPassword']
  })

export type RegisterInput = z.infer<typeof registerSchema>
