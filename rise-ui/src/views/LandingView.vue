<script>
export default {
  data() {
    return {
      id: 0,
      landing: null,
    }
  },
  mounted() {
    this.id = this.$route.params.id
    this.load()
  },
  methods: {
    async load() {
      try {
        let res = await fetch(`/indexer/landings?id=${this.id}`, { method: 'GET' })
        switch (res.status) {
          case 200:
            break
          default:
            console.error(res)
            return
        }
        let data = await res.json()
        this.landing = data[0]
      } catch (e) {
        console.error(e)
        return
      }
    },
  },
}
</script>

<template>
  <div v-if="landing" class="card">
    <div class="card-header">Landing</div>
    <div class="card-content">
      <div class="card-field">
        <span class="card-field-label">ID</span>
        <span class="card-field-value">{{ landing.id }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Drone</span>
        <span class="card-field-value">
          <div class="h-scroll-container">{{ landing.drone }}</div>
        </span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Station</span>
        <span class="card-field-value">
          <div class="h-scroll-container">{{ landing.station }}</div>
        </span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Landlord</span>
        <span class="card-field-value">
          <div class="h-scroll-container">{{ landing.landlord }}</div>
        </span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Taken off</span>
        <span class="card-field-value">{{ landing.is_taken_off }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Rejected</span>
        <span class="card-field-value">{{ landing.is_rejected }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Date</span>
        <span class="card-field-value">{{ new Date(landing.date * 1000) }}</span>
      </div>
      <hr v-if="landing.agreements" />
      <div v-if="landing.agreements && landing.agreements.length >= 1" class="card-field">
        <span class="card-field-label">Agreement Station & Drone</span>
        <span class="card-field-value">
          <pre>{{ JSON.stringify(landing.agreements[0], null, 4) }}</pre>
        </span>
      </div>
      <div v-if="landing.agreements && landing.agreements.length >= 2" class="card-field">
        <span class="card-field-label">Agreement Station & Landlord</span>
        <span class="card-field-value">
          <pre>{{ JSON.stringify(landing.agreements[1], null, 4) }}</pre>
        </span>
      </div>
    </div>
  </div>
</template>
