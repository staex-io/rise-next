<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>
<script>
import router from '@/router'
export default {
  beforeMount() {
    router.push({ path: window.location.pathname })
  },
}
</script>

<template>
  <header>
    <nav>
      <img class="logo" alt="Staex logo" src="@/assets/logo-light.svg" />
      <ul>
        <li>
          <RouterLink to="/create-agreement">Create Agreement</RouterLink>
        </li>
        <li>
          <RouterLink to="/sign-agreement">Sign Agreement</RouterLink>
        </li>
      </ul>
    </nav>
  </header>
  <RouterView />
</template>
