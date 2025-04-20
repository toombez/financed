<script setup lang="ts">
import init, { WasmNewtypeValidationError, ExpirationDate  } from 'financed_core'
onBeforeMount(async () => {
    await init()

    const e = new ExpirationDate("2025-04-20 20:00")

    try {
        const e1 = ExpirationDate.safe_new("2022");
    } catch (error) {
        const _error = error as WasmNewtypeValidationError
        _error.errors.map((v) => {
            console.error(v.message)
        })
    }

    console.log(e.value)
})
</script>

<template>
  <div>
    <NuxtRouteAnnouncer />
    <NuxtWelcome />
  </div>
</template>
