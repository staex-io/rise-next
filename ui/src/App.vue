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
      <a href="/wallet">
        <img class="logo" alt="Staex logo" src="@/assets/logo-light.svg" />
      </a>
      <ul>
        <li class="menu-item">
          <RouterLink to="/">Agreements</RouterLink>
          <ul class="submenu">
            <li>
              <RouterLink to="/create-agreement">Create Agreement</RouterLink>
            </li>
            <li>
              <RouterLink to="/sign-agreement">Sign Agreement</RouterLink>
            </li>
            <li>
              <RouterLink to="/agreements">Agreements</RouterLink>
            </li>
            <li>
              <RouterLink to="/get-agreement">Get Agreement</RouterLink>
            </li>
          </ul>
        </li>
        <li>
          <RouterLink to="/wallet">Wallet</RouterLink>
        </li>
        <li>
          <RouterLink to="/did">DID</RouterLink>
        </li>
        <li>
          <RouterLink to="/qrcode">QRCode</RouterLink>
        </li>
      </ul>
    </nav>
  </header>
  <RouterView />
</template>
