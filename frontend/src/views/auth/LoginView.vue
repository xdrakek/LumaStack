<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/zod'
// import { useAuthStore } from '@/stores/auth'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { loginSchema, type LoginInput } from '@/schemas/auth'

const router = useRouter()
// const authStore = useAuthStore()

const loading = ref(false)
const error = ref('')

const { errors, handleSubmit, defineField } = useForm<LoginInput>({
  validationSchema: toTypedSchema(loginSchema)
})

const [username, usernameAttrs] = defineField('username')
const [password, passwordAttrs] = defineField('password')

const onSubmit = handleSubmit(async (values) => {
  loading.value = true
  error.value = ''

  try {
    // TODO: Implementar llamada al API
    console.log('Login:', values)

    // Mock de login exitoso
    // authStore.setAuth('mock-token', {
    //   id: 1,
    //   username: values.username,
    //   email: `${values.username}@example.com`,
    //   role: 'user'
    // })

    router.push('/dashboard')
  } catch (err) {
    error.value = 'Error al iniciar sesión'
    console.error(err)
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="min-h-screen h-screen flex items-center justify-center bg-cover bg-center bg-no-repeat relative" style="background-image: url('/src/assets/bg-2.webp')">
    <!-- Overlay oscuro -->
    <div class="absolute inset-0 bg-black/40"></div>

    <Card class="w-full max-w-md relative z-10">
      <CardHeader>
        <CardTitle class="text-2xl">Iniciar Sesión</CardTitle>
        <CardDescription>
          Ingresa tus credenciales para acceder a <strong class="text-primary">LumaStack</strong>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form @submit.prevent="onSubmit" class="space-y-4">
          <div class="space-y-2">
            <Label for="username">Usuario</Label>
            <Input
              id="username"
              v-model="username"
              v-bind="usernameAttrs"
              type="text"
              placeholder="Ingresa tu usuario"
              :class="{ 'border-destructive': errors.username }"
            />
            <p v-if="errors.username" class="text-sm text-destructive">
              {{ errors.username }}
            </p>
          </div>

          <div class="space-y-2">
            <Label for="password">Contraseña</Label>
            <Input
              id="password"
              v-model="password"
              v-bind="passwordAttrs"
              type="password"
              placeholder="Ingresa tu contraseña"
              :class="{ 'border-destructive': errors.password }"
            />
            <p v-if="errors.password" class="text-sm text-destructive">
              {{ errors.password }}
            </p>
          </div>

          <Alert v-if="error" variant="destructive">
            <AlertDescription>
              {{ error }}
            </AlertDescription>
          </Alert>

          <Button
            type="submit"
            :disabled="loading"
            class="w-full"
          >
            {{ loading ? 'Cargando...' : 'Ingresar' }}
          </Button>
        </form>
      </CardContent>
    </Card>
  </div>
</template>
