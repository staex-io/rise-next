<script>
import { ethers } from 'ethers'

export default {
  data() {
    return {
      stations: [],
    }
  },
  methods: {
    async load() {
      try {
        let res = await fetch(`/indexer/stations`, { method: 'GET' })
        switch (res.status) {
          case 200:
            break
          default:
            console.error(res)
            return
        }
        let data = await res.json()
        for (let i = 0; i < data.length; i++) {
          data[i].price = ethers.formatEther(ethers.parseEther(data[i].price))
        }
        this.stations = data
      } catch (e) {
        console.error(e)
        return
      }
    },
  },
  created() {
    this.load()
  },
}
</script>

<template>
  <h1>Stations</h1>
  <div>
    <table v-if="stations.length">
      <thead>
        <tr>
          <th>Address</th>
          <th>Location</th>
          <th>Price (ETH)</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="{ address, location, price } in stations" :key="address">
          <td>{{ address }}</td>
          <td>
            <a :href="`https://www.google.com/maps/place/${location}`" target="_blank">
              {{ location }}
            </a>
          </td>
          <td>{{ price }}</td>
        </tr>
      </tbody>
    </table>
    <p v-else>There are no stations at the moment.</p>
  </div>
</template>
