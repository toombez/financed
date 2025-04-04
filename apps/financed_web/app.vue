<script setup lang="ts">
import init, { a } from 'financed_core'
onBeforeMount(async () => {
    await init()

    console.log(a())
})
</script>

<template>
  <div>
    <NuxtRouteAnnouncer />
    <NuxtWelcome />
  </div>
</template>
