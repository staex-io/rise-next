<script>
export default {
  data() {
    return {
      id: 0,
      landing: null,
    }
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
  mounted() {
    this.id = this.$route.params.id
    this.load()
  },
}
</script>

<template>
  <div class="card" v-if="landing">
    <div class="card-header">Landing</div>
    <div class="card-content">
      <div class="card-field">
        <span class="card-field-label">ID</span>
        <span class="card-field-value">{{ landing.id }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Drone</span>
        <span class="card-field-value">{{ landing.drone }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Station</span>
        <span class="card-field-value">{{ landing.station }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Landlord</span>
        <span class="card-field-value">{{ landing.landlord }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Taken off</span>
        <span class="card-field-value">{{ landing.is_taken_off }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Rejected</span>
        <span class="card-field-value">{{ landing.is_rejected }}</span>
      </div>
      <hr />
      <div class="card-field">
        <span class="card-field-label">Agreement Station & Drone</span>
        <span class="card-field-value">
          <pre>{{ JSON.stringify(landing.agreements[0], null, 4) }}</pre>
        </span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Agreement Station & Landlord</span>
        <span class="card-field-value">
          <pre>{{ JSON.stringify(landing.agreements[1], null, 4) }}</pre>
        </span>
      </div>
    </div>
  </div>
</template>
